use ggez::{
    graphics::{self, Color, Mesh, Rect},
    Context,
};

use crate::game::{consts, coords::Coords, direction::Direction};

use super::{
    maths,
    segment::{Growable, Renderable, Segment},
};

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Line {
    pub beg: Coords,
    pub end: Coords,
    pub dir: Direction,
}

impl Segment for Line {}

impl Line {
    pub fn new(pos: Coords, dir: Direction) -> Self {
        Self {
            beg: pos,
            end: pos + dir.as_coords() * 0.01,
            dir,
        }
    }

    pub fn size(&self) -> f32 {
        match self.dir {
            Direction::UP | Direction::DOWN => (self.end.y - self.beg.y).abs(),
            Direction::LEFT | Direction::RIGHT => (self.end.x - self.beg.x).abs(),
        }
    }
}

impl Growable for Line {
    fn grow(&mut self, dist: f32) -> f32 {
        match self.dir {
            Direction::UP => self.end.y -= dist,
            Direction::DOWN => self.end.y += dist,
            Direction::LEFT => self.end.x -= dist,
            Direction::RIGHT => self.end.x += dist,
        };
        0.
    }

    fn shrink(&mut self, dist: f32) -> f32 {
        let left = maths::clamp(dist - self.size(), 0., dist);
        match self.dir {
            Direction::UP => self.beg.y -= dist,
            Direction::DOWN => self.beg.y += dist,
            Direction::LEFT => self.beg.x -= dist,
            Direction::RIGHT => self.beg.x += dist,
        };

        return left;
    }

    fn get_end(&self) -> Coords {
        self.end
    }

    fn get_dir(&self) -> Direction {
        self.dir
    }
}

impl Renderable for Line {
    fn get_bbox(&self) -> Rect {
        let (x, y, w, h) = match self.dir {
            Direction::UP => (
                self.end.x - consts::SNAKE_HALF_WIDTH,
                self.end.y,
                consts::SNAKE_WIDTH,
                (self.end.y - self.beg.y).abs(),
            ),
            Direction::DOWN => (
                self.end.x - consts::SNAKE_HALF_WIDTH,
                self.beg.y,
                consts::SNAKE_WIDTH,
                (self.end.y - self.beg.y).abs(),
            ),
            Direction::LEFT => (
                self.end.x,
                self.end.y - consts::SNAKE_HALF_WIDTH,
                (self.end.x - self.beg.x).abs(),
                consts::SNAKE_WIDTH,
            ),
            Direction::RIGHT => (
                self.beg.x,
                self.end.y - consts::SNAKE_HALF_WIDTH,
                (self.end.x - self.beg.x).abs(),
                consts::SNAKE_WIDTH,
            ),
        };

        Rect::new(x, y, w, h)
    }

    fn draw(&self, ctx: &mut Context) {
        let mesh = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.get_bbox(),
            Color::from_rgb(255, 255, 0),
        )
        .unwrap();
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())
            .expect("Error while drawing Line");

        #[cfg(feature = "debug")]
        {
            let mesh = Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::stroke(1.),
                self.get_bbox(),
                Color::from_rgb(255, 0, 0),
            )
            .unwrap();
            graphics::draw(ctx, &mesh, graphics::DrawParam::default())
                .expect("Error while drawing Line border");
        }
    }
}
