use crate::proto::snake::snake_server_server::SnakeServerServer;
use crate::proto::snake::{ChatMessage, Login, SendResult};
use tokio::sync::mpsc::Sender;
use tokio::sync::{mpsc, Mutex};
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

mod proto;

#[derive(Debug, Default)]
struct SnakeServer {
    chat_clients: Mutex<Vec<Sender<Result<ChatMessage, Status>>>>,
}

#[tonic::async_trait]
impl proto::snake::snake_server_server::SnakeServer for SnakeServer {
    type ReceiveMessageStream = ReceiverStream<Result<ChatMessage, Status>>;

    async fn receive_message(
        &self,
        _request: Request<Login>,
    ) -> Result<Response<Self::ReceiveMessageStream>, Status> {
        let (tx, rx) = mpsc::channel(4);
        let mut lock = self.chat_clients.lock().await;
        lock.push(tx);
        Ok(Response::new(Self::ReceiveMessageStream::new(rx)))
    }

    async fn send_message(
        &self,
        request: Request<ChatMessage>,
    ) -> Result<Response<SendResult>, Status> {
        let message = request.into_inner();
        let clients = self.chat_clients.lock().await;
        for client in clients.iter() {
            let result = client
                .send(Ok(ChatMessage {
                    user: message.user.clone(),
                    message: message.message.clone(),
                }))
                .await;
            if let Err(err) = result {
                println!("{:?}", err.to_string());
            }
        }
        Ok(Response::new(SendResult {}))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let snake_server = SnakeServer::default();

    Server::builder()
        .add_service(SnakeServerServer::new(snake_server))
        .serve(addr)
        .await?;

    Ok(())
}
