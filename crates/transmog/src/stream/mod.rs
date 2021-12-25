//! Asynchronous access to a Transmog-encoded item stream.
//!
//! This crate enables you to asynchronously read from a Transmog-encoded
//! stream, or write transmog-encoded values. Most serialization format do not
//! natively support serializing and deserializing in an asynchronous
//! environment.
//!
//! Transmog works around that on the receive side by buffering received bytes
//! until a full element's worth of data has been received, and only then
//! calling into the underlying [`Format`]. To make this work, it relies on the
//! sender to prefix each encoded element with its encoded size.
//!
//! On the write side, Transmog buffers the serialized values, and
//! asynchronously sends the resulting bytestream.
//!
//! This module has been adapted from
//! [`async-bincode`](https://github.com/jonhoo/async-bincode) to generically
//! support different serialization formats, as well as this crates own
//! versioning features.
#![warn(missing_docs)]

mod reader;
mod writer;

use std::{
    fmt, io,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    pin::Pin,
    task::{Context, Poll},
};

use futures_core::Stream;
use futures_sink::Sink;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncRead, ReadBuf};

pub use self::{
    reader::TransmogReader,
    writer::{AsyncDestination, SyncDestination, TransmogWriter, TransmogWriterFor},
};
use crate::format::Format;

/// Builder helper to specify types without the need of turbofishing.
pub struct Builder<TReads, TWrites, TStream, TFormat> {
    stream: TStream,
    format: TFormat,
    datatypes: PhantomData<(TReads, TWrites)>,
}

impl<TStream, TFormat> Builder<(), (), TStream, TFormat> {
    /// Returns a new stream builder for `stream` and `format`.
    pub fn new(stream: TStream, format: TFormat) -> Self {
        Self {
            stream,
            format,
            datatypes: PhantomData,
        }
    }
}

impl<TStream, TFormat> Builder<(), (), TStream, TFormat> {
    /// Sets `T` as the type for both sending and receiving.
    pub fn sends_and_receives<T: Serialize + for<'de> Deserialize<'de>>(
        self,
    ) -> Builder<T, T, TStream, TFormat> {
        Builder {
            stream: self.stream,
            format: self.format,
            datatypes: PhantomData,
        }
    }
}

impl<TReads, TStream, TFormat> Builder<TReads, (), TStream, TFormat> {
    /// Sets `T` as the type of data that is written to this stream.
    pub fn sends<T: Serialize + for<'de> Deserialize<'de>>(
        self,
    ) -> Builder<TReads, T, TStream, TFormat> {
        Builder {
            stream: self.stream,
            format: self.format,
            datatypes: PhantomData,
        }
    }
}

impl<TWrites, TStream, TFormat> Builder<(), TWrites, TStream, TFormat> {
    /// Sets `T` as the type of data that is read from this stream.
    pub fn receives<T: Serialize + for<'de> Deserialize<'de>>(
        self,
    ) -> Builder<T, TWrites, TStream, TFormat> {
        Builder {
            stream: self.stream,
            format: self.format,
            datatypes: PhantomData,
        }
    }
}

impl<TReads, TWrites, TStream, TFormat> Builder<TReads, TWrites, TStream, TFormat>
where
    TFormat: Clone,
{
    /// Build this stream to include the serialized data's size before each
    /// serialized value.
    ///
    /// This is necessary for compatability with a remote [`TransmogReader`].
    pub fn for_async(self) -> TransmogStream<TReads, TWrites, TStream, AsyncDestination, TFormat> {
        TransmogStream::new(self.stream, self.format).for_async()
    }

    /// Build this stream only send Transmog-encoded values.
    ///
    /// This is necessary for compatability with stock Transmog receivers.
    pub fn for_sync(self) -> TransmogStream<TReads, TWrites, TStream, SyncDestination, TFormat> {
        TransmogStream::new(self.stream, self.format)
    }
}

/// A wrapper around an asynchronous stream that receives and sends bincode-encoded values.
///
/// To use, provide a stream that implements both [`AsyncWrite`](tokio::io::AsyncWrite) and [`AsyncRead`], and then use
/// [`Sink`] to send values and [`Stream`] to receive them.
///
/// Note that an `TransmogStream` must be of the type [`AsyncDestination`] in order to be
/// compatible with an [`TransmogReader`] on the remote end (recall that it requires the
/// serialized size prefixed to the serialized data). The default is [`SyncDestination`], but these
/// can be easily toggled between using [`TransmogStream::for_async`].
#[derive(Debug)]
pub struct TransmogStream<TReads, TWrites, TStream, TDestination, TFormat> {
    stream: TransmogReader<
        InternalTransmogWriter<TStream, TWrites, TDestination, TFormat>,
        TReads,
        TFormat,
    >,
}

#[doc(hidden)]
pub struct InternalTransmogWriter<TStream, T, TDestination, TFormat>(
    TransmogWriter<TStream, T, TDestination, TFormat>,
);

