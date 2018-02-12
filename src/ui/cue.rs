use complex::*;
use ui::cue_line_calculator::*;
use geometry::*;

pub fn get_cue_line(mouse_position: &Point2D,
                    ball_position: &Point2D,
                    tip_distance_from_cueball: f64,
                    table: &ScreenRectangle) -> CueLine {

    let cue_line_params = CueLineParams {
        mouse_position, ball_position,
        tip_distance_from_cueball,
        table
    };

    CueLine::calculate(&cue_line_params)
}