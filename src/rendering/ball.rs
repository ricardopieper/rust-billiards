use entities::*;
use opengl_graphics::GlGraphics;
use graphics::*;
use rendering::drawing::rgb;


pub fn render_balls(pool: &Pool, c: &Context, gl: &mut GlGraphics) {
    for ball in pool.balls.iter() {

        let r = ball.radius * ((pool.window_width + pool.window_height) as f64 / 2.0);
        let position = ball.position.to_screen_point(pool.window_width, pool.window_height);

        let radius = ellipse::circle(position.x, position.y, r);
        let color = rgb(255.0, 0.0, 0.0, 1.0);
        ellipse(color, radius, c.transform, gl);
    }
}


pub fn render_cueball(pool: &Pool, c: &Context, gl: &mut GlGraphics) {
    let r = pool.cueball.radius * ((pool.window_width + pool.window_height) as f64 / 2.0);
    let position = pool.cueball.position.to_screen_point(pool.window_width, pool.window_height);

    let radius = ellipse::circle(position.x, position.y, r);
    let white = rgb(255.0, 255.0, 255.0, 1.0);
    ellipse(white, radius, c.transform, gl);
}

