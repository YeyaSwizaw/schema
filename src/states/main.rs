use stateloop::{
    state::Action,
    app::{Data, Event},
    winit::ElementState,
};

use super::MainHandler;

use glium::Display;

use ::Stuff;
use ::states::State;

impl<'a> MainHandler for Data<Stuff<'a>, Display> {
    fn handle_event(&mut self, event: Event) -> Action<State> {
        let stuff = &mut self.data;

        match event {
            Event::MouseInput{state: ElementState::Pressed, ..} => {
                let coord = stuff.display_values.world_coord(stuff.input_values.mouse);

                if let Some(ref mut index) = stuff.focus {
                    *index = stuff.model.view.bring_to_front(*index);
                    let table = stuff.model.view.get_table(*index).unwrap();
                    Action::Done(State::DragTable(*index, (coord.0 - table.pos.0, coord.1 - table.pos.1)))
                } else {
                    let index = stuff.model.add_table("", stuff.display_values.world_coord(stuff.input_values.mouse));
                    Action::Done(State::AddTable(index, coord))
                }

            },

            _ => stuff.default_action(event)
        }
    }

    fn handle_tick(&mut self) {
        let stuff = &mut self.data;

        stuff.check_scroll();

        if stuff.input_values.moved {
            stuff.check_focus();
            stuff.input_values.moved = false;
        }
    }

    fn handle_render(&self) {
        self.data.render_frame(self.window());
    }
}

