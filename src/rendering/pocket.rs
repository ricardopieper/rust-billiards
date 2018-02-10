use entities::*;
use opengl_graphics::GlGraphics;
use graphics::*;
use rendering::drawing::rgb;

pub fn render_pockets(pool: &Pool, c: &Context, gl: &mut GlGraphics) {
    for pocket in pool.pockets.iter() {
        let r = pocket.radius * ((pool.window_width + pool.window_height) as f64 / 2.0);
        let position = pocket.position.to_screen_point(pool.window_width, pool.window_height);
        let radius = ellipse::circle(position.x, position.y, r);
        let black = rgb(0.0, 0.0, 0.0, 1.0);
        ellipse(black, radius, c.transform, gl);
    }
}