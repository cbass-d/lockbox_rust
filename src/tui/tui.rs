use color_eyre::eyre::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use futures::{FutureExt, StreamExt};
use ratatui::prelude::*;
use std::io::{self, Stdout};
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};

use crate::state_handler::action::Action;

pub struct Tui {
    pub action_tx: UnboundedSender<Action>,
}

pub enum Event {
    Error,
    Tick,
    Key(KeyEvent),
}

pub struct EventHandler {
    _tx: UnboundedSender<Event>,
    rx: UnboundedReceiver<Event>,
    task: Option<JoinHandle<()>>,
}

impl EventHandler {
    pub fn new() -> Self {
        let tick_rate = std::time::Duration::from_millis(250);

        let (tx, rx) = mpsc::unbounded_channel();
        let _tx = tx.clone();

        let task = tokio::spawn(async move {
            let mut reader = crossterm::event::EventStream::new();
            let mut interval = tokio::time::interval(tick_rate);
            loop {
                let delay = interval.tick();
                let crossterm_event = reader.next().fuse();
                tokio::select! {
                    maybe_event = crossterm_event => match maybe_event {
                        Some(Ok(event)) => {
                            match event {
                                crossterm::event::Event::Key(key) => {
                                    if key.kind == crossterm::event::KeyEventKind::Press {
                                        tx.send(Event::Key(key)).unwrap();
                                    }
                                }
                                _ => {},
                            }
                        }
                        Some(Err(_)) => {
                            tx.send(Event::Error).unwrap();
                        }
                        None => {},
                    },
                    _ = delay => {
                        tx.send(Event::Tick).unwrap();
                    }
                }
            }
        });

        Self {
            _tx,
            rx,
            task: Some(task),
        }
    }

    pub async fn next(&mut self) -> Result<Event> {
        self.rx
            .recv()
            .await
            .ok_or(color_eyre::eyre::eyre!("Unable to get event"))
    }
}

impl Tui {
    pub fn new() -> (Self, UnboundedReceiver<Action>, EventHandler) {
        let (action_tx, action_rx) = mpsc::unbounded_channel::<Action>();
        let event_handler = EventHandler::new();

        (Self { action_tx }, action_rx, event_handler)
    }

    pub fn setup_terminal() -> Terminal<CrosstermBackend<Stdout>> {
        let mut stdout = io::stdout();
        enable_raw_mode().unwrap();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
        Terminal::new(CrosstermBackend::new(stdout)).unwrap()
    }

    pub fn teardown_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
        disable_raw_mode().unwrap();
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .unwrap();

        terminal.show_cursor().unwrap();
    }
}
