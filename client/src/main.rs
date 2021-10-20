mod events;
mod proto;

use crate::proto::snake::snake_server_client::SnakeServerClient;
use crate::proto::snake::{ChatMessage, Login};
use tokio::io;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ui::init_board()?;
    let mut client = SnakeServerClient::connect("http://[::1]:50051").await?;
    println!("Enter Name");
    let user = read_next_line().await?;
    let request = tonic::Request::new(Login { user: user.clone() });

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

mod ui {
    use crate::events::{Config, Event, Events};
    use std::thread::sleep;
    use std::{error::Error, io, time::Duration};
    use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
    use tui::widgets::canvas::Line;
    use tui::{
        backend::TermionBackend,
        layout::{Constraint, Layout, Rect},
        style::Color,
        widgets::{
            canvas::{Canvas, Map, MapResolution, Rectangle},
            Block, Borders,
        },
        Terminal,
    };

    struct Snake {
        line_stripe: Vec<Point>,
        direction: Direction,
    }

    impl Snake {
        fn new() -> Snake {
            Self {
                line_stripe: vec![Point::new(10, 10), Point::new(15, 10)],
                direction: Direction::Right,
            }
        }
        fn update(&mut self) {
            let point = self.line_stripe.last().expect("Never Zero length");
            if self.line_stripe.len() == 1 {
                self.line_stripe
                    .push(point.move_in_direction(self.direction));
            } else {
                let prev = &self.line_stripe[self.line_stripe.len() - 2];
                let can_update = self.direction == point.direction(prev);
                if can_update {
                    let len = self.line_stripe.len();
                    self.line_stripe[len - 1] = point.move_in_direction(self.direction);
                } else {
                    self.line_stripe
                        .push(point.move_in_direction(self.direction));
                }
            }
        }
    }

    #[derive(Copy, Clone, Eq, PartialEq)]
    enum Direction {
        Left,
        Right,
        Up,
        Down,
    }

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

    pub fn init_board() -> Result<(), Box<dyn Error>> {
        let stdout = std::io::stdout().into_raw_mode()?;
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Setup event handlers
        let config = Config {
            tick_rate: Duration::from_millis(250),
            ..Default::default()
        };
        let events = Events::with_config(config);
        let mut snake = Snake::new();
        loop {
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(tui::layout::Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(f.size());
                let canvas = Canvas::default()
                    .block(Block::default().borders(Borders::ALL).title("Pong"))
                    .paint(|ctx| {
                        let mut prev = snake.line_stripe.first().expect("");
                        for current in snake.line_stripe.iter().skip(1) {
                            ctx.draw(&Line {
                                x1: prev.x as f64,
                                y1: prev.y as f64,
                                x2: current.x as f64,
                                y2: current.y as f64,
                                color: Color::Yellow,
                            });
                            prev = current;
                        }
                    })
                    .x_bounds([10.0, 110.0])
                    .y_bounds([10.0, 110.0]);
                let area = Rect::new(0, 0, 500, 500);
                f.render_widget(canvas, chunks[0]);
                /*let canvas = Canvas::default()
                    .block(Block::default().borders(Borders::ALL).title("Pong"))
                    .paint(|ctx| {
                        ctx.draw(&app.ball);
                    })
                    .x_bounds([10.0, 110.0])
                    .y_bounds([10.0, 110.0]);
                f.render_widget(canvas, chunks[1]);*/
            })?;

            match events.next()? {
                Event::Input(input) => match input {
                    Key::Char('q') => {
                        break;
                    }
                    Key::Down => {
                        snake.direction = Direction::Down;
                    }
                    Key::Up => {
                        snake.direction = Direction::Up;
                    }
                    Key::Right => {
                        snake.direction = Direction::Right;
                    }
                    Key::Left => {
                        snake.direction = Direction::Left;
                    }

                    _ => {}
                },
                Event::Tick => {
                    snake.update();
                }
                _ => {}
            }
        }
        Ok(())
    }
}
