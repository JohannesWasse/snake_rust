use crate::proto::snake as proto;
use std::collections::HashMap;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use tonic::Status;

type SnakeClients = HashMap<String, Sender<Result<proto::PlayerState, Status>>>;

#[derive(Debug)]
pub(crate) struct Snake {
    clients: Mutex<SnakeClients>,
    line_stripe: Mutex<Vec<Point>>,
    direction: Mutex<Direction>,
}

impl Snake {
    pub fn new() -> Snake {
        Self {
            clients: Default::default(),
            line_stripe: Mutex::new(vec![Point::new(10, 10), Point::new(15, 10)]),
            direction: Mutex::new(Direction::Right),
        }
    }

    pub async fn update(&self) {
        let mut line_stripe = &mut *self.line_stripe.lock().await;
        let point = line_stripe.last().expect("Never Zero length");
        let direction = *self.direction.lock().await;
        if line_stripe.len() == 1 {
            line_stripe.push(point.move_in_direction(direction));
        } else {
            let prev = &line_stripe[line_stripe.len() - 2];
            let can_update = direction == point.direction(prev);
            if can_update {
                let len = line_stripe.len();
                line_stripe[len - 1] = point.move_in_direction(direction);
            } else {
                line_stripe.push(point.move_in_direction(direction));
            }
        }
    }

    pub(crate) async fn add_client(
        &self,
        name: String,
        sender: Sender<Result<proto::PlayerState, Status>>,
    ) {
        let mut clients = self.clients.lock().await;
        clients.insert(name, sender);
    }

    async fn line_stripe_proto(&self) -> Vec<proto::Point> {
        let mut line_stripe = &mut *self.line_stripe.lock().await;
        line_stripe
            .iter()
            .map(|p| proto::Point { x: p.x, y: p.y })
            .collect()
    }

    async fn clients_clone(&self) -> SnakeClients {
        let clients = self.clients.lock().await;
        clients.clone()
    }

    async fn send_message(&self, new_move: &proto::PlayerMove) {}
    pub async fn update_clients(&self) {
        let clients = self.clients_clone().await;
        let mut to_be_removed = Vec::new();
        let proto_message = proto::PlayerState {
            line_stripe: self.line_stripe_proto().await,
        };
        for (name, client) in clients.iter() {
            let result = client.send(Ok(proto_message.clone())).await;
            if let Err(err) = result {
                tracing::info!("Removing client {:?}", err.to_string());
                to_be_removed.push(name.clone());
            }
        }
        self.remove_clients(&to_be_removed).await
    }

    async fn remove_clients(&self, to_be_removed: &Vec<String>) {
        let mut clients = self.clients.lock().await;
        for client in to_be_removed {
            clients.remove(client);
        }
    }
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn direction(&self, prev: &Self) -> Direction {
        if self.y - prev.y == 0 {
            if self.x - prev.x > 0 {
                return Direction::Right;
            } else {
                return Direction::Left;
            }
        }
        if self.x - prev.x == 0 {
            if self.y - prev.y > 0 {
                return Direction::Up;
            } else {
                return Direction::Down;
            }
        }
        panic!()
    }

    fn move_in_direction(&self, d: Direction) -> Point {
        match d {
            Direction::Down => self.add_y(-1),
            Direction::Left => self.add_x(-1),
            Direction::Right => self.add_x(1),
            Direction::Up => self.add_y(1),
        }
    }

    fn add_x(&self, dx: i32) -> Point {
        Point {
            x: self.x + dx,
            y: self.y,
        }
    }

    fn add_y(&self, dy: i32) -> Point {
        Point {
            x: self.x,
            y: self.y + dy,
        }
    }
}
