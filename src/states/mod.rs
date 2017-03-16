use glium::glutin::Event;

use ::App;
use ::values::WorldCoord;
use ::view::Index;

use self::main::MainHandler;
use self::add::AddTableHandler;
use self::drag::DragTableHandler;

mod main;
mod add;
mod drag;

#[derive(Copy, Clone)]
pub enum State {
    Main,
    AddTable(Index, WorldCoord),
    DragTable(Index, (i32, i32)),
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
            State::DragTable(index, offset) => DragTableHandler::handle_event(app, event, index, offset),
            State::Quit => Action::Quit,
        }
    }

    pub fn handle_tick(self, app: &mut App) {
        match self {
            State::Main => MainHandler::handle_tick(app),
            State::AddTable(index, start) => AddTableHandler::handle_tick(app, index, start),
            State::DragTable(index, offset) => DragTableHandler::handle_tick(app, index, offset),
            State::Quit => ()
        }
    }
}
