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
        render_pool(gl);
        render_pockets(&pool,&c, gl);
        render_balls(&pool, &c, gl);
        render_cueball(&pool, &c, gl);
        if pool.cueball.is_stopped() {
            render_cue(&pool, &c, gl);
        }
    });
}

fn render_pool(gl: &mut GlGraphics) {
    clear(rgb(50.0, 200.0, 50.0, 1.0), gl);
}

