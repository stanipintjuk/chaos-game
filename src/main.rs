extern crate gnuplot;
extern crate nalgebra;
extern crate rand;
use gnuplot::*;
use nalgebra as na;
use nalgebra::{Vector2 as Vec2, Dim, Matrix1};
use std::vec::Vec;

mod render;
use render::*;
mod math;
use math::*;

// Constant rules
//const PART_OF_WAY: f64 = 4.0 / 7.0;
const PART_OF_WAY: f64 = 1.0 / 2.0;
// interesting stuff will happen if this is less than 1.0 and more than 0.5
const ITERATIONS: usize = 100000;

type Vector2 = Vec2<f64>;
type Shape = Vec<Vector2>;

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
    let path = get_path(&shape, start, PART_OF_WAY, ITERATIONS);

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
    let path = get_path2(&[&squ, &tri2], start, PART_OF_WAY, ITERATIONS);

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
        start,
        PART_OF_WAY,
        ITERATIONS);

    let point = PointSymbol('O');
    let tiny = PointSize(0.1);

    let mut fg = Figure::new();
    for (path, color) in paths {
        render(path.iter(), &mut fg, &[Color(color), point, tiny])
    }
    fg.show();
}

fn first4000Kimg() {

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
        start,
        PART_OF_WAY,
        ITERATIONS);

    let point = PointSymbol('O');
    let tiny = PointSize(0.01);


    let mut fg = Figure::new();
    set_bg(&mut fg, "#000000");
    for (path, color) in paths {
        render(path.iter(), &mut fg, &[Color(color), point, tiny])
    }
    //fg.set_terminal("pngcairo size 3840, 2160 background rgb 'black'", "");
    fg.show();
}

fn playing_around2() {

    let botleft = vec![
        point(0.0, 0.0),
        point(0.0, 0.5),
        point(0.5, 0.0),
    ];
    
    let botright = vec![
        point(0.5, 0.0),
        point(1.0, 0.0),
        point(1.0, 0.5),
    ];

    let topleft = vec![
        point(0.0, 0.5),
        point(0.0, 1.0),
        point(0.5, 1.0),
    ];

    let topright = vec![
        point(0.5, 1.0),
        point(1.0, 1.0),
        point(1.0, 0.5),
    ];

    let red = "red";
    let blue = "blue";
    let green = "green";
    let yellow = "yellow";

    let start = Vector2::new(0.0, 0.0);
    let paths = get_path2_with_color(
        &[
        (&botleft, blue),
        (&botright, red),
        (&topleft, green),
        (&topright, yellow),
        ], 
        start,
        PART_OF_WAY,
        ITERATIONS);

    let point = PointSymbol('O');
    let tiny = PointSize(0.01);

    let mut fg = Figure::new();
    set_bg(&mut fg, "#000000");
    for (path, color) in paths {
        render(path.iter(), &mut fg, &[Color(color), point, tiny])
    }
    render(botleft.iter(), &mut fg, &[Color(red), point]);
    render(botright.iter(), &mut fg, &[Color(blue), point]);
    render(topleft.iter(), &mut fg, &[Color(green), point]);
    render(topright.iter(), &mut fg, &[Color(yellow), point]);
    //fg.set_terminal("pngcairo size 3840, 2160 background rgb 'white'", "");
    fg.show();
}

fn main() {
    playing_around2()
}

