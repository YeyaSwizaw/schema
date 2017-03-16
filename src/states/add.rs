use glium::glutin::{Event, ElementState};

use ::App;
use ::states::{State, Action};
use ::values::WorldCoord;
use ::view::Index;

pub trait AddTableHandler {
    fn handle_event(&mut self, event: Event, index: Index, start: WorldCoord) -> Action;
    fn handle_tick(&mut self, index: Index, start: WorldCoord);
}

impl<'a> AddTableHandler for App<'a> {
    fn handle_event(&mut self, event: Event, _: Index, _: WorldCoord) -> Action {
        match event {
            Event::MouseInput(ElementState::Released, _) => Action::Done(State::Main),

            _ => Action::Default
        }
    }

    fn handle_tick(&mut self, index: Index, start: WorldCoord) {
        self.check_scroll();

        let table = self.model.view.get_table_mut(index).unwrap();
        let coord = self.display_values.world_coord(self.input_values.mouse);

        let size = (coord.0 - start.0, coord.1 - start.1);
        table.size = size;
    }
}
