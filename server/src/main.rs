mod proto;
mod snake;

use crate::proto::snake::snake_server_server::SnakeServerServer;
use crate::proto::snake::{ChatMessage, Login, PlayerMove, PlayerState, SendResult};
use crate::snake::Snake;
use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;
use prost::alloc::sync::Arc;
use std::collections::HashMap;
use structopt::StructOpt;
use tokio::sync::mpsc::Sender;
use tokio::sync::{mpsc, Mutex};
use tokio::time::Duration;
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Server;
use tonic::{Request, Response, Status};
use tracing::{info, span, Level};

type ChatClients = HashMap<String, Sender<Result<ChatMessage, Status>>>;

#[derive(Debug, Clone)]
struct SnakeServer {
    chat: Arc<Chat>,
    snake: Arc<snake::Snake>,
}

impl SnakeServer {
    fn new() -> Self {
        Self {
            chat: Arc::new(Chat::default()),
            snake: Arc::new(Snake::new()),
        }
    }
}

#[derive(Debug, Default)]
struct Chat {
    chat_clients: Mutex<ChatClients>,
}

impl Chat {
    async fn add_client(&self, name: String, sender: Sender<Result<ChatMessage, Status>>) {
        let mut clients = self.chat_clients.lock().await;
        clients.insert(name, sender);
    }

    async fn send_message(&self, message: &ChatMessage) {
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
    async fn make_move(
        &self,
        request: Request<PlayerMove>,
    ) -> Result<Response<SendResult>, Status> {
        self.snake.make_move(request.get_ref()).await;
        Ok(Response::new(SendResult {}))
    }

    type ObserveGameStateStream = ReceiverStream<Result<PlayerState, Status>>;
    async fn observe_game_state(
        &self,
        request: Request<Login>,
    ) -> Result<Response<Self::ObserveGameStateStream>, Status> {
        let (tx, rx) = mpsc::channel(4);
        self.snake.add_client(request.into_inner().user, tx).await;
        Ok(Response::new(Self::ObserveGameStateStream::new(rx)))
    }

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
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let game_running = Arc::new(AtomicBool::new(true));
    let opt = Opt::from_args();
    let snake_server = SnakeServer::new();

    init_tracing();
    spawn_game_updater(game_running.clone(), snake_server.clone());
    spawn_client_updater(game_running.clone(), snake_server.clone());
    run_grpc_server(&opt, snake_server).await?;

    Ok(())
}

async fn run_grpc_server(
    opt: &Opt,
    snake_server: SnakeServer,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting server on port {}", opt.port);
    let addr = format!("[::1]:{}", opt.port).parse()?;
    Server::builder()
        .trace_fn(|request| {
            span!(
                Level::INFO,
                "Request",
                "{} {}",
                request.method(),
                request.uri().to_string()
            )
        })
        .add_service(SnakeServerServer::new(snake_server))
        .serve(addr)
        .await?;

    Ok(())
}

fn init_tracing() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing_subscriber::filter::LevelFilter::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

fn spawn_client_updater(game_running: Arc<AtomicBool>, snake_server: SnakeServer) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_millis(200));
        while game_running.load(Ordering::SeqCst) {
            let _ = interval.tick().await;
            snake_server.snake.update_clients().await;
        }
    });
}

fn spawn_game_updater(game_running: Arc<AtomicBool>, snake_server1: SnakeServer) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_millis(200));
        while game_running.load(Ordering::SeqCst) {
            let _ = interval.tick().await;
            snake_server1.snake.update().await;
        }
    });
}
