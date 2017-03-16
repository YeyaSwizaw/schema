use glium::glutin::Event;

use ::App;
use ::values::WorldCoord;
use ::view::Index;

use self::main::MainHandler;
use self::add::AddTableHandler;

mod main;
mod add;

#[derive(Copy, Clone)]
pub enum State {
    Main,
    AddTable(Index, WorldCoord),
    Quit,
}

#[derive(Copy, Clone)]
pub enum Action {
    Default,
    Continue,
    Done(State),
    Quit
}

impl State {
    pub fn handle_event(self, app: &mut App, event: Event) -> Action {
        match self {
            State::Main => MainHandler::handle_event(app, event),
            State::AddTable(index, start) => AddTableHandler::handle_event(app, event, index, start),
            State::Quit => Action::Quit,
        }
    }
}
