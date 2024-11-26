#[derive(Clone)]
pub enum Options {
    MainMenu,
    ManageKeyRing,
}

#[derive(Clone)]
pub struct State {
    pub options: Options,
    pub exit: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            options: Options::MainMenu,
            exit: false,
        }
    }
}

impl State {
    pub fn exit(&mut self) {
        self.exit = true;
    }
}
