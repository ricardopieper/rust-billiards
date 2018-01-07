use entities::*;
use opengl_graphics::GlGraphics;
use graphics::*;
use piston::input::*;
use rendering::pocket::render_pockets;
use rendering::ball::render_balls;
use rendering::ball::render_cueball;
use rendering::cue::render_cue;
use rendering::drawing::*;

pub fn render(pool: &Pool, window_width: u32, window_height: u32, args: &RenderArgs, gl: &mut GlGraphics) {
    gl.draw(args.viewport(), |c, gl| {
        render_pool(gl);
        render_pockets(&pool.pockets, &args, &c, gl);
        render_balls(&pool.balls, &args, &c, gl);
        render_cueball(&pool, &args, &c, gl);
        render_cue(&pool, &args, &c, gl);
    });
}

fn render_pool(gl: &mut GlGraphics) {
    clear(rgb(50.0, 200.0, 50.0, 1.0), gl);
}

