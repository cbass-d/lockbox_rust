#[derive(Clone)]
pub enum Options {
    MainMenu,
    ManageKeyRing,
}

#[derive(Clone)]
pub enum CurrentPage {
    MainPage,
}

#[derive(Clone)]
pub struct State {
    pub options: Options,
    pub exit: bool,
    pub current_page: CurrentPage,
    pub show_file_popup: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            options: Options::MainMenu,
            exit: false,
            current_page: CurrentPage::MainPage,
            show_file_popup: false,
        }
    }
}

impl State {
    pub fn set_popup(&mut self) {
        self.show_file_popup = !self.show_file_popup;
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }
}
