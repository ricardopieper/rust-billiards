use entities::*;
use opengl_graphics::GlGraphics;
use graphics::*;
use piston::input::*;
use rendering::drawing::rgb;

pub fn render_pockets(pockets: &Vec<Pocket>, args: &RenderArgs, c: &Context, gl: &mut GlGraphics) {
    for pocket in pockets {
        let r = pocket.radius * ((args.width + args.height) as f64 / 2.0);
        let position = pocket.position.scale_to_args(args);
        let radius = ellipse::circle(position.x, position.y, r);
        let black = rgb(0.0, 0.0, 0.0, 1.0);
        ellipse(black, radius, c.transform, gl);
    }
}