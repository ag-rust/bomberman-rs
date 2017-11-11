extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod app;
mod timer;
mod frame_counter;
mod geometry;
mod world;
mod objects;
mod drawing;
mod color;
mod movement;

use app::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

fn main() {
    let opengl = OpenGL::V3_2;

    let window_height = 500;
    let window_width = 500;

    let mut window: Window = WindowSettings::new("bomberman", [window_width, window_height])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App::new(GlGraphics::new(opengl));

    let mut events = Events::new(EventSettings::new());
    while let Some(event) = events.next(&mut window) {
        if let Some(render_args) = event.render_args() {
            app.render(&render_args);
        }

        if let Some(button) = event.press_args() {
            app.press(&button);
        }

        if let Some(update_args) = event.update_args() {
            app.update(&update_args);
        }
    }
}
