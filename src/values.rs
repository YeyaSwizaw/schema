use glium::uniforms::{AsUniformValue, UniformValue};

#[derive(Debug, Copy, Clone)]
pub struct DisplayValues {
    pub size: (u32, u32),
    pub offset: (i32, i32),
    pub scale: f32,
}

implement_uniform_block!(DisplayValues, size, offset, scale);

#[derive(Default, Copy, Clone)]
pub struct InputValues {
    pub mouse: DisplayCoord,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool
}

#[derive(Default, Copy, Clone)]
pub struct DisplayCoord(pub i32, pub i32);

#[derive(Copy, Clone)]
pub struct WorldCoord(pub i32, pub i32);

impl DisplayValues {
    pub fn new(size: (u32, u32)) -> DisplayValues {
        DisplayValues {
            size: size,
            offset: (0, 0),
            scale: 1.0
        }
    }

    pub fn world_coord(&self, coord: DisplayCoord) -> WorldCoord {
        WorldCoord(
            (coord.0 as f32 / self.scale + self.offset.0 as f32) as i32, 
            (coord.1 as f32 / self.scale + self.offset.1 as f32) as i32
        )
    }
}

impl InputValues {
    pub fn new() -> InputValues {
        Default::default()
    }
}

impl AsUniformValue for WorldCoord {
    fn as_uniform_value(&self) -> UniformValue {
        UniformValue::IntVec2([self.0, self.1])
    }
}

#[derive(Copy, Clone)]
pub struct TableVertex {
    vertex: (u32, u32)
}

pub fn tv(x: u32, y: u32) -> TableVertex {
    TableVertex {
        vertex: (x, y)
    }
}

implement_vertex!(TableVertex, vertex);

