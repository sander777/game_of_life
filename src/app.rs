pub use glutin_window::GlutinWindow;
pub use opengl_graphics::{GlGraphics, OpenGL};
pub use piston::event_loop::{EventSettings, Events};
pub use piston::input::{
    Button, Key, MouseCursorEvent, PressEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent,
};
pub use piston::window::WindowSettings;

use crate::*;
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum AppState {
    Pause,
    Run,
}

pub struct App {
    gl: GlGraphics,
    dlt: f64,
    upd_dlt: f64,
    world: world::World,
    size: u32,
    state: AppState,
}

impl App {
    pub fn new(opengl: OpenGL, w: u32, h: u32) -> App {
        App {
            gl: GlGraphics::new(opengl),
            dlt: 0.0,
            upd_dlt: 0.01,
            size: 30,
            world: world::World::new(w as usize, h as usize),
            state: AppState::Pause,
        }
    }
    pub fn set_upd_dlt(mut self: Self, dlt: f64) -> Self {
        self.upd_dlt = dlt;
        self
    }
    pub fn set_cell_size(mut self: Self, size: u32) -> Self {
        self.size = size;
        self
    }

    pub fn render(self: &mut Self, args: RenderArgs) {
        use graphics::*;

        self.gl.draw(args.viewport(), |_c, gl| {
            clear([0.0, 0.0, 0.0, 0.0], gl);
        });
        let color = match self.state {
            AppState::Pause => [0.7, 0.7, 0.7, 1.0],
            AppState::Run => [1.0, 1.0, 1.0, 1.0],
        };
        self.world.render(args, &mut self.gl, self.size, color);
    }

    pub fn update(self: &mut Self, args: UpdateArgs) {
        self.dlt += args.dt;
        if self.dlt >= self.upd_dlt {
            if AppState::Run == self.state {
                self.world.calculate_world();
                self.world.process_world();
            }
            self.dlt = 0.0;
        }
    }

    pub fn change_cell(self: &mut Self, pos: [f64; 2]) {
        if self.state == AppState::Pause {
            let (x, y) = (
                pos[0] as usize / self.size as usize,
                pos[1] as usize / self.size as usize,
            );
            self.world.change_cell(x, y);
        }
    }

    pub fn clear(self: &mut Self) {
        self.state = AppState::Pause;
        self.world.clear();
    }

    pub fn start_or_stop(self: &mut Self) {
        self.state = match self.state {
            AppState::Pause => AppState::Run,
            AppState::Run => AppState::Pause,
        }
    }

    pub fn state(self: &Self) -> AppState {
        self.state
    }
}
