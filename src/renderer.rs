use glium::{Surface, Blend};
use glium::program::Program;
use glium::backend::Facade;
use glium::index::{NoIndices, PrimitiveType};
use glium::vertex::VertexBuffer;
use glium::draw_parameters::DrawParameters;

use ::shaders;
use ::view::View;
use ::values::*;

pub struct Renderer<'a> {
    table_program: Program,
    table_indices: NoIndices,
    table_vertices: VertexBuffer<TableVertex>,

    draw_params: DrawParameters<'a>,
}

impl<'a> Renderer<'a> {
    pub fn new<'b, Display: Facade>(display: &'b Display) -> Renderer<'a> {
        Renderer {
            table_program: shaders::table_shader(display),
            table_indices: NoIndices(PrimitiveType::TriangleFan),
            table_vertices: VertexBuffer::new(display, &[tv(0, 0), tv(0, 1), tv(1, 1), tv(1, 0)]).unwrap(),

            draw_params: DrawParameters {
                blend: Blend::alpha_blending(),
                ..Default::default()
            }
        }
    }

    pub fn render_tables<Target: Surface>(&self, target: &mut Target, values: &DisplayValues, view: &View) {
        target.clear_color(0.3, 0.3, 0.3, 1.0);

        for table in view.tables() {
            let uniforms = uniform! {
                position: table.pos,
                size: table.size,
                off: values.offset,
                display: values.size,
                scale: values.scale,
            };

            target.draw(&self.table_vertices, &self.table_indices, &self.table_program, &uniforms, &self.draw_params).unwrap();
        }
    }
}

