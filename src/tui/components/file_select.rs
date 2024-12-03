use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Padding},
    Frame,
};
use tokio::sync::mpsc::UnboundedSender;

use super::{
    component::{Component, ComponentRender, RenderProps},
    header::Header,
    input_box::InputBox,
    popup_area,
};
use crate::state_handler::{action::Action, state::State};

pub struct FileSelect {
    header: Header,
    action_tx: UnboundedSender<Action>,
    input_box: InputBox,
}

impl Component for FileSelect {
    fn new(state: &State, action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        FileSelect {
            header: Header::new(state, action_tx.clone()),
            input_box: InputBox::new(state, action_tx.clone()),
            action_tx,
        }
    }

    fn update(self, state: &State) -> Self
    where
        Self: Sized,
    {
        Self {
            header: self.header.update(state),
            input_box: self.input_box.update(state),
            action_tx: self.action_tx,
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        self.input_box.handle_key_event(key);
    }
}

impl ComponentRender<RenderProps> for FileSelect {
    fn render(&mut self, frame: &mut Frame, props: RenderProps) {
        let popup_block = Block::default()
            .title("Select file")
            .borders(Borders::ALL)
            .style(Style::default());

        let area = popup_area(props.area, 70, 55);
        frame.render_widget(popup_block, area);

        let constraints = Constraint::from_percentages([10, 10, 80]);
        let layout = Layout::default().constraints(constraints).split(area);

        self.input_box
            .render(frame, RenderProps { area: layout[1] });
    }
}

