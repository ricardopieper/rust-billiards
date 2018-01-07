use entities::*;
use opengl_graphics::GlGraphics;
use graphics::*;
use piston::input::*;
use rendering::drawing::rgb;

pub fn render_cue(pool: &Pool, args: &RenderArgs, c: &Context, gl: &mut GlGraphics) {
    //draw a line from the ball center to mouse pos

    let mouse_position = &pool.mouse_pos;

    let radius = pool.cueball.radius * ((args.width + args.height) as f64 / 2.0);

    let line_coordinates = cue_line(&pool.mouse_pos, &pool.cueball.position.scale_to_args(args), args, radius);

    let color = rgb(255.0, 0.0, 0.0, 1.0);
    line(color, 3.0, line_coordinates, c.transform, gl);
}

fn find_intersections(mouse_position: &ScreenPoint2D, ball_position: &ScreenPoint2D, radius: f64) -> [f64; 2] {
    //https://math.stackexchange.com/questions/228841/how-do-i-calculate-the-intersections-of-a-straight-line-and-a-circle
    //https://cscheng.info/2016/06/09/calculate-circle-line-intersection-with-javascript-and-p5js.html

    let m = (ball_position.y - mouse_position.y) / (ball_position.x - mouse_position.x);
    let c = ((m * mouse_position.x) - (mouse_position.y)) * -1.0;
    let h = ball_position.x;
    let k = ball_position.y;

    let a = 1.0 + (m * m); //accounts for (x)^2 and mx^2
    let b = (-h * 2.0) + (m * (c - k)) * 2.0; //accounts for x*h + x * h and m * c-k + m * c-k
    let c = (h * h) + (c - k).powi(2) - (radius * radius); //accounts for the rest of the elements

    let x_pos = (-b + ((b * b) - (4.0 * a * c)).sqrt()) / (2.0 * a);
    let x_neg = (-b - ((b * b) - (4.0 * a * c)).sqrt()) / (2.0 * a);

    [x_pos, x_neg]
}

fn find_appropriate_intersection(x_points: &[f64; 2], mouse_position: &ScreenPoint2D, ball_position: &ScreenPoint2D) -> ScreenPoint2D {
    let m = (ball_position.y - mouse_position.y) / (ball_position.x - mouse_position.x);
    let c = ((m * mouse_position.x) - (mouse_position.y)) * -1.0;

    let y = |x: f64| {
        (m * x) + c
    };

    let screen_points: Vec<Complex> = x_points
        .iter()
        .map(|x| Complex { x: *x, y: y(*x) })
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
    let intersections_close = find_intersections(&mouse_position, &ball_position, radius * 3.0);

    let cue_size = ((args.height as f64 + args.width as f64) / 8.0); //15%
    let intersections_far = find_intersections(&mouse_position, &ball_position, (radius * 3.0) + cue_size);

    let cue_tip = find_appropriate_intersection(&intersections_close, &mouse_position, &ball_position);
    let cue_end = find_appropriate_intersection(&intersections_far, &mouse_position, &ball_position);

    [cue_tip.x, cue_tip.y, cue_end.x, cue_end.y]
}

