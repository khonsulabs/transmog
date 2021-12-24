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
        let mut stream = TransmogStream::<u64, u64, _, _, _>::new(stream, Pot).for_async();
        let (r, w) = stream.tcp_split();
        r.forward(w).await.unwrap();
    });

    let client = tokio::net::TcpStream::connect(&addr).await.unwrap();
    let mut client = TransmogStream::<u64, u64, _, _, _>::new(client, Pot).for_async();

    client.send(42_u64).await.unwrap();
    assert_eq!(client.next().await.unwrap().unwrap(), 42_u64);

    drop(client);
    Ok(())
}

#[test]
fn runs() {
    main().unwrap();
}
