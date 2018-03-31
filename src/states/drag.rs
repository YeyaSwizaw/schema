use stateloop::{
    state::Action,
    app::{Data, Event},
    winit::ElementState,
};

use glium::Display;

use super::DragTableHandler;

use ::Stuff;
use ::states::State;
use ::values::WorldCoord;
use ::view::Index;

impl<'a> DragTableHandler for Data<Stuff<'a>, Display> {
    fn handle_event(&mut self, event: Event, _: Index, _: (i32, i32)) -> Action<State> {
        match event {
            Event::MouseInput{state: ElementState::Released, ..} => Action::Done(State::Main()),

            _ => self.data.default_action(event)
        }
    }

    fn handle_tick(&mut self, index: Index, offset: (i32, i32)) {
        let stuff = &mut self.data;

        stuff.check_scroll();

        let table = stuff.model.view.get_table_mut(index).unwrap();
        let coord = stuff.display_values.world_coord(stuff.input_values.mouse);

        let pos = WorldCoord(coord.0 - offset.0, coord.1 - offset.1);
        table.pos = pos;
    }

    fn handle_render(&self, _: Index, _: (i32, i32)) {
        self.data.render_frame(self.window());
    }
}
