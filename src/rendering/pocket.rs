use entities::*;
use opengl_graphics::GlGraphics;
use graphics::*;
use rendering::drawing::rgb;

pub fn render_pockets(pool: &Pool, c: &Context, gl: &mut GlGraphics) {
    for pocket in pool.pockets.iter() {
        let r = pocket.radius * pool.play_area.aspect();
        let position = pocket.position.to_screen_point_from_rect(&pool.play_area);
        let radius = ellipse::circle(position.x, 2.0 * position.y, r);
        let black = rgb(0.0, 0.0, 0.0, 1.0);
        ellipse(black, radius, c.transform, gl);
    }
}