impl<TStream: fmt::Debug, T, TDestination, TFormat> fmt::Debug
    for InternalTransmogWriter<TStream, T, TDestination, TFormat>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.get_ref().fmt(f)
    }
}

impl<TReads, TWrites, TStream, TDestination, TFormat>
    TransmogStream<TReads, TWrites, TStream, TDestination, TFormat>
{
    /// Gets a reference to the underlying stream.
    ///
    /// It is inadvisable to directly read from or write to the underlying stream.
    pub fn get_ref(&self) -> &TStream {
        self.stream.get_ref().0.get_ref()
    }

    /// Gets a mutable reference to the underlying stream.
    ///
    /// It is inadvisable to directly read from or write to the underlying stream.
    pub fn get_mut(&mut self) -> &mut TStream {
        self.stream.get_mut().0.get_mut()
    }

    /// Unwraps this `TransmogStream`, returning the underlying stream.
    ///
    /// Note that any leftover serialized data that has not yet been sent, or received data that
    /// has not yet been deserialized, is lost.
    pub fn into_inner(self) -> (TStream, TFormat) {
        self.stream.into_inner().0.into_inner()
    }
}

impl<TStream, TFormat> TransmogStream<(), (), TStream, SyncDestination, TFormat> {
    /// Creates a new instance that sends `format`-encoded payloads over `stream`.
    pub fn build(stream: TStream, format: TFormat) -> Builder<(), (), TStream, TFormat> {
        Builder::new(stream, format)
    }
}

impl<TReads, TWrites, TStream, TFormat>
    TransmogStream<TReads, TWrites, TStream, SyncDestination, TFormat>
where
    TFormat: Clone,
{
    /// Creates a new instance that sends `format`-encoded payloads over `stream`.
    pub fn new(stream: TStream, format: TFormat) -> Self {
        TransmogStream {
            stream: TransmogReader::new(
                InternalTransmogWriter(TransmogWriter::new(stream, format.clone())),
                format,
            ),
        }
    }

    /// Creates a new instance that sends `format`-encoded payloads over the
    /// default stream for `TStream`.
    pub fn default_for(format: TFormat) -> Self
    where
        TStream: Default,
    {
        Self::new(TStream::default(), format)
    }
}

impl<TReads, TWrites, TStream, TDestination, TFormat>
    TransmogStream<TReads, TWrites, TStream, TDestination, TFormat>
where
    TFormat: Clone,
{
    /// Make this stream include the serialized data's size before each serialized value.
    ///
    /// This is necessary for compatability with a remote [`TransmogReader`].
    pub fn for_async(self) -> TransmogStream<TReads, TWrites, TStream, AsyncDestination, TFormat> {
        let (stream, format) = self.into_inner();
        TransmogStream {
            stream: TransmogReader::new(
                InternalTransmogWriter(TransmogWriter::new(stream, format.clone()).for_async()),
                format,
            ),
        }
    }

    /// Make this stream only send Transmog-encoded values.
    ///
    /// This is necessary for compatability with stock Transmog receivers.
    pub fn for_sync(self) -> TransmogStream<TReads, TWrites, TStream, SyncDestination, TFormat> {
        let (stream, format) = self.into_inner();
        TransmogStream::new(stream, format)
    }
}

/// A reader of Transmog-encoded data from a [`TcpStream`](tokio::net::TcpStream).
pub type TransmogTokioTcpReader<'a, TReads, TFormat> =
    TransmogReader<tokio::net::tcp::ReadHalf<'a>, TReads, TFormat>;
/// A writer of Transmog-encoded data to a [`TcpStream`](tokio::net::TcpStream).
pub type TransmogTokioTcpWriter<'a, TWrites, TDestination, TFormat> =
    TransmogWriter<tokio::net::tcp::WriteHalf<'a>, TWrites, TDestination, TFormat>;

impl<TReads, TWrites, TDestination, TFormat>
    TransmogStream<TReads, TWrites, tokio::net::TcpStream, TDestination, TFormat>
where
    TFormat: Clone,
{
    /// Split a TCP-based stream into a read half and a write half.
    ///
    /// This is more performant than using a lock-based split like the one provided by `tokio-io`
    /// or `futures-util` since we know that reads and writes to a `TcpStream` can continue
    /// concurrently.
    ///
    /// Any partially sent or received state is preserved.
    pub fn tcp_split(
        &mut self,
    ) -> (
        TransmogTokioTcpReader<TReads, TFormat>,
        TransmogTokioTcpWriter<TWrites, TDestination, TFormat>,
    ) {
        // First, steal the reader state so it isn't lost
        let rbuff = self.stream.buffer.split();
        // Then, fish out the writer
        let writer = &mut self.stream.get_mut().0;
        let format = writer.format().clone();
        // And steal the writer state so it isn't lost
        let wbuff = writer.buffer.split_off(0);
        let wsize = writer.written;
        // Now split the stream
        let (r, w) = writer.get_mut().split();
        // Then put the reader back together
        let mut reader = TransmogReader::new(r, format.clone());
        reader.buffer = rbuff;
        // And then the writer
        let mut writer: TransmogWriter<_, _, TDestination, TFormat> =
            TransmogWriter::new(w, format).make_for();
        writer.buffer = wbuff;
        writer.written = wsize;
        // All good!
        (reader, writer)
    }
}

