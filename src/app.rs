use drawing::*;
use color::*;
use frame_counter::*;
use graphics::*;
use opengl_graphics::GlGraphics;
use piston::input::*;
use timer::*;
use world::*;

pub struct App {
    pub gl: GlGraphics,
    pub frame_counter: FrameCounter,
    pub world: World,
}

impl App {
    pub fn new(gl: GlGraphics) -> App {
        App {
            gl: gl,
            frame_counter: FrameCounter::new(),
            world: World::new(),
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let mut timer = Timer::start();
        timer.set_name("render");

        let world = &self.world;
        self.gl.draw(args.viewport(), move |c, gl| {
            clear(Color::black().to_raw(), gl);
            world.draw(c, gl);
        });

        timer.print_duration();

        match self.frame_counter.increment() {
            IncrementResult::Ok => {}
            IncrementResult::Overflow => panic!("FrameCounter overflowed"),
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        let mut timer = Timer::start();
        self.world.update();
        timer.set_name("update");
        timer.print_duration();
    }

    pub fn press(&mut self, args: &Button) {
        let mut timer = Timer::start();
        self.world.press(args);
        timer.set_name("press");
        timer.print_duration();
    }
}
