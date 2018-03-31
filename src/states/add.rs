use stateloop::{
    state::Action,
    app::{Data, Event},
    winit::ElementState,
};

use glium::Display;

use super::AddTableHandler;

use ::Stuff;
use ::states::State;
use ::values::WorldCoord;
use ::view::Index;

impl<'a> AddTableHandler for Data<Stuff<'a>, Display> {
    fn handle_event(&mut self, event: Event, _: Index, _: WorldCoord) -> Action<State> {
        match event {
            Event::MouseInput{state: ElementState::Released, ..} => Action::Done(State::Main()),

            _ => self.data.default_action(event)
        }
    }

    fn handle_tick(&mut self, index: Index, start: WorldCoord) {
        let stuff = &mut self.data;

        stuff.check_scroll();

        let table = stuff.model.view.get_table_mut(index).unwrap();
        let coord = stuff.display_values.world_coord(stuff.input_values.mouse);

        let size = (coord.0 - start.0, coord.1 - start.1);
        table.size = size;
    }

    fn handle_render(&self, _: Index, _: WorldCoord) {
        self.data.render_frame(self.window());
    }
}
