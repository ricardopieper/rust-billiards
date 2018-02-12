use entities::*;
use opengl_graphics::GlGraphics;
use graphics::*;
use rendering::drawing::rgb;


pub fn render_balls(pool: &Pool, c: &Context, gl: &mut GlGraphics) {
    for ball in pool.balls.iter() {

        let r = ball.radius * pool.play_area.aspect();
        let position = ball.position.to_screen_point_from_rect(&pool.play_area);

        let radius = ellipse::circle(position.x, position.y, r);
        let color = rgb(255.0, 0.0, 0.0, 1.0);
        ellipse(color, radius, c.transform, gl);
    }
}


pub fn render_cueball(pool: &Pool, c: &Context, gl: &mut GlGraphics) {
    let r = pool.cueball.radius *  pool.play_area.aspect();
    let position = pool.cueball.position.to_screen_point_from_rect(&pool.play_area);

    let radius = ellipse::circle(position.x, position.y, r);
    let white = rgb(255.0, 255.0, 255.0, 1.0);
    ellipse(white, radius, c.transform, gl);
}

