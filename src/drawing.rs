use graphics::*;
use opengl_graphics::GlGraphics;
use world::*;
use geometry::*;
use objects::*;
use color::*;

pub trait Drawable<Args> {
    fn draw_with(&self, args: Args, c: Context, gl: &mut GlGraphics);
}

impl Drawable<()> for World {
    fn draw_with(&self, _: (), c: Context, gl: &mut GlGraphics) {
        self.hero.draw_with(&self.hero.position, c, gl);

        for enemy in &self.enemies {
            enemy.draw_with(&enemy.position, c, gl);
        }
    }
}

impl<'a> Drawable<&'a Point> for Hero {
    fn draw_with(&self, point: &'a Point, c: Context, gl: &mut GlGraphics) {
        self.shape.draw_with((point, &self.color), c, gl);
    }
}

impl<'a> Drawable<&'a Point> for Enemy {
    fn draw_with(&self, point: &'a Point, c: Context, gl: &mut GlGraphics) {
        self.shape.draw_with((point, &self.color), c, gl);
    }
}

impl<'a> Drawable<(&'a Point, &'a Color)> for Square {
    fn draw_with(&self, args: (&'a Point, &'a Color), c: Context, gl: &mut GlGraphics) {
        let (point, color) = args;

        let x = point.x as f64;
        let y = point.y as f64;
        let size = self.size as f64;
        let square = rectangle::square(x, y, size);

        let transform = c.transform.trans(point.x as f64, point.y as f64);
        rectangle(color.to_raw(), square, transform, gl);
    }
}

impl<Args, O> Drawable<Args> for Object<O>
where
    O: Drawable<Args>,
{
    fn draw_with(&self, args: Args, c: Context, gl: &mut GlGraphics) {
        self.object.draw_with(args, c, gl);
    }
}

pub trait DrawWithoutArgs {
    fn draw(&self, c: Context, gl: &mut GlGraphics);
}

impl<T> DrawWithoutArgs for T
where
    T: Drawable<()>,
{
    fn draw(&self, c: Context, gl: &mut GlGraphics) {
        self.draw_with((), c, gl);
    }
}
