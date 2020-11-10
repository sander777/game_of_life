extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

mod app;
mod cell;
mod world;

use app::*;

const SPACE_SIZE: [u32; 2] = [40, 20];
const SIZE: u32 = 30;
const WINDOW_SIZE: [u32; 2] = [SPACE_SIZE[0] * SIZE, SPACE_SIZE[1] * SIZE];

fn main() {
    let opengl = OpenGL::V4_5;
    let mut window: GlutinWindow = WindowSettings::new("Pause", WINDOW_SIZE)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut app = App::new(opengl, SPACE_SIZE[0], SPACE_SIZE[1], SIZE);
    let mut last_pos = [0.0, 0.0];
    let mut event = Events::new(EventSettings::new());
    while let Some(e) = event.next(&mut window) {
        if let Some(e) = e.mouse_cursor_args() {
            last_pos = e;
        }
        if let Some(args) = e.render_args() {
            app.render(args);
        }
        if let Some(args) = e.update_args() {
            app.update(args);
        }
        let press = match e.press_args() {
            Some(it) => it,
            _ => continue,
        };
        match press {
            Button::Mouse(m) => match m {
                piston::MouseButton::Left => {
                    app.change_cell(last_pos);
                }
                _ => {}
            },
            Button::Keyboard(key) => match key {
                Key::Space => app.start_or_stop(),
                Key::R => app.clear(),
                _ => {}
            }
            _ => {}
        }
        window.ctx.window().set_title(match app.state() {
            AppState::Pause => "Pause",
            AppState::Run => "Run",
        })
    }
}