impl<TStream, T, TDestination, TFormat> AsyncRead
    for InternalTransmogWriter<TStream, T, TDestination, TFormat>
where
    TStream: AsyncRead + Unpin,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut ReadBuf,
    ) -> Poll<Result<(), io::Error>> {
        Pin::new(self.get_mut().get_mut()).poll_read(cx, buf)
    }
}

impl<TStream, T, TDestination, TFormat> Deref
    for InternalTransmogWriter<TStream, T, TDestination, TFormat>
{
    type Target = TransmogWriter<TStream, T, TDestination, TFormat>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<TStream, T, TDestination, TFormat> DerefMut
    for InternalTransmogWriter<TStream, T, TDestination, TFormat>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<TReads, TWrites, TStream, TDestination, TFormat> Stream
    for TransmogStream<TReads, TWrites, TStream, TDestination, TFormat>
where
    TStream: Unpin,
    TransmogReader<
        InternalTransmogWriter<TStream, TWrites, TDestination, TFormat>,
        TReads,
        TFormat,
    >: Stream<Item = Result<TReads, TFormat::Error>>,
    TFormat: Format<TWrites>,
{
    type Item = Result<TReads, TFormat::Error>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.stream).poll_next(cx)
    }
}

impl<TReads, TWrites, TStream, TDestination, TFormat> Sink<TWrites>
    for TransmogStream<TReads, TWrites, TStream, TDestination, TFormat>
where
    TStream: Unpin,
    TransmogWriter<TStream, TWrites, TDestination, TFormat>: Sink<TWrites, Error = TFormat::Error>,
    TFormat: Format<TWrites>,
{
    type Error = TFormat::Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut **self.stream.get_mut()).poll_ready(cx)
    }

    fn start_send(mut self: Pin<&mut Self>, item: TWrites) -> Result<(), Self::Error> {
        Pin::new(&mut **self.stream.get_mut()).start_send(item)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut **self.stream.get_mut()).poll_flush(cx)
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut **self.stream.get_mut()).poll_close(cx)
    }
}

#[cfg(test)]
mod tests {
    use futures::prelude::*;
    use serde::{de::DeserializeOwned, Serialize};
    use tokio::io::AsyncWriteExt;

    use super::*;
    use crate::format::{Bincode, Pot};

    async fn it_works<
        T: Serialize + DeserializeOwned + std::fmt::Debug + Clone + PartialEq + Send,
        TFormat: Format<T> + Clone + 'static,
    >(
        format: TFormat,
        values: &[T],
    ) {
        let echo = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = echo.local_addr().unwrap();

        let task_format = format.clone();
        tokio::spawn(async move {
            let (stream, _) = echo.accept().await.unwrap();
            let mut stream = TransmogStream::<T, T, _, _, _>::new(stream, task_format).for_async();
            let (r, w) = stream.tcp_split();
            r.forward(w).await.unwrap();
        });

        let client = tokio::net::TcpStream::connect(&addr).await.unwrap();
        let mut client = TransmogStream::<T, T, _, _, _>::new(client, format).for_async();

        for value in values {
            client.send(value.clone()).await.unwrap();
            assert_eq!(&client.next().await.unwrap().unwrap(), value);
        }

        drop(client);
    }

    #[tokio::test]
    async fn it_works_bincode() {
        // Test short payloads
        it_works(Bincode, &[44, 42]).await;
        // Test a long payload
        it_works(Bincode, &[vec![0_u8; 1_000_000]]).await;
    }

    #[tokio::test]
    async fn it_works_pot() {
        // Test short payloads
        it_works(Pot, &[44, 42]).await;
        // Test a long payload
        it_works(Pot, &[vec![0_u8; 1_000_000]]).await;
    }

    #[tokio::test]
    async fn lots() {
        let echo = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = echo.local_addr().unwrap();

        tokio::spawn(async move {
            let (stream, _) = echo.accept().await.unwrap();
            let mut stream =
                TransmogStream::<usize, usize, _, _, _>::new(stream, Bincode).for_async();
            let (r, w) = stream.tcp_split();
            r.forward(w).await.unwrap();
        });

        let n = 81920;
        let stream = tokio::net::TcpStream::connect(&addr).await.unwrap();
        let mut c = TransmogStream::new(stream, Bincode).for_async();

        futures::stream::iter(0usize..n)
            .map(Ok)
            .forward(&mut c)
            .await
            .unwrap();

        c.get_mut().shutdown().await.unwrap();

        let mut at = 0;
        while let Some(got) = c.next().await.transpose().unwrap() {
            assert_eq!(at, got);
            at += 1;
        }
        assert_eq!(at, n);
    }
}
