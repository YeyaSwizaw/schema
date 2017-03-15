#[macro_use] extern crate glium;

use std::time::{Duration, Instant};
use std::thread::sleep;

use glium::{DisplayBuild};
use glium::glutin::{WindowBuilder, Event, ElementState, MouseScrollDelta, VirtualKeyCode};
use glium::backend::glutin_backend::GlutinFacade;

use model::Model;
use renderer::Renderer;
use values::*;

mod model;
mod view;
mod shaders;
mod values;
mod renderer;

struct App<'a> {
    display: GlutinFacade,
    renderer: Renderer<'a>,
    values: DisplayValues,
    mouse: DisplayCoord,

    model: Model
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        let display = WindowBuilder::new()
            .with_title("Schema Designer")
            .with_dimensions(600, 600)
            .with_decorations(false)
            .with_vsync()
            .build_glium()
            .unwrap();

        let values = DisplayValues::new(display.get_window().unwrap().get_inner_size_pixels().unwrap());
        let renderer = Renderer::new(&display);

        App {
            display: display,
            renderer: renderer,
            values: values,
            mouse: DisplayCoord(0, 0),

            model: Model::new()
        }
    }

    fn main_loop(&mut self) -> Option<()> {
        for event in self.display.poll_events() {
            match event {
                Event::Resized(w, h) => self.values.size = (w, h),

                Event::MouseMoved(x, y) => self.mouse = DisplayCoord(x, y),

                Event::MouseWheel(d, _) => {
                    match d {
                        MouseScrollDelta::LineDelta(_, _) => unimplemented!(),
                        MouseScrollDelta::PixelDelta(_, yd) => {
                            let yd = std::cmp::max(std::cmp::min(yd as i32, 200), -200);
                            self.values.scale += yd as f32 / 200.0;
                        }
                    }
                },

                Event::MouseInput(ElementState::Pressed, _) => {
                    self.model.add_table("", self.values.world_coord(self.mouse));
                },

                Event::KeyboardInput(ElementState::Pressed, _, Some(code)) => {
                    match code {
                        VirtualKeyCode::Up | VirtualKeyCode::W => self.values.offset.1 -= 20,
                        VirtualKeyCode::Down | VirtualKeyCode::S => self.values.offset.1 += 20,
                        VirtualKeyCode::Left | VirtualKeyCode::A => self.values.offset.0 -= 20,
                        VirtualKeyCode::Right | VirtualKeyCode::D => self.values.offset.0 += 20,
                        _ => ()
                    }
                },

                Event::Closed => return None,

                _ => ()
            }
        }

        let mut target = self.display.draw();
        self.renderer.render_tables(&mut target, &self.values, &self.model.view);
        target.finish().unwrap();

        Some(())
    }

    fn run(&mut self) {
        let mut accum = Duration::from_millis(0);
        let mut prev = Instant::now();

        let spf = Duration::from_millis(33);

        while let Some(()) = self.main_loop() {
            let now = Instant::now();
            accum += now - prev;
            prev = now;

            while accum >= spf {
                accum -= spf;
            }

            sleep(spf - accum);
        }
    }
}

fn main() {
    App::new().run()
}
