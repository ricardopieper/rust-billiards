use entities::*;
use opengl_graphics::GlGraphics;
use graphics::*;
use piston::input::*;
use rendering::pocket::render_pockets;
use rendering::ball::render_balls;
use rendering::ball::render_cueball;
use rendering::cue::render_cue;
use rendering::drawing::*;

pub fn render(pool: &mut Pool, args: &RenderArgs, gl: &mut GlGraphics) {
    pool.window_width = args.width as f64;
    pool.window_height = args.height as f64;
    gl.draw(args.viewport(), |c, gl| {
        render_pool(pool, &c, gl);
        render_pockets(&pool,&c, gl);
        render_balls(&pool, &c, gl);
        render_cueball(&pool, &c, gl);
        if pool.cueball.is_stopped() {
            render_cue(&pool, &c, gl);
        }
    });
}

fn render_pool(pool: &mut Pool, c: &Context, gl: &mut GlGraphics) {
    clear(rgb(0.0,0.0,0.0,1.0),gl);

    //when the window ratio is perfectly 2 : 1 we render at (0,0) (w, h)
    //when it's 1:1 the pool will occupy 50% of the window in the middle (center, vertical).
    //the other 50% will be divided by 2 so 25%

    let ratio = pool.window_width / pool.window_height;


    if ratio < 2.0 { //less wide than desired
        //forces a 2:1 ratio
        pool.window_rect.horizontal_size = pool.window_width;
        pool.window_rect.vertical_size = pool.window_width / 2.0;

        //calculates origin displaced from 0,0
        pool.window_rect.origin.y = (pool.window_height - pool.window_rect.vertical_size) / 2.0;
        pool.window_rect.origin.x = 0.0;
    }

    if ratio > 2.0 { //way too wide
        pool.window_rect.vertical_size = pool.window_height;
        pool.window_rect.horizontal_size = pool.window_height * 2.0;

        //calculates origin displaced from 0,0
        pool.window_rect.origin.x = (pool.window_width -  pool.window_rect.horizontal_size) / 2.0;
        pool.window_rect.origin.y = 0.0;
    }

    let rect = [
        pool.window_rect.origin.x,
        pool.window_rect.origin.y,
        pool.window_rect.horizontal_size,
        pool.window_rect.vertical_size];

    rectangle([1.0; 4], rect, c.transform, gl);

    clear(rgb(50.0, 200.0, 50.0, 1.0), gl);
}

