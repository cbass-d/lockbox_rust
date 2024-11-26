use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Constraint, Layout},
    Frame,
};
use tokio::sync::mpsc::UnboundedSender;

use super::{
    action_box::ActionBox,
    component::{Component, ComponentRender, RenderProps},
    header::Header,
};
use crate::state_handler::{action::Action, state::State};

pub struct MainPage {
    header: Header,
    action_box: ActionBox,
}

impl Component for MainPage {
    fn new(state: &State, action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        MainPage {
            header: Header::new(state, action_tx.clone()),
            action_box: ActionBox::new(state, action_tx),
        }
    }

    fn update(self, state: &State) -> Self
    where
        Self: Sized,
    {
        Self {
            header: self.header.update(state),
            action_box: self.action_box.update(state),
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        self.action_box.handle_key_event(key);
    }
}

impl ComponentRender<RenderProps> for MainPage {
    fn render(&mut self, frame: &mut Frame, props: RenderProps) {
        let constraints = Constraint::from_percentages([20, 80]);
        let layout = Layout::default().constraints(constraints).split(props.area);

        self.header.render(
            frame,
            RenderProps {
                area: layout[0],
                options: props.options.clone(),
            },
        );
        self.action_box.render(
            frame,
            RenderProps {
                area: layout[1],
                options: props.options,
            },
        );
    }
}
