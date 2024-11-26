use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use super::state::State;

pub struct StateHandler {
    pub state_tx: UnboundedSender<State>,
}

impl StateHandler {
    pub fn new() -> (Self, UnboundedReceiver<State>) {
        let (state_tx, state_rx) = mpsc::unbounded_channel::<State>();

        (Self { state_tx }, state_rx)
    }
}
