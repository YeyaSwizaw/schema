#![feature(box_syntax)]

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
    display_values: DisplayValues,
    input_values: InputValues,

    model: Model
}

#[derive(Copy, Clone)]
enum State {
    Main
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
        renderer.update_display(&values);

        App {
            display: display,
            renderer: renderer,
            display_values: values,
            input_values: InputValues::new(),

            model: Model::new()
        }
    }

    fn handle_main(&mut self) -> Option<State> {
        for event in self.display.poll_events() {
            match event {
                Event::Resized(w, h) => {
                    self.display_values.size = (w, h);
                    self.renderer.update_display(&self.display_values);
                }

                Event::MouseMoved(x, y) => self.input_values.mouse = DisplayCoord(x, y),

                Event::MouseWheel(d, _) => {
                    match d {
                        MouseScrollDelta::LineDelta(_, _) => unimplemented!(),
                        MouseScrollDelta::PixelDelta(_, yd) => {
                            let yd = std::cmp::max(std::cmp::min(yd as i32, 200), -200);
                            self.display_values.scale += yd as f32 / 200.0;
                            self.renderer.update_display(&self.display_values);
                        }
                    }
                },

                Event::MouseInput(ElementState::Pressed, _) => {
                    self.model.add_table("", self.display_values.world_coord(self.input_values.mouse));
                },

                Event::KeyboardInput(ElementState::Pressed, _, Some(code)) => {
                    match code {
                        VirtualKeyCode::Up | VirtualKeyCode::W => self.input_values.up = true,
                        VirtualKeyCode::Down | VirtualKeyCode::S => self.input_values.down = true,
                        VirtualKeyCode::Left | VirtualKeyCode::A => self.input_values.left = true,
                        VirtualKeyCode::Right | VirtualKeyCode::D => self.input_values.right = true,
                        _ => ()
                    }
                },

                Event::KeyboardInput(ElementState::Released, _, Some(code)) => {
                    match code {
                        VirtualKeyCode::Up | VirtualKeyCode::W => self.input_values.up = false,
                        VirtualKeyCode::Down | VirtualKeyCode::S => self.input_values.down = false,
                        VirtualKeyCode::Left | VirtualKeyCode::A => self.input_values.left = false,
                        VirtualKeyCode::Right | VirtualKeyCode::D => self.input_values.right = false,
                        _ => ()
                    }
                },

                Event::Closed => return None,

                _ => ()
            }
        }

        Some(State::Main)
    }

    fn handle_state(&mut self, state: State) -> Option<State> {
        match state {
            State::Main => self.handle_main()
        }
    }

    fn tick(&mut self) {
        let mut moved = false;

        if self.input_values.up {
            self.display_values.offset.1 -= 10;
            moved = true;
        }

        if self.input_values.down {
            self.display_values.offset.1 += 10;
            moved = true;
        }

        if self.input_values.left {
            self.display_values.offset.0 -= 10;
            moved = true;
        }

        if self.input_values.right {
            self.display_values.offset.0 += 10;
            moved = true;
        }

        if moved {
            self.renderer.update_display(&self.display_values);
        }
    }

    fn render_frame(&self) {
        let mut target = self.display.draw();
        self.renderer.render_tables(&mut target, &self.model.view);
        target.finish().unwrap();
    }

    fn run(&mut self) {
        let mut accum = Duration::from_millis(0);
        let mut prev = Instant::now();

        let spf = Duration::from_millis(33);

        let mut state = State::Main;
        while let Some(next) = self.handle_state(state) {
            state = next;
            self.render_frame();

            let now = Instant::now();
            accum += now - prev;
            prev = now;

            while accum >= spf {
                accum -= spf;

                self.tick();
            }

            sleep(spf - accum);
        }
    }
}

fn main() {
    App::new().run()
}
