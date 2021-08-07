use crate::proto::snake::snake_server_client::SnakeServerClient;
use crate::proto::snake::{ChatMessage, Login};
use tokio::io;
use tokio::io::{AsyncBufReadExt, BufReader};
mod proto;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SnakeServerClient::connect("http://[::1]:50051").await?;



    let stdin = io::stdin();
    let mut lines = BufReader::new(stdin).lines();
    println!("Enter Name");
    let user = lines.next_line().await?.unwrap();
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
        let line = lines.next_line().await?.unwrap();
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
