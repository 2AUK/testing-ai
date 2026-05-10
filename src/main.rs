use bytes::BytesMut;
use serde::{Deserialize, Serialize};
use tokio::{io::AsyncReadExt, net::TcpListener};

#[derive(Debug, Deserialize)]
struct Username(String);

#[derive(Debug, Deserialize)]
struct AgentRequest {
    username: Username,
    time: String,
    request: String,
}

#[derive(Serialize)]
struct AgentResponse {
    time: String,
    response: String,
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(mut stream: tokio::net::TcpStream) {
    let mut buf = BytesMut::with_capacity(1024);
    stream.read_buf(&mut buf).await.unwrap();
    println!("{:?}", buf);
    let request: AgentRequest = serde_json::from_slice(&buf).unwrap();
    println!("{:?}", request);
}
