mod events;
mod proto;

use crate::proto::snake::snake_server_client::SnakeServerClient;
use crate::proto::snake::Login;
use tokio::io;
use tokio::io::{AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SnakeServerClient::connect("http://[::1]:50051").await?;
    println!("Enter Name");
    let user = read_next_line().await?;
    let request = tonic::Request::new(Login { user: user.clone() });
    let response = client.observe_game_state(request).await?.into_inner();
    ui::init_board(client.clone(), user.clone(), response)?;
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
    use crate::proto::snake::snake_server_client::SnakeServerClient;
    use crate::proto::snake::{PlayerMove, PlayerState};
    use prost::alloc::sync::Arc;
    use std::io::Stdout;
    use std::sync::Mutex;
    use std::{error::Error, time::Duration};
    use termion::raw::RawTerminal;
    use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
    use tonic::transport::Channel;
    use tonic::{Request, Streaming};
    use tui::widgets::canvas::{Context, Line};
    use tui::{
        backend::TermionBackend,
        layout::{Constraint, Layout},
        style::Color,
        widgets::{canvas::Canvas, Block, Borders},
        Frame, Terminal,
    };

    pub fn init_board(
        mut client: SnakeServerClient<Channel>,
        user: String,
        mut response: Streaming<PlayerState>,
    ) -> Result<(), Box<dyn Error>> {
        let mut terminal = new_terminal()?;
        let state = new_state();
        handle_messages(response, state.clone());
        // Setup event handlers
        let config = Config {
            tick_rate: Duration::from_millis(250),
            ..Default::default()
        };
        let events = Events::with_config(config);
        loop {
            terminal.draw(|f| draw(&state, f))?;
            let next = events.next()?;
            if handle_key_event(&mut client, &user, next)? == true {
                break;
            }
        }
        Ok(())
    }

    fn handle_messages(mut response: Streaming<PlayerState>, state: Arc<Mutex<PlayerState>>) {
        tokio::spawn(async move {
            while let Ok(Some(message)) = response.message().await {
                if let Ok(mut l) = state.lock() {
                    *l = message;
                }
            }
        });
    }

    fn new_state() -> Arc<Mutex<PlayerState>> {
        Arc::new(Mutex::new(PlayerState {
            line_stripe: vec![],
        }))
    }

    fn new_terminal() -> Result<
        Terminal<TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<Stdout>>>>>,
        Box<dyn Error>,
    > {
        let stdout = std::io::stdout().into_raw_mode()?;
        let stdout = MouseTerminal::from(stdout);
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        Ok(Terminal::new(backend)?)
    }

    type FrameType<'a> =
        Frame<'a, TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<Stdout>>>>>;

    fn draw(state2: &Arc<Mutex<PlayerState>>, f: &mut FrameType) {
        let chunks = Layout::default()
            .direction(tui::layout::Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(f.size());
        let canvas = Canvas::default()
            .block(Block::default().borders(Borders::ALL).title("Pong"))
            .paint(|ctx| paint_game(&state2, ctx))
            .x_bounds([10.0, 110.0])
            .y_bounds([10.0, 110.0]);
        f.render_widget(canvas, chunks[0]);
    }

    fn paint_game(state2: &Arc<Mutex<PlayerState>>, ctx: &mut Context) {
        if let Ok(snake) = state2.lock() {
            if snake.line_stripe.len() == 0 {
                return;
            }
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
        }
    }

    fn handle_key_event(
        client: &mut SnakeServerClient<Channel>,
        user: &String,
        next: Event<Key>,
    ) -> Result<bool, Box<dyn Error>> {
        match next {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    return Ok(true);
                }
                Key::Down => handle_down(client, user)?,
                Key::Up => {
                    handle_up(client, user)?;
                }
                Key::Right => {
                    handle_right(client, user)?;
                }
                Key::Left => {
                    handle_left(client, user)?;
                }
                _ => {}
            },
            Event::Tick => {}
        }
        return Ok(false);
    }

    fn handle_left(
        client: &mut SnakeServerClient<Channel>,
        user: &String,
    ) -> Result<(), Box<dyn Error>> {
        let r = Request::new(PlayerMove {
            direction: 1,
            name: user.clone(),
        });
        futures::executor::block_on(client.make_move(r))?;
        Ok(())
    }

    fn handle_right(
        client: &mut SnakeServerClient<Channel>,
        user: &String,
    ) -> Result<(), Box<dyn Error>> {
        let r = Request::new(PlayerMove {
            direction: 3,
            name: user.clone(),
        });
        futures::executor::block_on(client.make_move(r))?;
        Ok(())
    }

    fn handle_up(
        client: &mut SnakeServerClient<Channel>,
        user: &String,
    ) -> Result<(), Box<dyn Error>> {
        let r = Request::new(PlayerMove {
            direction: 0,
            name: user.clone(),
        });
        futures::executor::block_on(client.make_move(r))?;
        Ok(())
    }

    fn handle_down(
        client: &mut SnakeServerClient<Channel>,
        user: &String,
    ) -> Result<(), Box<dyn Error>> {
        let r = Request::new(PlayerMove {
            direction: 2,
            name: user.clone(),
        });
        futures::executor::block_on(client.make_move(r))?;
        Ok(())
    }
}
