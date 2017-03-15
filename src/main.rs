#[macro_use] extern crate glium;

use glium::{Surface, DisplayBuild};
use glium::glutin::{WindowBuilder, Event, ElementState, MouseScrollDelta};
use glium::index::{NoIndices, PrimitiveType};
use glium::vertex::VertexBuffer;

use model::Model;

mod model;
mod view;
mod shaders;

#[derive(Copy, Clone)]
struct TableVertex {
    vertex: (u32, u32)
}

fn tv(x: u32, y: u32) -> TableVertex {
    TableVertex {
        vertex: (x, y)
    }
}

implement_vertex!(TableVertex, vertex);

fn main() {
    let display = WindowBuilder::new()
        .with_title("Schema Designer")
        .with_dimensions(600, 600)
        .with_decorations(false)
        .build_glium()
        .unwrap();

    let table_program = shaders::table_shader(&display);
    let table_indices = NoIndices(PrimitiveType::TriangleFan);
    let table_vertices = VertexBuffer::new(&display, &[tv(0, 0), tv(0, 1), tv(1, 1), tv(1, 0)]).unwrap();

    let mut display_size = display.get_window().unwrap().get_inner_size_pixels().unwrap();

    let mut mouse = (0.0, 0.0);
    let mut offset = (0.0f32, 0.0f32);
    let mut scale = 1.0;

    let mut model = Model::new();

    loop {
        for event in display.poll_events() {
            match event {
                Event::Resized(w, h) => display_size = (w, h),

                Event::MouseMoved(x, y) => mouse = (x as f32, y as f32),

                Event::MouseWheel(d, p) => {
                    match d {
                        MouseScrollDelta::LineDelta(_, _) => unimplemented!(),
                        MouseScrollDelta::PixelDelta(_, yd) => {
                            let yd = std::cmp::max(std::cmp::min(yd as i32, 200), -200);
                            scale += yd as f32 / 200.0;
                            println!("Scale: {}", scale);
                        }
                    }

                    println!("{:?}, {:?}", d, p);
                },

                Event::MouseInput(ElementState::Pressed, _) => {
                    model.add_table("memes", (mouse.0 / scale + offset.0, mouse.1 / scale + offset.1));
                },

                Event::KeyboardInput(ElementState::Pressed, _, _) => {
                    offset.1 += 10.0;
                },

                Event::Closed => return,

                _ => ()
            }
        }

        let mut target = display.draw();
        target.clear_color(0.3, 0.3, 0.3, 0.0);

        for table in model.view.tables() {
            target.draw(
                &table_vertices,
                table_indices,
                &table_program,
                &uniform! {
                    position: table.pos,
                    size: (100u32, 100u32),
                    off: offset,
                    display: display_size,
                    scale: scale,
                },
                &Default::default()
            ).unwrap();
        }

        target.finish().unwrap();
    }
}
