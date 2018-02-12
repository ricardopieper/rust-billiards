use entities::*;
use opengl_graphics::GlGraphics;
use graphics::*;
use rendering::drawing::rgb;
use ui::cue;
use ui::CueLine;

pub fn render_cue(pool: &Pool, c: &Context, gl: &mut GlGraphics) {
    let color = rgb(255.0, 0.0, 0.0, 1.0);

    let ref mouse_pos = pool.mouse_pos;
    let ref ball_pos = pool.cueball.position.to_screen_point_from_rect(&pool.play_area);
    let distance_from_cueball = (pool.cueball.radius * 3.0) * pool.play_area.width;

    let cue_line = cue::get_cue_line(
       mouse_pos, ball_pos, distance_from_cueball, &pool.play_area
    );

    line(color, 3.0, to_f64_coordinates(cue_line), c.transform, gl);
}

fn to_f64_coordinates(cue_line: CueLine) -> [f64; 4] {
    [cue_line.tip.x, cue_line.tip.y, cue_line.grip.x, cue_line.grip.y]
}