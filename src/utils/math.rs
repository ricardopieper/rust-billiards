use entities::{Circle, Line};
use complex::*;

pub fn intersections<T>(circle_position: &T, circle_radius: f64, line: &Line) -> [f64; 2] where T: Complex {
    //https://math.stackexchange.com/questions/228841/how-do-i-calculate-the-intersections-of-a-straight-line-and-a-circle
    //https://cscheng.info/2016/06/09/calculate-circle-line-intersection-with-javascript-and-p5js.html

    let m = line.slope;
    let c = line.y_intercept;

    let h = circle_position.x();
    let k = circle_position.y();

    let a = 1.0 + m.powi(2); //accounts for (x)^2 and mx^2
    let b = (-h * 2.0) + (m * (c - k)) * 2.0; //accounts for x*h + x * h and m * c-k + m * c-k
    let c = h.powi(2) + (c - k).powi(2) - circle_radius.powi(2); //accounts for the rest of the elements

    let x_pos = (-b + ((b * b) - (4.0 * a * c)).sqrt()) / (2.0 * a);
    let x_neg = (-b - ((b * b) - (4.0 * a * c)).sqrt()) / (2.0 * a);

    [x_pos, x_neg]
}