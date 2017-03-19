use glium::glutin::Event;

use ::values::WorldCoord;
use ::view::Index;

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
