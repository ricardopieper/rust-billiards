use complex::*;
use ui::cue_line_calculator::*;

pub fn get_cue_line(mouse_pos: &ScreenPoint2D,
                    ball_pos: &ScreenPoint2D,
                    distance_from_cueball: f64,
                    window_width: f64,
                    window_height: f64) -> CueLine {

    let cue_line_params = CueLineParams {
        mouse_position: mouse_pos,
        ball_position: ball_pos,
        window_width: window_width as f64,
        window_height: window_height as f64,
        tip_distance_from_cueball_center: distance_from_cueball
    };

    CueLine::calculate(&cue_line_params)
}