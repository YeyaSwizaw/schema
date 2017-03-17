use glium::{Surface, Blend, Depth, DepthTest};
use glium::program::Program;
use glium::backend::Facade;
use glium::index::{NoIndices, PrimitiveType};
use glium::vertex::VertexBuffer;
use glium::uniforms::UniformBuffer;
use glium::draw_parameters::DrawParameters;

use ::shaders;
use ::view::View;
use ::values::*;

#[derive(Copy, Clone)]
struct DisplayUniforms {
    display_block: DisplayValues
}

fn du(values: DisplayValues) -> DisplayUniforms {
    DisplayUniforms {
        display_block: values
    }
}

implement_uniform_block!(DisplayUniforms, display_block);

pub struct Renderer<'a> {
    display_uniforms: UniformBuffer<DisplayUniforms>,

    table_program: Program,
    table_indices: NoIndices,
    table_vertices: VertexBuffer<TableVertex>,

    draw_params: DrawParameters<'a>,
}

impl<'a> Renderer<'a> {
    pub fn new<'b, Display: Facade>(display: &'b Display) -> Renderer<'a> {
        Renderer {
            display_uniforms: UniformBuffer::empty_dynamic(display).unwrap(),

            table_program: shaders::table_shader(display),
            table_indices: NoIndices(PrimitiveType::TriangleFan),
            table_vertices: VertexBuffer::new(display, &[tv(0, 0), tv(0, 1), tv(1, 1), tv(1, 0)]).unwrap(),

            draw_params: DrawParameters {
                blend: Blend::alpha_blending(),
                depth: Depth {
                    test: DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                ..Default::default()
            }
        }
    }

    pub fn update_display(&self, values: &DisplayValues) {
        self.display_uniforms.write(&du(*values))
    }

    pub fn render_tables<Target: Surface>(&self, target: &mut Target, view: &View) {
        for table in view.tables() {
            let uniforms = uniform! {
                position: table.pos,
                size: table.size,
                z: table.z,
                inner_colour: table.inner_colour,
                outer_colour: table.outer_colour,
                display_block: &self.display_uniforms,
            };

            target.draw(&self.table_vertices, &self.table_indices, &self.table_program, &uniforms, &self.draw_params).unwrap();
        }
    }
}

