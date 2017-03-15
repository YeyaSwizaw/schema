use glium::backend::Facade;
use glium::program::Program;

static TABLE_VERTEX: &'static str = include_str!("table_vertex.glsl");
static TABLE_FRAGMENT: &'static str = include_str!("table_fragment.glsl");

pub fn table_shader<Display: Facade>(display: &Display) -> Program {
    Program::from_source(display, TABLE_VERTEX, TABLE_FRAGMENT, None).unwrap()
}
