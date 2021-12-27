use std::{
    io,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

use bytes::{Buf, BytesMut};
use futures_core::{ready, Stream};
use ordered_varint::Variable;
use tokio::io::{AsyncRead, ReadBuf};
use transmog::Format;

/// A wrapper around an asynchronous reader that produces an asynchronous stream
/// of Transmog-decoded values.
///
/// To use, provide a reader that implements [`AsyncRead`], and then use
/// [`Stream`] to access the deserialized values.
///
/// Note that the sender *must* prefix each serialized item with its size
/// encoded using [`ordered-varint`](ordered_varint).
#[derive(Debug)]
pub struct TransmogReader<R, T, F> {
    format: F,
    reader: R,
    pub(crate) buffer: BytesMut,
    into: PhantomData<T>,
}

impl<R, T, F> Unpin for TransmogReader<R, T, F> where R: Unpin {}

impl<R, T, F> TransmogReader<R, T, F> {
    /// Gets a reference to the underlying reader.
    ///
    /// It is inadvisable to directly read from the underlying reader.
    pub fn get_ref(&self) -> &R {
        &self.reader
    }

    /// Gets a mutable reference to the underlying reader.
    ///
    /// It is inadvisable to directly read from the underlying reader.
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.reader
    }

    /// Returns a reference to the internally buffered data.
    ///
    /// This will not attempt to fill the buffer if it is empty.
    pub fn buffer(&self) -> &[u8] {
        &self.buffer[..]
    }

    /// Unwraps this `TransmogReader`, returning the underlying reader.
    ///
    /// Note that any leftover data in the internal buffer is lost.
    pub fn into_inner(self) -> R {
        self.reader
    }
}

impl<R, T, F> TransmogReader<R, T, F> {
    /// Returns a new instance that reads `format`-encoded data for `reader`.
    pub fn new(reader: R, format: F) -> Self {
        TransmogReader {
            format,
            buffer: BytesMut::with_capacity(8192),
            reader,
            into: PhantomData,
        }
    }

    /// Returns a new instance that reads `format`-encoded data for `R::default()`.
    pub fn default_for(format: F) -> Self
    where
        R: Default,
    {
        Self::new(R::default(), format)
    }
}

impl<R, T, F> Stream for TransmogReader<R, T, F>
where
    R: AsyncRead + Unpin,
    F: Format<T>,
{
    type Item = Result<T, F::Error>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            let fill_result = ready!(self
                .as_mut()
                .fill(cx, 9)
                .map_err(<F::Error as From<std::io::Error>>::from))?;

            let mut buf_reader = &self.buffer[..];
            let buffer_start = buf_reader.as_ptr() as usize;
            if let Ok(message_size) = u64::decode_variable(&mut buf_reader) {
                let header_len = buf_reader.as_ptr() as usize - buffer_start;
                let target_buffer_size = usize::try_from(message_size).unwrap() + header_len;

                ready!(self
                    .as_mut()
                    .fill(cx, target_buffer_size)
                    .map_err(<F::Error as From<std::io::Error>>::from))?;

                if self.buffer.len() >= target_buffer_size {
                    let message = self
                        .format
                        .deserialize(&self.buffer[header_len..target_buffer_size])
                        .unwrap();
                    self.buffer.advance(target_buffer_size);
                    break Poll::Ready(Some(Ok(message)));
                }
            } else if let ReadResult::Eof = fill_result {
                break Poll::Ready(None);
            }
        }
    }
}

#[derive(Debug)]
enum ReadResult {
    ReceivedData,
    Eof,
}

impl<R, T, F> TransmogReader<R, T, F>
where
    R: AsyncRead + Unpin,
{
    fn fill(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        target_size: usize,
    ) -> Poll<Result<ReadResult, io::Error>> {
        if self.buffer.len() >= target_size {
            // we already have the bytes we need!
            return Poll::Ready(Ok(ReadResult::ReceivedData));
        }

        // make sure we can fit all the data we're about to read
        // and then some, so we don't do a gazillion syscalls
        if self.buffer.capacity() < target_size {
            let missing = target_size - self.buffer.capacity();
            self.buffer.reserve(missing);
        }

        let had = self.buffer.len();
        // this is the bit we'll be reading into
        let mut rest = self.buffer.split_off(had);
        // this is safe because we're not extending beyond the reserved capacity
        // and we're never reading unwritten bytes
        let max = rest.capacity();
        // In the original implementation, this was an unsafe operation.
        // unsafe { rest.set_len(max) };
        rest.resize(max, 0);

        let mut buf = ReadBuf::new(&mut rest[..]);
        ready!(Pin::new(&mut self.reader).poll_read(cx, &mut buf))?;
        let n = buf.filled().len();
        // adopt the new bytes
        let read = rest.split_to(n);
        self.buffer.unsplit(read);
        if n == 0 {
            return Poll::Ready(Ok(ReadResult::Eof));
        }

        Poll::Ready(Ok(ReadResult::ReceivedData))
    }
}
