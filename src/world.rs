use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;

use crate::cell::CellState;

pub struct World {
    size: (usize, usize),
    space: Vec<Vec<CellState>>,
}

impl World {
    pub fn new(w: usize, h: usize) -> World {
        let mut res = World {
            size: (w, h),
            space: Vec::with_capacity(h),
        };
        for i in 0..h {
            res.space.push(Vec::with_capacity(w));
            for _ in 0..w {
                res.space[i].push(CellState::Dead);
            }
        }

        res
    }
    pub fn count_neigbors(self: &Self, x: usize, y: usize) -> u8 {
        let mut res: u8 = 0;
        let x = x as i32;
        let y = y as i32;
        for i in (x - 1)..=(x + 1) {
            for j in (y - 1)..=(y + 1) {
                if i >= 0
                    && i < self.size.0 as i32
                    && j >= 0
                    && j < self.size.1 as i32
                    && !(i == x && j == y)
                {
                    match self.space[j as usize][i as usize] {
                        CellState::Alive | CellState::GoingToDie => res += 1,
                        _ => {}
                    }
                }
            }
        }

        res
    }
    pub fn calculate_world(self: &mut Self) {
        for i in 0..self.size.0 {
            for j in 0..self.size.1 {
                let neigbors = self.count_neigbors(i, j);
                self.space[j][i].calculate_state(neigbors);
            }
        }
    }

    pub fn process_world(self: &mut Self) {
        for i in &mut self.space {
            for j in i {
                j.process_state();
            }
        }
    }
    pub fn render(self: &Self, args: RenderArgs, gl: &mut GlGraphics, size: u32, color: [f32; 4]) {
        use graphics::*;
        gl.draw(args.viewport(), |c, gl| {
            let square = rectangle::square(0.0, 0.0, size as f64);
            for i in 0..self.size.0 {
                for j in 0..self.size.1 {
                    match self.space[j][i] {
                        CellState::Alive => {
                            let transform = c
                                .transform
                                .trans(i as f64 * size as f64, j as f64 * size as f64);
                            rectangle(color, square, transform, gl);
                        }
                        _ => {}
                    }
                }
            }
        });
    }
    pub fn change_cell(self: &mut Self, x: usize, y: usize) {
        let curr = self.space[y][x];
        self.space[y][x] = match curr {
            CellState::Alive => CellState::Dead,
            CellState::Dead => CellState::Alive,
            _ => CellState::Dead,
        }
    }
    pub fn clear(self: &mut Self) {
        for i in &mut self.space {
            for j in i {
                *j = CellState::Dead;
            }
        }
    }
}
