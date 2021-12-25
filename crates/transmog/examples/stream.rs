//! Demonstrates using async streaming serialization/deserialization with any
//! `Format` implementor. In this example, [`Pot]`is the format.

use futures::{SinkExt, StreamExt};
use transmog::{format::Pot, stream::TransmogStream};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let echo = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = echo.local_addr().unwrap();

    tokio::spawn(async move {
        let (stream, _) = echo.accept().await.unwrap();
        let mut stream = TransmogStream::build(stream, Pot)
            .sends_and_receives::<u64>()
            .for_async();
        let (r, w) = stream.tcp_split();
        r.forward(w).await.unwrap();
    });

    let client = tokio::net::TcpStream::connect(&addr).await.unwrap();
    let mut client = TransmogStream::build(client, Pot)
        .sends_and_receives::<u64>()
        .for_async();

    client.send(42).await.unwrap();
    assert_eq!(client.next().await.unwrap().unwrap(), 42);

    drop(client);
    Ok(())
}

#[test]
fn runs() {
    main().unwrap();
}
