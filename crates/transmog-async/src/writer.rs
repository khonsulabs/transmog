use std::{
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

use futures_core::ready;
use futures_sink::Sink;
use ordered_varint::Variable;
use tokio::io::AsyncWrite;
use transmog::Format;

/// A wrapper around an asynchronous sink that accepts, serializes, and sends Transmog-encoded
/// values.
///
/// To use, provide a writer that implements [`AsyncWrite`], and then use [`Sink`] to send values.
///
/// Note that an `TransmogWriter` must be of the type [`AsyncDestination`] in order to be
/// compatible with an [`TransmogReader`](super::TransmogReader) on the remote end (recall that it requires the
/// serialized size prefixed to the serialized data). The default is [`SyncDestination`], but these
/// can be easily toggled between using [`TransmogWriter::for_async`].
#[derive(Debug)]
pub struct TransmogWriter<W, T, D, F> {
    format: F,
    writer: W,
    pub(crate) written: usize,
    pub(crate) buffer: Vec<u8>,
    scratch_buffer: Vec<u8>,
    from: PhantomData<T>,
    dest: PhantomData<D>,
}

impl<W, T, D, F> Unpin for TransmogWriter<W, T, D, F> where W: Unpin {}

impl<W, T, D, F> TransmogWriter<W, T, D, F> {
    /// Gets a reference to the underlying format.
    ///
    /// It is inadvisable to directly write to the underlying writer.
    pub fn format(&self) -> &F {
        &self.format
    }

    /// Gets a reference to the underlying writer.
    ///
    /// It is inadvisable to directly write to the underlying writer.
    pub fn get_ref(&self) -> &W {
        &self.writer
    }

    /// Gets a mutable reference to the underlying writer.
    ///
    /// It is inadvisable to directly write to the underlying writer.
    pub fn get_mut(&mut self) -> &mut W {
        &mut self.writer
    }

    /// Unwraps this `TransmogWriter`, returning the underlying writer.
    ///
    /// Note that any leftover serialized data that has not yet been sent is lost.
    pub fn into_inner(self) -> (W, F) {
        (self.writer, self.format)
    }
}

impl<W, T, F> TransmogWriter<W, T, SyncDestination, F> {
    /// Returns a new instance that sends `format`-encoded data over `writer`.
    pub fn new(writer: W, format: F) -> Self {
        TransmogWriter {
            format,
            buffer: Vec::new(),
            scratch_buffer: Vec::new(),
            writer,
            written: 0,
            from: PhantomData,
            dest: PhantomData,
        }
    }

    /// Returns a new instance that sends `format`-encoded data over
    /// `W::defcfault()`.
    pub fn default_for(format: F) -> Self
    where
        W: Default,
    {
        Self::new(W::default(), format)
    }
}

impl<W, T, F> TransmogWriter<W, T, SyncDestination, F> {
    /// Make this writer include the serialized data's size before each serialized value.
    ///
    /// This is necessary for compatability with [`TransmogReader`](super::TransmogReader).
    pub fn for_async(self) -> TransmogWriter<W, T, AsyncDestination, F> {
        self.make_for()
    }
}

impl<W, T, D, F> TransmogWriter<W, T, D, F> {
    pub(crate) fn make_for<D2>(self) -> TransmogWriter<W, T, D2, F> {
        TransmogWriter {
            format: self.format,
            buffer: self.buffer,
            writer: self.writer,
            written: self.written,
            from: self.from,
            scratch_buffer: self.scratch_buffer,
            dest: PhantomData,
        }
    }
}

impl<W, T, F> TransmogWriter<W, T, AsyncDestination, F> {
    /// Make this writer only send Transmog-encoded values.
    ///
    /// This is necessary for compatability with stock Transmog receivers.
    pub fn for_sync(self) -> TransmogWriter<W, T, SyncDestination, F> {
        self.make_for()
    }
}

/// A marker that indicates that the wrapping type is compatible with [`TransmogReader`](super::TransmogReader).
#[derive(Debug)]
pub struct AsyncDestination;

/// A marker that indicates that the wrapping type is compatible with stock Transmog receivers.
#[derive(Debug)]
pub struct SyncDestination;

#[doc(hidden)]
pub trait TransmogWriterFor<T, F>
where
    F: Format<T>,
{
    fn append(&mut self, item: &T) -> Result<(), F::Error>;
}

impl<W, T, F> TransmogWriterFor<T, F> for TransmogWriter<W, T, AsyncDestination, F>
where
    F: Format<T>,
{
    fn append(&mut self, item: &T) -> Result<(), F::Error> {
        if let Some(serialized_length) = self.format.serialized_size(item)? {
            let size = usize_to_u64(serialized_length)?;
            size.encode_variable(&mut self.buffer)?;
            self.format.serialize_into(item, &mut self.buffer)?;
        } else {
            // Use a scratch buffer to measure the size. This introduces an
            // extra data copy, but by reusing the scratch buffer, that should
            // be the only overhead.
            self.scratch_buffer.truncate(0);
            self.format.serialize_into(item, &mut self.scratch_buffer)?;

            let size = usize_to_u64(self.scratch_buffer.len())?;
            size.encode_variable(&mut self.buffer)?;
            self.buffer.append(&mut self.scratch_buffer);
        }
        Ok(())
    }
}

fn usize_to_u64(value: usize) -> Result<u64, std::io::Error> {
    u64::try_from(value).map_err(|_| std::io::Error::from(std::io::ErrorKind::OutOfMemory))
}

impl<W, T, F> TransmogWriterFor<T, F> for TransmogWriter<W, T, SyncDestination, F>
where
    F: Format<T>,
{
    fn append(&mut self, item: &T) -> Result<(), F::Error> {
        self.format.serialize_into(item, &mut self.buffer)
    }
}

impl<W, T, D, F> Sink<T> for TransmogWriter<W, T, D, F>
where
    F: Format<T>,
    W: AsyncWrite + Unpin,
    Self: TransmogWriterFor<T, F>,
{
    type Error = F::Error;

    fn poll_ready(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(mut self: Pin<&mut Self>, item: T) -> Result<(), Self::Error> {
        self.append(&item)?;
        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // allow us to borrow fields separately
        let this = self.get_mut();

        // write stuff out if we need to
        while this.written != this.buffer.len() {
            let n =
                ready!(Pin::new(&mut this.writer).poll_write(cx, &this.buffer[this.written..]))?;
            this.written += n;
        }

        // we have to flush before we're really done
        this.buffer.clear();
        this.written = 0;
        Pin::new(&mut this.writer)
            .poll_flush(cx)
            .map_err(<F::Error as From<std::io::Error>>::from)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        ready!(self.as_mut().poll_flush(cx))?;
        Pin::new(&mut self.writer)
            .poll_shutdown(cx)
            .map_err(<F::Error as From<std::io::Error>>::from)
    }
}
