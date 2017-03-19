use glium::glutin::{Event, ElementState};
use stateloop::state::Action;
use stateloop::app::App;

use super::AddTableHandler;

use ::Stuff;
use ::states::State;
use ::values::WorldCoord;
use ::view::Index;

impl<'a> AddTableHandler for App<Stuff<'a>> {
    fn handle_event(&mut self, event: Event, _: Index, _: WorldCoord) -> Action<State> {
        match event {
            Event::MouseInput(ElementState::Released, _) => Action::Done(State::Main()),

            _ => self.data_mut().default_action(event)
        }
    }

    fn handle_tick(&mut self, index: Index, start: WorldCoord) {
        let stuff = self.data_mut();

        stuff.check_scroll();

        let table = stuff.model.view.get_table_mut(index).unwrap();
        let coord = stuff.display_values.world_coord(stuff.input_values.mouse);

        let size = (coord.0 - start.0, coord.1 - start.1);
        table.size = size;
    }

    fn handle_render(&mut self, _: Index, _: WorldCoord) {
        self.data().render_frame(self.display())
    }
}
