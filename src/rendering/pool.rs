use entities::*;
use opengl_graphics::GlGraphics;
use graphics::*;
use piston::input::*;
use rendering::pocket::*;
use rendering::ball::*;
use rendering::cue::*;
use rendering::drawing::*;
use geometry::*;
use complex::*;

pub fn render(pool: &mut Pool, args: &RenderArgs, gl: &mut GlGraphics) {
    gl.draw(args.viewport(), |c, gl| {
        render_pool(pool, args, &c, gl);
        render_pockets(&pool, &c, gl);
        render_balls(&pool, &c, gl);
        render_cueball(&pool, &c, gl);
        if pool.ball_positions.cueball.is_stopped() {
            render_cue(&pool, &c, gl);
        }
    });
}

fn render_pool(pool: &mut Pool, args: &RenderArgs, c: &Context, gl: &mut GlGraphics) {
    clear(rgb(0.0, 0.0, 0.0, 1.0), gl);

    let window_width = args.width as f64;
    let window_height = args.height as f64;


    let window = ScreenRectangleParent {
        origin: ScreenPoint2D::new(0.0, 0.0),
        width: window_width,
        height: window_height
    };

    pool.play_area.parent = window;

    //when the window ratio is perfectly 2 : 1 we render at (0,0) (w, h)
    //when it's 1:1 the pool will occupy 50% of the window in the middle (center, vertical).
    //the other 50% will be divided by 2 so 25%

    let ratio = window_width / window_height;


    if ratio < 2.0 { //less wide than desired
        //forces a 2:1 ratio
        pool.play_area.width = window_width;
        pool.play_area.height = window_width / 2.0;

        //calculates origin displaced from 0,0
        pool.play_area.origin.y = (window_height - pool.play_area.height) / 2.0;
        pool.play_area.origin.x = 0.0;
    }
    else if ratio > 2.0 { //way too wide
        pool.play_area.height = window_height;
        pool.play_area.width = window_height * 2.0;

        //calculates origin displaced from 0,0
        pool.play_area.origin.x = (window_width - pool.play_area.width) / 2.0;
        pool.play_area.origin.y = 0.0;
    }
    else {
        pool.play_area.origin.x = 0.0;
        pool.play_area.origin.y = 0.0;
        pool.play_area.width = window_width;
        pool.play_area.height = window_height;
    }

    let rect = [
        pool.play_area.origin.x,
        pool.play_area.origin.y,
        pool.play_area.width,
        pool.play_area.height];

    clear(rgb(0.0, 0.0, 0.0, 1.0), gl);

    rectangle(rgb(50.0, 200.0, 50.0, 1.0), rect, c.transform, gl);
}

