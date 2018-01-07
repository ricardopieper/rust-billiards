use entities::*;
use opengl_graphics::GlGraphics;
use graphics::*;
use piston::input::*;
use rendering::drawing::rgb;


pub fn render_balls(balls: &Vec<Ball>, args: &RenderArgs, c: &Context, gl: &mut GlGraphics) {
    for ball in balls {

        let r = ball.radius * ((args.width + args.height) as f64 / 2.0);
        let position = ball.position.scale_to_args(args);

        let radius = ellipse::circle(position.x, position.y, r);
        let color = rgb(255.0, 0.0, 0.0, 1.0);
        ellipse(color, radius, c.transform, gl);
    }
}


pub fn render_cueball(pool: &Pool, args: &RenderArgs, c: &Context, gl: &mut GlGraphics) {
    let r = pool.cueball.radius * ((args.width + args.height) as f64 / 2.0);
    let position = pool.cueball.position.scale_to_args(args);

    let radius = ellipse::circle(position.x, position.y, r);
    let white = rgb(255.0, 255.0, 255.0, 1.0);
    ellipse(white, radius, c.transform, gl);
}

