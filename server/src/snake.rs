use crate::proto::snake as proto;
use std::collections::HashMap;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use tonic::Status;

type SnakeClients = HashMap<String, Sender<Result<proto::PlayerState, Status>>>;

#[derive(Debug)]
pub(crate) struct Snake {
    clients: Mutex<SnakeClients>,
    line_stripe: Mutex<LineStripe>,
    direction: Mutex<Direction>,
}

impl Snake {
    pub fn new() -> Snake {
        Self {
            clients: Default::default(),
            line_stripe: Mutex::new(LineStripe::new()),
            direction: Mutex::new(Direction::Right),
        }
    }

    pub async fn update(&self) {
        let line_stripe = &mut *self.line_stripe.lock().await;
        let d = self.direction.lock().await;
        line_stripe.move_in_direction(*d);
    }

    pub(crate) async fn add_client(
        &self,
        name: String,
        sender: Sender<Result<proto::PlayerState, Status>>,
    ) {
        let mut clients = self.clients.lock().await;
        clients.insert(name, sender);
        let mut l = self.line_stripe.lock().await;
        *l = LineStripe::new();
    }

    async fn line_stripe_proto(&self) -> Vec<proto::Point> {
        let line_stripe = self.line_stripe.lock().await;
        line_stripe.line_stripe_proto()
    }

    async fn clients_clone(&self) -> SnakeClients {
        let clients = self.clients.lock().await;
        clients.clone()
    }

    pub async fn make_move(&self, new_move: &proto::PlayerMove) {
        let mut d = self.direction.lock().await;
        *d = match new_move.direction {
            0 => Direction::Up,
            1 => Direction::Left,
            2 => Direction::Down,
            3 => Direction::Right,
            _ => panic!("Invalid value for direction {}", new_move.direction),
        };
    }

    pub async fn update_clients(&self) {
        let clients = self.clients_clone().await;
        let mut to_be_removed = Vec::new();
        let proto_message = proto::PlayerState {
            line_stripe: self.line_stripe_proto().await,
        };
        for (name, client) in clients.iter() {
            tracing::debug!("Send state to client");
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

/// Invariant:
/// length of points must be greater 2
/// lines are always vertical or horizontal
/// p[i].x - p[i-1].x == 0 || p[i].y - p[i-1].y == 0
#[derive(Debug)]
struct LineStripe {
    points: Vec<Point>,
}

impl LineStripe {
    fn new() -> LineStripe {
        Self::from(Point::new(10, 10), Point::new(15, 10))
    }

    fn from(start: Point, end: Point) -> Self {
        LineStripe {
            points: vec![start, end],
        }
    }

    fn line_stripe_proto(&self) -> Vec<proto::Point> {
        self.points
            .iter()
            .map(|p| proto::Point { x: p.x, y: p.y })
            .collect()
    }

    fn move_in_direction(&mut self, direction: Direction) {
        let len = self.points.len();
        let prev = self.points[len - 2];
        let current = self.points[len - 1];
        if direction == current.direction(&prev) {
            self.points[len - 1] = current.move_in_direction(direction);
        } else {
            self.points.push(current.move_in_direction(direction));
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

#[cfg(test)]
mod tests {
    use crate::snake::{Direction, LineStripe, Point};

    #[test]
    fn it_works() {
        let mut line_stripe = LineStripe::from(Point::new(10, 10), Point::new(10, 15));
        line_stripe.move_in_direction(Direction::Up);
        assert_eq!(Point::new(10, 10), line_stripe.points[0]);
        assert_eq!(Point::new(10, 16), line_stripe.points[1]);
        line_stripe.move_in_direction(Direction::Right);
        assert_eq!(Point::new(10, 10), line_stripe.points[0]);
        assert_eq!(Point::new(10, 16), line_stripe.points[1]);
        assert_eq!(Point::new(11, 16), line_stripe.points[2]);
    }
}
