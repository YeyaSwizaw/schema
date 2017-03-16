use glium::glutin::{Event, ElementState};

use ::App;
use ::states::{State, Action};

pub trait MainHandler {
    fn handle_event(&mut self, event: Event) -> Action;
    fn handle_tick(&mut self);
}

impl<'a> MainHandler for App<'a> {
    fn handle_event(&mut self, event: Event) -> Action {
        match event {
            Event::MouseInput(ElementState::Pressed, _) => {
                let coord = self.display_values.world_coord(self.input_values.mouse);

                if let Some(index) = self.focus {
                    self.model.view.bring_to_front(index);
                    let table = self.model.view.get_table(index).unwrap();
                    Action::Done(State::DragTable(index, (coord.0 - table.pos.0, coord.1 - table.pos.1)))
                } else {
                    let index = self.model.add_table("", self.display_values.world_coord(self.input_values.mouse));
                    Action::Done(State::AddTable(index, coord))
                }

            },

            _ => Action::Default
        }
    }

    fn handle_tick(&mut self) {
        self.check_scroll();

        if self.input_values.moved {
            self.check_focus();
            self.input_values.moved = false;
        }
    }
}

