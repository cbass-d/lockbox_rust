use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, List, ListState},
    Frame,
};
use tokio::sync::mpsc::UnboundedSender;

use super::{
    component::{Component, ComponentRender, RenderProps},
    file_select::FileSelect,
    header::Header,
};
use crate::state_handler::{action::Action, state::State};

pub struct MainPage {
    header: Header,
    options_list: OptionsList,
    action_tx: UnboundedSender<Action>,
    file_select: FileSelect,
    popup: bool,
}

struct OptionsList {
    options: Vec<String>,
    state: ListState,
}

impl Default for OptionsList {
    fn default() -> Self {
        Self {
            options: vec![
                "1 - Enrcypt File".to_owned(),
                "2 - Decrypt File".to_owned(),
                "3 - Manage Keys".to_owned(),
            ],
            state: ListState::default(),
        }
    }
}

impl Component for MainPage {
    fn new(state: &State, action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        MainPage {
            header: Header::new(state, action_tx.clone()),
            options_list: OptionsList::default(),
            file_select: FileSelect::new(state, action_tx.clone()),
            action_tx,
            popup: false,
        }
    }

    fn update(self, state: &State) -> Self
    where
        Self: Sized,
    {
        Self {
            header: self.header.update(state),
            file_select: self.file_select.update(state),
            action_tx: self.action_tx,
            options_list: self.options_list,
            popup: state.show_file_popup,
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        if self.popup {
            self.file_select.handle_key_event(key);
        } else {
            match key.code {
                KeyCode::Char('q') => {
                    let _ = self.action_tx.send(Action::Quit);
                }
                KeyCode::Up => {
                    self.options_list.state.select_previous();
                }
                KeyCode::Down => {
                    self.options_list.state.select_next();
                }
                KeyCode::Enter => match self.options_list.state.selected() {
                    Some(0) => {
                        let _ = self.action_tx.send(Action::SelectFile).unwrap();
                    }
                    Some(1) => {}
                    Some(2) => {}
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

impl ComponentRender<RenderProps> for MainPage {
    fn render(&mut self, frame: &mut Frame, props: RenderProps) {
        let constraints = Constraint::from_percentages([20, 80]);
        let layout = Layout::default().constraints(constraints).split(props.area);
        let options = List::new(self.options_list.options.clone())
            .block(Block::default().borders(Borders::NONE))
            .fg(Color::Magenta)
            .highlight_style(Style::new().reversed())
            .highlight_symbol("*");

        self.header.render(frame, RenderProps { area: layout[0] });
        frame.render_stateful_widget(options, layout[1], &mut self.options_list.state);

        if self.popup {
            self.file_select
                .render(frame, RenderProps { area: frame.area() });
        }
    }
}
