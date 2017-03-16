use glium::glutin::{Event, ElementState};

use ::App;
use ::states::{State, Action};
use ::values::{DisplayCoord, WorldCoord};
use ::view::Index;

pub trait AddTableHandler {
    fn handle_event(&mut self, event: Event, index: Index, start: WorldCoord) -> Action;
}

impl<'a> AddTableHandler for App<'a> {
    fn handle_event(&mut self, event: Event, index: Index, start: WorldCoord) -> Action {
        match event {
            Event::MouseMoved(x, y) => {
                let table = self.model.view.get_table_mut(index).unwrap();
                let coord = self.display_values.world_coord(DisplayCoord(x, y));

                let size = (coord.0 - start.0, coord.1 - start.1);
                table.pos = WorldCoord(start.0 + size.0 / 2, start.1 + size.1 / 2);
                table.size = size;
                Action::Default
            },

            Event::MouseInput(ElementState::Released, _) => Action::Done(State::Main),

            _ => Action::Default
        }
    }
}
