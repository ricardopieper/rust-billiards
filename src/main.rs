mod entities;
mod rendering;
mod utils;
mod complex;
mod ui;
mod geometry;
mod physics;

extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::Button::Mouse;

use entities::*;
use rendering::pool::*;
use complex::*;
use geometry::*;
use physics::*;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new(
        "Rust Billiards",
        [600, 300]
    )
        .opengl(opengl)
        .samples(4)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let ball = Ball {
        number: 1,
        position: Point2D::new(0.35, 0.25),
        speed: Point2D::new(0.0, 0.0),
        radius: 0.025
    };

   /* let ball2 = Ball {
        number: 1,
        position: Point2D::new(0.25, 0.075),
        speed: Point2D::new(0.1, 0.0),
        radius: 0.025
    };

    let ball3 = Ball {
        number: 1,
        position: Point2D::new(0.25, 0.125),
        speed: Point2D::new(0.1, 0.0),
        radius: 0.025
    };

    let ball4 = Ball {
        number: 1,
        position: Point2D::new(0.25, 0.175),
        speed: Point2D::new(0.1, 0.0),
        radius: 0.025
    };

    let ball5 = Ball {
        number: 1,
        position: Point2D::new(0.25, 0.225),
        speed: Point2D::new(0.1, 0.0),
        radius: 0.025
    };

    let ball6 = Ball {
        number: 1,
        position: Point2D::new(0.25, 0.275),
        speed: Point2D::new(0.1, 0.0),
        radius: 0.025
    };

    let ball7 = Ball {
        number: 1,
        position: Point2D::new(0.25, 0.325),
        speed: Point2D::new(0.1, 0.0),
        radius: 0.025
    };

    let ball8 = Ball {
        number: 1,
        position: Point2D::new(0.25, 0.375),
        speed: Point2D::new(0.1, 0.0),
        radius: 0.025
    };

    let ball9 = Ball {
        number: 1,
        position: Point2D::new(0.25, 0.425),
        speed: Point2D::new(0.1, 0.0),
        radius: 0.025
    };

    let ball10 = Ball {
        number: 1,
        position: Point2D::new(0.25, 0.470),
        speed: Point2D::new(0.1, 0.0),
        radius: 0.025
    };
*/
    let cueball = Ball {
        number: 0,
        position: Point2D::new(0.5, 0.25),
        speed: Vector2D::new(0.0, 0.0),
        radius: 0.025
    };

    let pocket_tl = Pocket { position: Point2D::new(0.0, 0.0), radius: 0.04 };
    let pocket_tm = Pocket { position: Point2D::new(0.5, 0.0), radius: 0.04 };
    let pocket_tr = Pocket { position: Point2D::new(1.0, 0.0), radius: 0.04 };
    let pocket_bl = Pocket { position: Point2D::new(0.0, 0.5), radius: 0.04 };
    let pocket_bm = Pocket { position: Point2D::new(0.5, 0.5), radius: 0.04 };
    let pocket_br = Pocket { position: Point2D::new(1.0, 0.5), radius: 0.04 };


    let mut pool = Pool {
        pockets: vec![
            pocket_tl,
            pocket_tm,
            pocket_tr,
            pocket_bl,
            pocket_bm,
            pocket_br
        ],
        ball_positions: BallPositions {
            balls: vec![ball /*, ball2, ball3, ball4, ball5, ball6, ball7, ball8, ball9, ball10*/],
            cueball
        },
        mouse_pos: ScreenPoint2D::new(354.0, 408.0),
        play_area: ScreenRectangle {
            origin: ScreenPoint2D::new(0.0, 0.0),
            height: 0.0,
            width: 0.0,
            parent: ScreenRectangleParent {
                origin: ScreenPoint2D::new(0.0, 0.0),
                height: 0.0,
                width: 0.0,
            }
        },
    };

    let mut gl = GlGraphics::new(opengl);

    let mut events = Events::new(EventSettings::new());
    events.set_ups(60);

    while let Some(e) = events.next(&mut window) {
        if let Some(u) = e.update_args() {
            pool.update();
        }
        if let Some(r) = e.render_args() {
            render(&mut pool,&r, &mut gl)
        }
        if let Some(pos) = e.mouse_cursor_args() {
            pool.set_mouse_pos(pos);
        }
        if let Some(Mouse(MouseButton::Left)) = e.press_args() {
            pool.stun_shot()
        }
    }
}