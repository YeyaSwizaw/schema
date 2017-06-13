#[macro_use] extern crate stateloop;
#[macro_use] extern crate glium;

use std::cmp;

use glium::Surface;
use glium::glutin::{Event, ElementState, MouseScrollDelta, VirtualKeyCode};
use glium::backend::glutin_backend::GlutinFacade;

use stateloop::app::App;
use stateloop::state::Action;

use model::Model;
use view::Index;
use renderer::Renderer;
use values::*;
use states::State;

mod model;
mod view;
mod shaders;
mod values;
mod renderer;
mod states;

pub struct Stuff<'a> {
    renderer: Renderer<'a>,

    display_values: DisplayValues,
    input_values: InputValues,
    focus: Option<Index>,

    model: Model
}

impl<'a> Stuff<'a> {
    fn default_action(&mut self, event: Event) -> Action<State> {
        match event {
            Event::Resized(w, h) => {
                self.display_values.size = (w, h);
                self.renderer.update_display(&self.display_values);
                Action::Continue
            },

            Event::MouseMoved(x, y) => {
                self.input_values.mouse = DisplayCoord(x, y);
                self.input_values.moved = true;
                Action::Continue
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
                            return Action::Continue
                        };

                        let b = self.display_values.world_coord(self.input_values.mouse);

                        self.display_values.offset.0 -= b.0 - a.0;
                        self.display_values.offset.1 -= b.1 - a.1;

                        self.renderer.update_display(&self.display_values);
                    }
                }

                Action::Continue
            },


            Event::KeyboardInput(ElementState::Pressed, _, Some(code)) => {
                match code {
                    VirtualKeyCode::Up | VirtualKeyCode::W => self.input_values.up = true,
                    VirtualKeyCode::Down | VirtualKeyCode::S => self.input_values.down = true,
                    VirtualKeyCode::Left | VirtualKeyCode::A => self.input_values.left = true,
                    VirtualKeyCode::Right | VirtualKeyCode::D => self.input_values.right = true,
                    _ => ()
                }

                Action::Continue
            },

            Event::KeyboardInput(ElementState::Released, _, Some(code)) => {
                match code {
                    VirtualKeyCode::Up | VirtualKeyCode::W => self.input_values.up = false,
                    VirtualKeyCode::Down | VirtualKeyCode::S => self.input_values.down = false,
                    VirtualKeyCode::Left | VirtualKeyCode::A => self.input_values.left = false,
                    VirtualKeyCode::Right | VirtualKeyCode::D => self.input_values.right = false,
                    _ => ()
                }

                Action::Continue
            },

            Event::Closed => Action::Quit,

            _ => Action::Continue
        }
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

    fn render_frame(&self, display: &GlutinFacade) {
        let mut target = display.draw();
        target.clear_color(0.3, 0.3, 0.3, 1.0);
        target.clear_depth(0.0);

        self.renderer.render_tables(&mut target, &self.model.view);

        if let Some(table) = self.focus.and_then(|index| self.model.view.get_table(index)) {
            self.renderer.render_focus(&mut target, table);
        }

        target.finish().unwrap();
    }
}

fn main() {
    App::new(
        |builder| builder
            .with_title("Schema Designer")
            .with_dimensions(600, 600)
            .with_depth_buffer(24)
            .with_vsync(),

        |display| {
            let values = DisplayValues::new(display.get_window().unwrap().get_inner_size_pixels().unwrap());
            let renderer = Renderer::new(display);
            renderer.update_display(&values);

            Stuff {
                renderer: renderer,

                display_values: values,
                input_values: InputValues::new(),
                focus: None,

                model: Model::new()
            }
        }
    )
        .run(30, State::Main())
}
