#![feature(box_syntax)]

#[macro_use] extern crate glium;

use std::time::{Duration, Instant};
use std::thread::sleep;
use std::cmp;

use glium::{DisplayBuild, Surface};
use glium::glutin::{WindowBuilder, Event, ElementState, MouseScrollDelta, VirtualKeyCode};
use glium::backend::glutin_backend::GlutinFacade;

use model::Model;
use view::Index;
use renderer::Renderer;
use values::*;
use states::{State, Action};

mod model;
mod view;
mod shaders;
mod values;
mod renderer;
mod states;

pub struct App<'a> {
    display: GlutinFacade,
    renderer: Renderer<'a>,

    display_values: DisplayValues,
    input_values: InputValues,
    focus: Option<Index>,

    model: Model
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        let display = WindowBuilder::new()
            .with_title("Schema Designer")
            .with_dimensions(600, 600)
            .with_decorations(false)
            .with_depth_buffer(24)
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
            focus: None,

            model: Model::new()
        }
    }

    fn default_actions(&mut self, event: Event) -> Option<State> {
        match event {
            Event::Resized(w, h) => {
                self.display_values.size = (w, h);
                self.renderer.update_display(&self.display_values);
                None
            },

            Event::MouseMoved(x, y) => {
                self.input_values.mouse = DisplayCoord(x, y);
                self.input_values.moved = true;
                None
            },

            Event::MouseWheel(d, _) => {
                match d {
                    MouseScrollDelta::LineDelta(_, _) => unimplemented!(),
                    MouseScrollDelta::PixelDelta(_, yd) => {
                        let sd = cmp::max(cmp::min(yd as i32, 200), -200) as f32 / -200.0;

                        let a = if self.display_values.scale + sd > 0.1 {
                            let a = self.display_values.world_coord(self.input_values.mouse);
                            self.display_values.scale += sd;
                            a
                        } else {
                            return None
                        };

                        let b = self.display_values.world_coord(self.input_values.mouse);

                        self.display_values.offset.0 -= b.0 - a.0;
                        self.display_values.offset.1 -= b.1 - a.1;

                        self.renderer.update_display(&self.display_values);
                    }
                }

                None
            },


            Event::KeyboardInput(ElementState::Pressed, _, Some(code)) => {
                match code {
                    VirtualKeyCode::Up | VirtualKeyCode::W => self.input_values.up = true,
                    VirtualKeyCode::Down | VirtualKeyCode::S => self.input_values.down = true,
                    VirtualKeyCode::Left | VirtualKeyCode::A => self.input_values.left = true,
                    VirtualKeyCode::Right | VirtualKeyCode::D => self.input_values.right = true,
                    _ => ()
                }

                None
            },

            Event::KeyboardInput(ElementState::Released, _, Some(code)) => {
                match code {
                    VirtualKeyCode::Up | VirtualKeyCode::W => self.input_values.up = false,
                    VirtualKeyCode::Down | VirtualKeyCode::S => self.input_values.down = false,
                    VirtualKeyCode::Left | VirtualKeyCode::A => self.input_values.left = false,
                    VirtualKeyCode::Right | VirtualKeyCode::D => self.input_values.right = false,
                    _ => ()
                }

                None
            },

            Event::Closed => Some(State::Quit),

            _ => None
        }
    }

    fn handle_events(&mut self, mut state: State) -> Option<State> {
        loop {
            let event = if let Some(event) = self.display.poll_events().next() {
                event
            } else {
                break
            };

            state = match state.handle_event(self, event.clone()) {
                Action::Default => if let Some(state) = self.default_actions(event) {
                    state
                } else {
                    state
                },

                Action::Continue => state,
                Action::Done(state) => state,
                Action::Quit => return None
            }
        }

        Some(state)
    }

    fn check_scroll(&mut self) {
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

    fn check_focus(&mut self) {
        let coord = self.display_values.world_coord(self.input_values.mouse);
        self.focus = self.model.view.check_focus(coord);
    }

    fn render_frame(&self) {
        let mut target = self.display.draw();
        target.clear_color(0.3, 0.3, 0.3, 1.0);
        target.clear_depth(1.0);

        self.renderer.render_tables(&mut target, &self.model.view);
        target.finish().unwrap();
    }

    fn run(&mut self) {
        let mut accum = Duration::from_millis(0);
        let mut prev = Instant::now();

        let spf = Duration::from_millis(33);

        let mut state = State::Main;
        while let Some(next) = self.handle_events(state) {
            state = next;
            self.render_frame();

            let now = Instant::now();
            accum += now - prev;
            prev = now;

            while accum >= spf {
                accum -= spf;

                state.handle_tick(self);
            }

            sleep(spf - accum);
        }
    }
}

fn main() {
    App::new().run()
}
