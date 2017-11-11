use objects::*;
use geometry::*;

pub trait MoveableViaInput {
    fn apply_move(&mut self, direction: Direction);
}

impl<T> MoveableViaInput for Object<T>
where
    T: Moveable,
{
    fn apply_move(&mut self, direction: Direction) {
        let new_pos = self.object.new_position_from(&self.position, direction);
        self.position = new_pos;
    }
}

pub trait Moveable {
    fn new_position_from(&mut self, position: &Point, direction: Direction) -> Point;
}

impl Moveable for Hero {
    fn new_position_from(&mut self, position: &Point, direction: Direction) -> Point {
        match direction {
            Direction::Up => {
                Point {
                    x: position.x,
                    y: position.y - 1,
                }
            }
            Direction::Down => {
                Point {
                    x: position.x,
                    y: position.y + 1,
                }
            }
            Direction::Left => {
                Point {
                    x: position.x - 1,
                    y: position.y,
                }
            }
            Direction::Right => {
                Point {
                    x: position.x + 1,
                    y: position.y,
                }
            }
        }
    }
}
