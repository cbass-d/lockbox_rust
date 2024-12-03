use std::time::Duration;
use tokio::{
    io::Result,
    sync::{
        broadcast::{self, Receiver, Sender},
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    },
};

use state_handler::{
    action::Action,
    state::{CurrentPage, State},
    StateHandler,
};
use tui::{
    app_router::AppRouter,
    components::component::{Component, ComponentRender, RenderProps},
    Event, Tui,
};

mod state_handler;
mod tui;

#[derive(Clone, Debug)]
enum Terminate {
    StateExit,
}

async fn run(shutdown_rx: &mut Receiver<Terminate>, shutdown_tx: Sender<Terminate>) {
    let (tui_handler, mut action_rx, mut event_handler) = Tui::new();
    let (state_handler, mut state_rx) = StateHandler::new();
    let mut shutdown_rx_tui = shutdown_rx.resubscribe();
    let mut shutdown_rx_state = shutdown_rx.resubscribe();

    let state_task = tokio::spawn(async move {
        let mut state = State::default();
        state_handler.state_tx.send(state.clone()).unwrap();

        let mut ticker = tokio::time::interval(Duration::from_millis(250));

        loop {
            if state.exit {
                let _ = shutdown_tx.send(Terminate::StateExit).unwrap();
            }

            tokio::select! {
                _tick = ticker.tick() => {},
                action = action_rx.recv() => {
                    match action.unwrap() {
                        Action::SelectFile => {
                            state.set_popup();
                        }
                        Action::ClosePopup => {
                            state.set_popup();
                        }
                        Action::Quit => {
                            state.exit();
                        },
                    }
                },
                _ = shutdown_rx_state.recv() => {
                    break;
                }
            }

            let _ = state_handler.state_tx.send(state.clone()).unwrap();
        }
    });

    let tui_task = tokio::spawn(async move {
        let mut terminal = Tui::setup_terminal();
        let state = state_rx.recv().await.unwrap();
        let mut app_router = AppRouter::new(&state, tui_handler.action_tx);

        // Iniital render
        let _ = terminal.draw(|f| app_router.render(f, RenderProps { area: f.area() }));

        let mut ticker = tokio::time::interval(Duration::from_millis(250));

        loop {
            tokio::select! {
                _tick = ticker.tick() => {},
                event = event_handler.next() => {
                    match event.unwrap()  {
                        Event::Key(key) => {
                            app_router.handle_key_event(key);
                        },
                        Event::Tick => {},
                        Event::Error => {},
                    }
                },
                state = state_rx.recv() => {
                    match state {
                            Some(state) => {
                                app_router = app_router.update(&state);
                            },
                            None => {},
                    }
                },
                _ = shutdown_rx_tui.recv() => {
                    break;
                }
            }

            let _ = terminal.draw(|f| app_router.render(f, RenderProps { area: f.area() }));
        }

        Tui::teardown_terminal(&mut terminal);
    });

    let (_, _) = tokio::join!(state_task, tui_task);
}

fn shutdown() {
    println!("Shutting down lockbox");
}

#[tokio::main]
async fn main() -> Result<()> {
    let (shutdown_tx, mut shutdown_rx) = broadcast::channel::<Terminate>(1);
    let mut shutdown_rx_main = shutdown_rx.resubscribe();

    tokio::spawn(async move {
        let _ = run(&mut shutdown_rx, shutdown_tx).await;
    })
    .await?;

    tokio::select! {
        _ = shutdown_rx_main.recv() => {
            shutdown();
        }
    }

    Ok(())
}
