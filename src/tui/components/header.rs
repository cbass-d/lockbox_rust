use crossterm::event::KeyEvent;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Style, Stylize},
    text::Text,
    widgets::{Block, Borders, Padding, Paragraph},
    Frame,
};
use tokio::sync::mpsc::UnboundedSender;

use super::component::{Component, ComponentRender, RenderProps};
use crate::state_handler::{action::Action, state::State};

const TITLE: &str = r###"
 _     _____ _____  _   ________  _______   __  _____  _     _____
| |   |  _  /  __ \| | / /| ___ \|  _  \ \ / / /  __ \| |   |_   _|
| |   | | | | /  \/| |/ / | |_/ /| | | |\ V /  | /  \/| |     | |  
| |   | | | | |    |    \ | ___ \| | | |/   \  | |    | |     | |  
| |___\ \_/ / \__/\| |\  \| |_/ /\ \_/ / /^\ \ | \__/\| |_____| |_ 
\_____/\___/ \____/\_| \_/\____/  \___/\/   \/  \____/\_____/\___/
"###;

pub struct Header {}

impl Component for Header {
    fn new(state: &State, action_tx: UnboundedSender<Action>) -> Self
    where
        Self: Sized,
    {
        Self {}
    }

    fn update(self, state: &State) -> Self
    where
        Self: Sized,
    {
        Self {}
    }

    fn handle_key_event(&mut self, key: KeyEvent) {}
}

impl ComponentRender<RenderProps> for Header {
    fn render(&mut self, frame: &mut Frame, props: RenderProps) {
        let style = Style::new().light_magenta().bold();
        let banner = Text::from(TITLE).style(style);
        let header = Paragraph::new(banner)
            .block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .border_style(Style::new().magenta()),
            )
            .centered();

        frame.render_widget(header, props.area);
    }
}
