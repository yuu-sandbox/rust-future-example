#![feature(async_await)]

use std::error::Error;

use tokio::net::UnixListener;
use tokio::net::UnixStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use futures::future::try_join;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "/tmp/can_dummy";
    let mut listener = UnixListener::bind(&addr).unwrap();

    let accept = listener.accept();
    let connect = UnixStream::connect(&addr);

    let ((mut server, _), mut client) = try_join(accept, connect).await?;
    let write_len = client.write(b"hello").await?;
    assert_eq!(write_len, 5);
    drop(client);

    let mut buf = [0u8; 5];
    server.read_exact(&mut buf).await?;
    assert_eq!(&buf, b"hello");
    let len = server.read(&mut buf).await?;
    assert_eq!(len, 0);

    Ok(())
}
