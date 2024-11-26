use crossterm::event::KeyEvent;
use ratatui::Frame;
use tokio::sync::mpsc::UnboundedSender;

use super::components::{
    component::{Component, ComponentRender, RenderProps},
    main_page::MainPage,
};
use crate::state_handler::{action::Action, state::State};

pub struct AppRouter {
    main_page: MainPage,
}

impl Component for AppRouter {
    fn new(state: &State, action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        AppRouter {
            main_page: MainPage::new(state, action_tx),
        }
    }

    fn update(self, state: &State) -> Self
    where
        Self: Sized,
    {
        AppRouter {
            main_page: self.main_page.update(state),
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        self.main_page.handle_key_event(key);
    }
}

impl ComponentRender<RenderProps> for AppRouter {
    fn render(&mut self, frame: &mut Frame, props: RenderProps) {
        self.main_page.render(
            frame,
            RenderProps {
                area: props.area,
                options: props.options,
            },
        );
    }
}
