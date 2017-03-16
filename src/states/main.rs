use glium::glutin::{Event, ElementState};

use ::App;
use ::states::{State, Action};

pub trait MainHandler {
    fn handle_event(&mut self, event: Event) -> Action;
}

impl<'a> MainHandler for App<'a> {
    fn handle_event(&mut self, event: Event) -> Action {
        match event {
            Event::MouseInput(ElementState::Pressed, _) => {
                let coord = self.display_values.world_coord(self.input_values.mouse);
                let index = self.model.add_table("", self.display_values.world_coord(self.input_values.mouse));
                Action::Done(State::AddTable(index, coord))
            },

            _ => Action::Default
        }
    }
}

