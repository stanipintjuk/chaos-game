extern crate gnuplot;
extern crate nalgebra;
extern crate rand;
use gnuplot::*;
use nalgebra as na;
use nalgebra::{Vector2 as Vec2, Dim, Matrix1};
use std::vec::Vec;
use rand::Rng;

// Constant rules
const PART_OF_WAY: f64 = 4.0 / 7.0;
//const PART_OF_WAY: f64 = 1.0 / 2.0;
// interesting stuff will happen if this is less than 1.0 and more than 0.5
const ITERATIONS: usize = 10000000;

type Vector2 = Vec2<f64>;
type Shape = Vec<Vector2>;

fn get_random_point(shape: &Shape) -> Vector2 {
    rand::thread_rng().choose(shape).expect("could not pick a random vector from shape").clone()
}

fn get_next_point(shape: &Shape, pos: Vector2, divisor: Matrix1<f64>) -> Vector2 {
    let dir_vec = get_random_point(shape);
    let walk = dir_vec - pos;
    let short_walk = walk * divisor;
    pos + short_walk
}

fn should_switch() -> bool {
    let r = rand::thread_rng().gen::<u32>();
    r % 16 == 0
}

fn get_path2_with_color<'a>(shapes: &[(&Shape, &'a str)], start: Vector2) -> Vec<(Shape, &'a str)> {
    let mut pos = start;
    let mut paths = vec![];
    for &(_, color) in shapes {
        paths.push((vec![], color));
    }

    let divisor = Matrix1::new(PART_OF_WAY);
    let mut shape_i = 0usize;

    for _ in 1..ITERATIONS {
        if should_switch() {
            let r = rand::thread_rng().gen::<usize>();
            shape_i =  r % shapes.len();
        }

        let next = get_next_point(shapes[shape_i].0 , pos, divisor);
        paths[shape_i].0.push(next);
        pos = next;
    }

    paths
}

fn get_path2(shapes: &[&Shape], start: Vector2) -> Shape {
    let mut pos = start;
    let mut path = vec![];

    let divisor = Matrix1::new(PART_OF_WAY);
    let mut shape = shapes[0];

    for _ in 1..ITERATIONS {
        if should_switch() {
            let i = rand::thread_rng().gen::<usize>() % shapes.len();
            shape = shapes[i];
        }

        let next = get_next_point(shape, pos, divisor);
        path.push(next);
        pos = next;
    }

    path
}

fn get_path(shape: &Shape, start: Vector2) -> Shape {

    let mut pos = start;
    let mut path = vec![];

    let divisor = Matrix1::new(PART_OF_WAY);

    for _ in 1..ITERATIONS {
        let next = get_next_point(&shape, pos, divisor);
        path.push(next);
        pos = next;
    }

    path
}

fn point(x: f64, y: f64) -> Vector2 {
    Vector2::new(x, y)
}

fn single() {
    let triangle = vec![
        Vector2::new(0.0, 0.0),
        Vector2::new(0.5, 1.0),
        Vector2::new(1.0, 0.0),
    ];

    let square = vec![
        Vector2::new(0.0, 0.0),
        Vector2::new(0.0, 1.0),
        Vector2::new(1.0, 0.0),
        Vector2::new(1.0, 1.0),
    ];

    let pentagon = vec![
        point(0.25, 0.0),
        point(0.05, 0.6),
        point(0.5, 1.0),
        point(0.95, 0.6),
        point(0.75, 0.0),
    ];

    let hexagon = vec![
        point(0.30, 0.0),
        point(0.0, 0.5),
        point(0.30, 1.0),
        point(0.70, 1.0),
        point(1.0, 0.5),
        point(0.70, 0.0),
    ];

    let shape = triangle;
    let start = Vector2::new(0.0, 0.0);
    let path = get_path(&shape, start);

    let red = &[Color("red"), PointSymbol('O')];
    let black = &[Color("black"), PointSymbol('O'), PointSize(0.1)];
    let orange = &[Color("orange"), PointSymbol('O')];

    let mut fg = Figure::new();
    render(shape.iter(), &mut fg, red);
    render((&[start]).iter(), &mut fg, orange);
    render(path.iter(), &mut fg, black);
    fg.show();
}

