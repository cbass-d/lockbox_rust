use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, List, ListDirection, ListItem, ListState, Paragraph},
    Frame,
};
use tokio::sync::mpsc::UnboundedSender;

use super::component::{Component, ComponentRender, RenderProps};
use crate::state_handler::{
    action::Action,
    state::{Options, State},
};

pub struct ActionBox {
    action_tx: UnboundedSender<Action>,
    options_list: OptionsList,
}

struct OptionsList {
    options: Vec<String>,
    list_state: ListState,
}

impl Default for OptionsList {
    fn default() -> Self {
        Self {
            options: vec![
                "1 - Enrcypt File".to_owned(),
                "2 - Decrypt File".to_owned(),
                "3 - Manage Keys".to_owned(),
            ],
            list_state: ListState::default(),
        }
    }
}

impl Component for ActionBox {
    fn new(_state: &State, action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        Self {
            action_tx,
            options_list: OptionsList::default(),
        }
    }

    fn update(self, _state: &State) -> Self
    where
        Self: Sized,
    {
        Self {
            action_tx: self.action_tx,
            options_list: OptionsList::default(),
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => {
                let _ = self.action_tx.send(Action::Quit);
            }
            KeyCode::Up => {
                self.options_list.list_state.select_previous();
            }
            KeyCode::Down => {
                self.options_list.list_state.select_next();
            }
            _ => {}
        }
    }
}

impl ComponentRender<RenderProps> for ActionBox {
    fn render(&mut self, frame: &mut Frame, props: RenderProps) {
        let options = List::new(self.options_list.options.clone())
            .block(Block::default().borders(Borders::NONE))
            .fg(Color::Magenta)
            .highlight_style(Style::new().reversed())
            .highlight_symbol("*");

        frame.render_stateful_widget(options, props.area, &mut self.options_list.list_state);
    }
}
