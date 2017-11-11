use geometry::*;
use color::*;

pub struct Object<T> {
    pub position: Point,
    pub object: T,
}

pub struct Hero {
    pub shape: Square,
    pub color: Color,
}

impl Hero {
    pub fn new() -> Hero {
        Hero {
            shape: Square { size: 10 },
            color: Color::green(),
        }
    }
}

pub struct Enemy {
    pub shape: Square,
    pub color: Color,
}

impl Enemy {
    pub fn new() -> Enemy {
        Enemy {
            shape: Square { size: 20 },
            color: Color::red(),
        }
    }
}
