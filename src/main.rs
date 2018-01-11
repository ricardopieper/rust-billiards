mod entities;
mod game;
mod rendering;
mod utils;
mod complex;

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

use entities::*;
use rendering::pool::*;
use complex::*;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new(
        "Rust Billiards",
        [640, 480]
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();


    let ball = Ball {
        number: 8,
        position: Point2D { x: 0.1, y: 0.3 },
        speed: Vector2D { x: 0.0, y: 0.0 },
        radius: 0.03
    };

    let cueball = Ball {
        number: 0,
        position: Point2D { x: 0.5, y: 0.5 },
        speed: Vector2D { x: 0.0, y: 0.0 },
        radius: 0.03
    };

    let pocket_tl = Pocket { position: Point2D { x: 0.0, y: 0.0}, radius: 0.05 };
    let pocket_tm = Pocket { position: Point2D { x: 0.5, y: 0.0}, radius: 0.05 };
    let pocket_tr = Pocket { position: Point2D { x: 1.0, y: 0.0}, radius: 0.05 };
    let pocket_bl = Pocket { position: Point2D { x: 0.0, y: 1.0}, radius: 0.05 };
    let pocket_bm = Pocket { position: Point2D { x: 0.5, y: 1.0}, radius: 0.05 };
    let pocket_br = Pocket { position: Point2D { x: 1.0, y: 1.0}, radius: 0.05 };


    let mut pool = Pool {
        pockets: vec![
            pocket_tl,
            pocket_tm,
            pocket_tr,
            pocket_bl,
            pocket_bm,
            pocket_br
        ],
        balls: vec![ball],
        mouse_pos: ScreenPoint2D {x: 354.0, y: 408.0},
        cueball };

    let mut gl = GlGraphics::new(opengl);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            render(&pool, r.width, r.height, &r, &mut gl)
        }
        if let Some(pos) = e.mouse_cursor_args() {
            pool.set_mouse_pos(pos);
        }
    }
}