mod proto;

use crate::proto::snake::snake_server_server::SnakeServerServer;
use crate::proto::snake::{ChatMessage, Login, SendResult};
use tokio::sync::mpsc::Sender;
use tokio::sync::{mpsc, Mutex};
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Server;
use tonic::{Request, Response, Status};
use std::collections::HashMap;
use structopt::StructOpt;
use tracing::{info, span, Level};

#[derive(Debug, Default)]
struct SnakeServer {
    chat: Chat
}

type ChatClients = HashMap<String, Sender<Result<ChatMessage, Status>>>;

#[derive(Debug, Default)]
struct Chat {
    chat_clients: Mutex<ChatClients>,
}

impl Chat {
    async fn add_client(&self, name: String, sender: Sender<Result<ChatMessage, Status>>) {
        let mut clients = self.chat_clients.lock().await;
        clients.insert(name, sender);
    }

    async fn send_message(&self, message: &ChatMessage){
        let mut clients = self.chat_clients.lock().await;
        let clone = clients.clone();
        for (name, client) in clone.iter() {
            if *name == message.user {
                continue;
            }
            let result = client
                .send(Ok(ChatMessage {
                    user: message.user.clone(),
                    message: message.message.clone(),
                }))
                .await;
            if let Err(err) = result {
                info!("Removing client {:?}", err.to_string());
                clients.remove(name);
            }
        }
    }
}

#[tonic::async_trait]
impl proto::snake::snake_server_server::SnakeServer for SnakeServer {
    type ReceiveMessageStream = ReceiverStream<Result<ChatMessage, Status>>;

    async fn receive_message(
        &self,
        request: Request<Login>,
    ) -> Result<Response<Self::ReceiveMessageStream>, Status> {
        let (tx, rx) = mpsc::channel(4);
        self.chat.add_client(request.into_inner().user, tx).await;
        Ok(Response::new(Self::ReceiveMessageStream::new(rx)))
    }

    async fn send_message(
        &self,
        request: Request<ChatMessage>,
    ) -> Result<Response<SendResult>, Status> {
        let message = request.into_inner();
        self.chat.send_message(&message).await;
        Ok(Response::new(SendResult {}))
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short, long, default_value = "50051")]
    port: u16
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(tracing_subscriber::filter::LevelFilter::DEBUG)
        // completes the builder.
        .finish();
    let opt = Opt::from_args();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
    info!("Starting server on port {}", opt.port);
    let addr = format!("[::1]:{}", opt.port).parse()?;
    let snake_server = SnakeServer::default();

    Server::builder()
        .trace_fn(|request|{
            span!(Level::INFO, "Request", "{} {}", request.method(), request.uri().to_string())
        })
        .add_service(SnakeServerServer::new(snake_server))
        .serve(addr)
        .await?;

    Ok(())
}
