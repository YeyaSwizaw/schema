use glium::glutin::{Event, ElementState};

use ::App;
use ::states::{State, Action};
use ::values::WorldCoord;
use ::view::Index;

pub trait DragTableHandler {
    fn handle_event(&mut self, event: Event, index: Index, offset: (i32, i32)) -> Action;
    fn handle_tick(&mut self, index: Index, offset: (i32, i32));
}

impl<'a> DragTableHandler for App<'a> {
    fn handle_event(&mut self, event: Event, _: Index, _: (i32, i32)) -> Action {
        match event {
            Event::MouseInput(ElementState::Released, _) => Action::Done(State::Main),

            _ => Action::Default
        }
    }

    fn handle_tick(&mut self, index: Index, offset: (i32, i32)) {
        self.check_scroll();

        let table = self.model.view.get_table_mut(index).unwrap();
        let coord = self.display_values.world_coord(self.input_values.mouse);

        let pos = WorldCoord(coord.0 - offset.0, coord.1 - offset.1);
        table.pos = pos;
    }
}
