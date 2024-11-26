use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, style::Color, Frame};
use tokio::sync::mpsc::UnboundedSender;

use crate::state_handler::{
    action::Action,
    state::{Options, State},
};

pub struct RenderProps {
    pub area: Rect,
    pub options: Options,
}

pub trait Component {
    fn new(state: &State, action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized;

    fn update(self, state: &State) -> Self
    where
        Self: Sized;

    fn handle_key_event(&mut self, key: KeyEvent);
}

pub trait ComponentRender<Props> {
    fn render(&mut self, frame: &mut Frame, props: Props);
}
