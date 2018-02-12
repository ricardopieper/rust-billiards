use complex::*;
use utils::math;
use entities::*;
use geometry::*;

#[derive(Debug)]
pub struct CueLine {
    pub tip: ScreenPoint2D,
    pub grip: ScreenPoint2D
}

#[derive(Debug)]
pub struct CueLineParams<'a> {
    pub mouse_position: &'a Point2D,
    pub ball_position: &'a Point2D,
    pub table: &'a ScreenRectangle,
    pub tip_distance_from_cueball: f64
}

impl CueLine {

    pub fn calculate(cue_line_params: &CueLineParams) -> CueLine {

        let mouse_pos = cue_line_params.mouse_position;
        let ball_pos = cue_line_params.ball_position;

        let line_vertices = if (mouse_pos).is_horizontally_aligned(ball_pos ) {
            CueLine::get_straight_vertical_cue_line(cue_line_params)
        } else {
            CueLine::calculate_cue_line(cue_line_params)
        };

        CueLine {
            tip: ScreenPoint2D::new(line_vertices[0], line_vertices[1]),
            grip: ScreenPoint2D::new(line_vertices[2], line_vertices[3])
        }
    }

    pub fn get_shot_vector(mouse_pos: &Point2D, ball_pos: &Point2D) -> Vector2D {
        let slope = math::calculate_slope(
            ball_pos,
            mouse_pos);

        let y_intercept = math::calculate_y_intercept(mouse_pos, slope);
        let x = if mouse_pos.is_at_left(ball_pos) { 0.0 } else { 1.0 };

        let y = slope * x + y_intercept;

        let target = if f64::is_nan(y) {
            Point2D::new(x, 0.0)
        } else {
            Point2D::new(x, y)
        };

        Vector2D::new(target.x - ball_pos.x, target.y - ball_pos.y).normalize()
    }

    fn get_cue_length(params: &CueLineParams) -> f64 {
        params.table.aspect() / 4.0
    }

    fn get_straight_vertical_cue_line(params: &CueLineParams) -> [f64; 4] {

        let mouse_pos = params.mouse_position.to_screen_point_relative(params.table);
        let ball_pos = params.ball_position.to_screen_point_relative(params.table);

        let cue_size = CueLine::get_cue_length(params);

        let signal = if mouse_pos.is_above(&ball_pos) { -1.0 } else { 1.0 };

        let distance = params.tip_distance_from_cueball;

        [mouse_pos.x,
            ball_pos.y + (distance * signal),
            mouse_pos.x,
            ball_pos.y + ((distance + cue_size) * signal)]
    }

    fn calculate_cue_line(params: &CueLineParams) -> [f64; 4] {
        let mouse_position = params.mouse_position.to_screen_point_relative(params.table);
        let ball_position = params.ball_position.to_screen_point_relative(params.table);

        let cue_size = CueLine::get_cue_length(params);

        let slope = math::calculate_slope(
            &ball_position,
            &mouse_position);

        let y_intercept = math::calculate_y_intercept(&mouse_position, slope);

        let line = Line { y_intercept, slope };

        let distance = params.tip_distance_from_cueball;

        let intersections_tip = math::intersections(&ball_position, distance, &line);
        let intersections_end = math::intersections(&ball_position, distance + cue_size, &line);

        let cue_tip = CueLine::get_cue_extremity(&intersections_tip, &line, &mouse_position);
        let cue_end = CueLine::get_cue_extremity(&intersections_end, &line, &mouse_position);

        return [cue_tip.x, cue_tip.y, cue_end.x, cue_end.y];
    }

    fn get_cue_extremity(x_points: &[f64; 2], line: &Line, mouse_position: &ScreenPoint2D) -> ScreenPoint2D {
        let screen_points: Vec<ScreenPoint2D> = x_points
            .iter()
            .map(|x| ScreenPoint2D::new(*x, line.y(*x)))
            .collect();

        let closest_to_mouse = mouse_position.find_farthest(screen_points.as_slice());

        ScreenPoint2D::new(closest_to_mouse.x, closest_to_mouse.y)
    }
}