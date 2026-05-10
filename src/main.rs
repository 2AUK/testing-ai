use bytes::BytesMut;
use rig::agent::Agent;
use rig::client::{CompletionClient, Nothing};
use rig::completion::Prompt;
use rig::providers::ollama::{self, CompletionModel};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
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
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    let client = ollama::Client::new(Nothing)?;

    let agent = client.agent("lfm2.5-thinking").preamble("You are a research assistant who specialises in system design, systems thinking, and Rust").build();

    loop {
        let (stream, _) = listener.accept().await?;
        handle_connection(stream, &agent).await;
    }
}

async fn handle_connection(mut stream: tokio::net::TcpStream, agent: &Agent<CompletionModel>) {
    let mut buf = BytesMut::with_capacity(1024);
    stream.read_buf(&mut buf).await.unwrap();
    println!("{:?}", buf);
    let request: AgentRequest = serde_json::from_slice(&buf).unwrap();
    let response = agent.prompt(request.request.as_str()).await.unwrap();
    stream.write_all(response.as_bytes()).await.unwrap();
}
