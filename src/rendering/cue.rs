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

enum VerticalOrientation {
    Top,
    Bottom
}


enum HorizontalOrientation {
    Left,
    Right
}


fn find_orientation(mouse_position: &ScreenPoint2D, ball_position: &ScreenPoint2D) -> (VerticalOrientation, HorizontalOrientation) {
    println!("mouse: {:?}, cueball: {:?}", mouse_position, ball_position);

    let horizontal_orientation = if mouse_position.x > ball_position.x {
        HorizontalOrientation::Right
    } else {
        HorizontalOrientation::Left
    };

    let vertical_orientation = if mouse_position.y > ball_position.y {
        VerticalOrientation::Top
    } else {
        VerticalOrientation::Bottom
    };

    (vertical_orientation, horizontal_orientation)
}

fn find_intersections(mouse_position: &ScreenPoint2D, ball_position: &ScreenPoint2D, radius: f64) -> [f64; 2] {
    //https://math.stackexchange.com/questions/228841/how-do-i-calculate-the-intersections-of-a-straight-line-and-a-circle
    //https://cscheng.info/2016/06/09/calculate-circle-line-intersection-with-javascript-and-p5js.html
    //(x - h)2 + (y - k)2 = r2
    //(x - h)2 + (mx + c - k)2 = r2
    //c = -545
    // k = 144
    let m = (ball_position.y - mouse_position.y) / (ball_position.x - mouse_position.x);
    let c = ((m * mouse_position.x) - (mouse_position.y)) * -1.0;
    let h = ball_position.x;
    let k = ball_position.y;

    let a = 1.0 + (m * m); //accounts for (x)^2 and mx^2
    let b = (-h * 2.0) + (m * (c - k)) * 2.0; //accounts for x*h + x * h and m * c-k + m * c-k
    let c = (h * h) + (c - k).powi(2) - (radius * radius); //accounts for the rest of the elements

    let x_pos = (-b + ((b*b) - (4.0*a*c)).sqrt()) / (2.0*a);
    let x_neg = (-b - ((b*b) - (4.0*a*c)).sqrt()) / (2.0*a);


    [x_pos, x_neg]

}

fn cue_line(mouse_position: &ScreenPoint2D, ball_position: &ScreenPoint2D, args: &RenderArgs, radius: f64) -> [f64; 4] {
    let intersections_close = find_intersections(&mouse_position, &ball_position, radius * 7.0);
    let intersections_far = find_intersections(&mouse_position, &ball_position, (radius * 7.0) + 150.0);

    let m = (ball_position.y - mouse_position.y) / (ball_position.x - mouse_position.x);
    let c = ((m * mouse_position.x) - (mouse_position.y)) * -1.0;


    let y = |x: f64| {
        (m * x) + c
    };

    let to_screen_point = |x: &f64| {
        ScreenPoint2D { x:*x, y: y(*x) }
    };

    let screen_points_close : Vec<Complex> = intersections_close
        .iter()
        .map(&to_screen_point)
        .map(|p| p.as_complex())
        .collect();

    let chosen_point_close = mouse_position.as_complex().find_closest(screen_points_close.as_slice());

    let screen_points_far : Vec<Complex> = intersections_far
        .iter()
        .map(&to_screen_point)
        .map(|p| p.as_complex())
        .collect();

    let chosen_point_far = mouse_position.as_complex().find_closest(screen_points_far.as_slice());

    [chosen_point_close.x, chosen_point_close.y, chosen_point_far.x, chosen_point_far.y]
}

