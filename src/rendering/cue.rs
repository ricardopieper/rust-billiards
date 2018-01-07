use entities::*;
use opengl_graphics::GlGraphics;
use graphics::*;
use piston::input::*;
use rendering::drawing::rgb;
use utils;
use entities::Line;

pub fn render_cue(pool: &Pool, args: &RenderArgs, c: &Context, gl: &mut GlGraphics) {
    //draw a line from the ball center to mouse pos

    let mouse_position = &pool.mouse_pos;

    let radius = pool.cueball.radius * ((args.width + args.height) as f64 / 2.0);

    let line_coordinates = cue_line(&pool.mouse_pos, &pool.cueball.position.scale_to_args(args), args, radius);

    let color = rgb(255.0, 0.0, 0.0, 1.0);
    line(color, 3.0, line_coordinates, c.transform, gl);
}


fn get_cue_extremity(x_points: &[f64; 2], line: &Line, mouse_position: &ScreenPoint2D) -> ScreenPoint2D {

    let screen_points: Vec<Complex> = x_points
        .iter()
        .map(|x| Complex { x: *x, y: line.y(*x) })
        .collect();

    let closest_to_mouse = mouse_position
        .as_complex()
        .find_closest(screen_points.as_slice());

    ScreenPoint2D {
        x: closest_to_mouse.x,
        y: closest_to_mouse.y
    }
}

fn cue_line(mouse_position: &ScreenPoint2D, ball_position: &ScreenPoint2D, args: &RenderArgs, radius: f64) -> [f64; 4] {

    let slope = (ball_position.y - mouse_position.y) / (ball_position.x - mouse_position.x);
    let y_intercept = ((slope * mouse_position.x) - (mouse_position.y)) * -1.0;

    let line = Line { y_intercept, slope };

    let cue_size = ((args.height as f64 + args.width as f64) / 8.0);

    let cue_tip_pivot = Circle { position: ball_position.as_complex(), radius: radius * 3.0 };
    let cue_end_pivot = Circle { position: ball_position.as_complex(), radius: (radius * 3.0) + cue_size };

    let intersections_tip = utils::math::intersections(&cue_tip_pivot, &line);
    let intersections_end = utils::math::intersections(&cue_end_pivot, &line);

    let cue_tip = get_cue_extremity(&intersections_tip, &line, &mouse_position);
    let cue_end = get_cue_extremity(&intersections_end, &line, &mouse_position);

    [cue_tip.x, cue_tip.y, cue_end.x, cue_end.y]
}

