use gnuplot::*;
use nalgebra::{Vector2 as Vec2, Dim, Matrix1};
use std::vec::Vec;

type Vector2 = Vec2<f64>;
type Shape = Vec<Vector2>;

pub fn render<'a, T: Iterator<Item=&'a Vector2>>(points: T, fg: &mut Figure, options: &[PlotOption]) {

    let mut xs = Vec::new();
    let mut ys = Vec::new();

    for point in points {
        xs.push(point.x.clone());
        ys.push(point.y.clone());
    }

    fg.axes2d()
        .set_border(false, &[], &[])
        .set_x_range(Fix(0.0), Fix(1.0))
        .set_y_range(Fix(0.0), Fix(1.0))
        .set_x_ticks(None, &[], &[])
        .set_y_ticks(None, &[], &[])
        .points(xs, ys, options);
}

pub fn set_bg(fg: &mut Figure, color: &str) {
    fg.axes2d()
        .set_border(false, &[], &[])
        .set_x_range(Fix(0.0), Fix(1.0))
        .set_y_range(Fix(0.0), Fix(1.0))
        .set_x_ticks(None, &[], &[])
        .set_y_ticks(None, &[], &[])
        .boxes(&[0.0, 0.0, 1.0, 1.0], 
               &[0.0, 1.0, 1.0, 0.0], 
               &[Color(color), FillRegion(Below)]);
}

pub fn stdout_4k(fg: &mut Figure) {
    fg.set_terminal("pngcairo size 3840, 2160 background rgb 'white'", "");
}
