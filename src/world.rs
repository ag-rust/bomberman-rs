use geometry::*;
use objects::*;
use movement::*;
use piston::input::*;

pub struct World {
    pub hero: Object<Hero>,
    pub enemies: Vec<Object<Enemy>>,
}

impl World {
    pub fn new() -> World {
        World {
            hero: Object {
                position: Point { x: 0, y: 0 },
                object: Hero::new(),
            },
            enemies: vec![
                Object {
                    position: Point { x: 10, y: 10 },
                    object: Enemy::new(),
                },
            ],
        }
    }
}

// Updating
impl World {
    pub fn update(&mut self) {
        // ...
    }
}

// Responding to input
impl World {
    pub fn press(&mut self, button: &Button) {
        match button {
            &Button::Keyboard(Key::Down) => {
                self.hero.apply_move(Direction::Down);
            }
            &Button::Keyboard(Key::Left) => {
                self.hero.apply_move(Direction::Left);
            }
            &Button::Keyboard(Key::Right) => {
                self.hero.apply_move(Direction::Right);
            }
            &Button::Keyboard(Key::Up) => {
                self.hero.apply_move(Direction::Up);
            }
            unknown => {
                println!("Unknown input: {:?}", unknown);
            }
        }
    }
}