fn multiple() {

    let tri1 = vec![
        point(0.0, 0.0),
        point(0.0, 0.3),
        point(0.2, 0.0),
    ];

    let tri2 = vec![
        point(0.7, 1.0),
        point(1.0, 1.0),
        point(1.0, 0.4),
    ];

    let squ = vec![
        point(0.0, 0.0),
        point(0.0, 0.2),
        point(0.2, 0.0),
        point(0.2, 0.2),
    ];

    let start = Vector2::new(0.0, 0.0);
    let path = get_path2(&[&squ, &tri2], start);

    let red = &[Color("red"), PointSymbol('O')];
    let blue = &[Color("blue"), PointSymbol('O')];
    let black = &[Color("black"), PointSymbol('O'), PointSize(0.1)];
    let orange = &[Color("orange"), PointSymbol('O')];

    let mut fg = Figure::new();
    render(squ.iter(), &mut fg, red);
    render(tri2.iter(), &mut fg, blue);
    render((&[start]).iter(), &mut fg, orange);
    render(path.iter(), &mut fg, black);
    fg.show();
}

fn multiple_color() {

    let tri1 = vec![
        point(0.0, 0.6),
        point(0.0, 1.0),
        point(0.2, 1.0),
    ];

    let tri2 = vec![
        point(0.7, 1.0),
        point(1.0, 1.0),
        point(1.0, 0.4),
    ];

    let squ = vec![
        point(0.0, 0.0),
        point(0.0, 0.2),
        point(0.2, 0.0),
        point(0.2, 0.2),
    ];

    let start = Vector2::new(0.0, 0.0);
    let paths = get_path2_with_color(
        &[
        (&squ, "red"),
        (&tri1, "green"),
        (&tri2, "blue"),
        ], 
        start);

    let point = PointSymbol('O');
    let tiny = PointSize(0.1);

    let mut fg = Figure::new();
    for (path, color) in paths {
        render(path.iter(), &mut fg, &[Color(color), point, tiny])
    }
    fg.show();
}

fn playing_around() {

    let tri1 = vec![
        point(0.6, 0.0),
        point(0.8, 0.1),
        point(1.0, 1.0),
    ];

    let tri2 = vec![
        point(0.2, 0.4),
        point(0.25, 0.1),
        point(0.5, 0.0),
    ];

    let col1 = "#370E61";
    let col2 = "#3D0002";
    let col3 = "#8F0A0E";

    let start = Vector2::new(0.0, 0.0);
    let paths = get_path2_with_color(
        &[
        (&tri1, col2),
        (&tri2, "#881B05"),
        ], 
        start);

    let point = PointSymbol('O');
    let tiny = PointSize(0.01);


    let mut fg = Figure::new();
    set_bg(&mut fg, "#000000");
    for (path, color) in paths {
        render(path.iter(), &mut fg, &[Color(color), point, tiny])
    }
    fg.set_terminal("pngcairo size 3840, 2160 background rgb 'black'", "");
    fg.show();
}

fn main() {
    playing_around();
}

fn render<'a, T: Iterator<Item=&'a Vector2>>(points: T, fg: &mut Figure, options: &[PlotOption]) {

    let mut xs = Vec::new();
    let mut ys = Vec::new();

    for point in points {
        xs.push(point.x.clone());
        ys.push(point.y.clone());
    }

    //fg.axes2d().set_pos_grid(2, 2, 1).points(xs, ys, options);
    fg.axes2d()
        .set_x_range(Fix(0.0), Fix(1.0))
        .set_y_range(Fix(0.0), Fix(1.0))
        .set_x_axis(false, &[])
        .set_y_axis(false, &[])
        .points(xs, ys, options);
}

fn set_bg(fg: &mut Figure, color: &str) {
    fg.axes2d()
        .set_x_range(Fix(0.0), Fix(1.0))
        .set_y_range(Fix(0.0), Fix(1.0))
        .set_x_axis(false, &[])
        .set_y_axis(false, &[])
        .boxes(&[0.0, 0.0, 1.0, 1.0], &[0.0, 1.0, 1.0, 0.0], &[Color(color)]);
}
