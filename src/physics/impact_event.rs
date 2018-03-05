use entities::*;
use geometry::*;
use complex::*;

#[derive(Debug)]
pub struct ImpactEvent {
    pub ball_a: i32,
    pub ball_b: i32,
    pub ball_a_vector: Point2D,
    pub ball_b_vector: Point2D,
    pub energy_transfer_a_to_b: f64,
    pub energy_transfer_b_to_a: f64,
}

impl ImpactEvent {
    pub fn new(ball_a: &Ball, ball_b: &Ball) -> ImpactEvent {

        println!("ball_a: {:?}, ball_b: {:?}", ball_a, ball_b);

        let ball_b_vector = ball_b.position.minus(&ball_a.position)
                                        .normalize();

        let ball_a_vector = ImpactEvent::get_perpendicular(&ball_b_vector, ball_a, ball_b)
                                        .normalize();

        let sine = ball_b_vector.x.abs() / ball_b_vector.magnitude();

        ImpactEvent {
            ball_a: ball_a.number,
            ball_b: ball_b.number,
            ball_a_vector, ball_b_vector,
            energy_transfer_b_to_a: sine,
            energy_transfer_a_to_b: 1.0 - sine
        }
    }

    pub fn get_perpendicular(ball_b_vector: &Vector2D, ball_a: &Ball, ball_b: &Ball) -> Vector2D {
        if ball_a.position.is_at_left(&ball_b.position) {
            ball_b_vector.perpendicular_left()
        } else if ball_a.position.is_at_right(&ball_b.position) {
            ball_b_vector.perpendicular_right()
        } else {
            Vector2D::new(0.0,0.0)
        }
    }
}