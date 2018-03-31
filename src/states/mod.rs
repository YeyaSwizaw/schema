use ::values::WorldCoord;
use ::view::Index;

use stateloop::app::Event;

mod main;
mod add;
mod drag;

states! {
    State {
        MainHandler Main(),
        AddTableHandler AddTable(index: Index, start: WorldCoord),
        DragTableHandler DragTable(index: Index, offset: (i32, i32))
    }
}
