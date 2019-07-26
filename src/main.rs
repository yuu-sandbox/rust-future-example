#![feature(async_await)]
use std::os::unix::net::SocketAddr;
use tokio_uds::{UnixListener, UnixStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn handler(mut stream: UnixStream, addr: SocketAddr) {
    eprintln!("connected: {:?}", addr);
    let mut buf = [0u8; 5];
    stream.read_exact(&mut buf).await;
    eprintln!("{:?}", buf);
}

/// echo -en "OK" |socat stdio /tmp/can_dummy
#[tokio::main]
async fn main() {
    let addr = "/tmp/can_dummy";
    let mut listener = UnixListener::bind(&addr).unwrap();

    while let Ok(client) = listener.accept().await {
        let (stream, addr) = client;
        handler(stream, addr).await;
    }
}
