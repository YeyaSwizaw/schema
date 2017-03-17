use glium::backend::Facade;
use glium::program::Program;

static TABLE_VERTEX: &'static str = include_str!("table_vertex.glsl");
static TABLE_FRAGMENT: &'static str = include_str!("table_fragment.glsl");

static FOCUS_VERTEX: &'static str = include_str!("focus_vertex.glsl");
static FOCUS_GEOMETRY: &'static str = include_str!("focus_geometry.glsl");
static FOCUS_FRAGMENT: &'static str = include_str!("focus_fragment.glsl");

pub fn table_shader<Display: Facade>(display: &Display) -> Program {
    Program::from_source(display, TABLE_VERTEX, TABLE_FRAGMENT, None).unwrap()
}

pub fn focus_shader<Display: Facade>(display: &Display) -> Program {
    Program::from_source(display, FOCUS_VERTEX, FOCUS_FRAGMENT, Some(FOCUS_GEOMETRY)).unwrap()
}
