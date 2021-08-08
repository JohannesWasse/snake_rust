mod proto;

use crate::proto::snake::snake_server_client::SnakeServerClient;
use crate::proto::snake::{ChatMessage, Login};
use tokio::io;
use tokio::io::{AsyncBufReadExt, BufReader};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SnakeServerClient::connect("http://[::1]:50051").await?;
    println!("Enter Name");
    let user = read_next_line().await?;
    let request = tonic::Request::new(Login {
        user: user.clone(),
    });

    let mut response = client.receive_message(request).await?.into_inner();
    tokio::spawn(async move {
        while let Ok(Some(message)) = response.message().await {
            println!("{}:\n{}", message.user, message.message)
        }
    });
    loop {
        let line = read_next_line().await?;
        if line == "exit" {
            break;
        }
        let request = tonic::Request::new(ChatMessage {
            user: user.clone(),
            message: line,
        });
        client.send_message(request).await?;
    }
    Ok(())
}

async fn read_next_line() -> Result<String, Box<dyn std::error::Error>> {
    loop {
        let stdin = io::stdin();
        let mut lines = BufReader::new(stdin).lines();
        if let Some(line) = lines.next_line().await? {
            return Ok(line);
        }
    }
}
