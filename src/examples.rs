use shapes;
use render::{Figure, render};
use math::*;
use gnuplot::{PointSymbol, Color, PointSize};
use shapes::*;

/**
 * This example will generate the Sierpinski triangle (the fractal that looks like the triforce)
 */
#[allow(dead_code)]
pub fn sierpinski_triangle() {
    // first create a shape. In this case we use a triangle
    let shape = shapes::triangle();

    // Create a starting point (doesn't really matter what it is)
    let start = Vector2::new(0.0, 0.0);
    // generate the points
    let path = get_path(&shape, start, 0.5, 100000);

    // create some rendering options for Gnuplot
    let red = &[Color("red"), PointSymbol('O')];
    let black = &[Color("black"), PointSymbol('O'), PointSize(0.1)];
    let orange = &[Color("orange"), PointSymbol('O')];

    // render the shapes
    let mut fg = Figure::new();
    render(shape.iter(), &mut fg, red);
    render((&[start]).iter(), &mut fg, orange);
    render(path.iter(), &mut fg, black);

    // and at last show the window
    fg.show();
}

/**
 * This example generates a cool looking square fractal.
 */
#[allow(dead_code)]
pub fn squares() {
    // first create a shape. In this case we use a triangle
    let shape = shapes::square();

    // Create a starting point (doesn't really matter what it is)
    let start = Vector2::new(0.0, 0.0);
    // generate the points. Note that this time the divisor is 0.75, while in the example above it
    // was 0.5
    let path = get_path(&shape, start, 0.75, 100000);

    // create some rendering options for Gnuplot
    let red = &[Color("red"), PointSymbol('O')];
    let black = &[Color("black"), PointSymbol('O'), PointSize(0.1)];
    let orange = &[Color("orange"), PointSymbol('O')];

    // render the shapes
    let mut fg = Figure::new();
    render(shape.iter(), &mut fg, red);
    render((&[start]).iter(), &mut fg, orange);
    render(path.iter(), &mut fg, black);

    // and at last show the window
    fg.show();
}

/**
 * This example will generate points while randomly alternating between two different shapes
 * */
#[allow(dead_code)]
pub fn alternate_two_shapes() {

    // As usual, create the shapes
    let tri = vec![
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

    // create a starting point and generate the points
    let start = Vector2::new(0.0, 0.0);
    let path = get_alternating_path(&[&squ, &tri], start, 0.75, 100000, 16);

    // define the Gnuplot options
    let red = &[Color("red"), PointSymbol('O')];
    let blue = &[Color("blue"), PointSymbol('O')];
    let black = &[Color("black"), PointSymbol('O'), PointSize(0.1)];
    let orange = &[Color("orange"), PointSymbol('O')];

    // render the shapes
    let mut fg = Figure::new();
    render(squ.iter(), &mut fg, red);
    render(tri.iter(), &mut fg, blue);
    render((&[start]).iter(), &mut fg, orange);
    render(path.iter(), &mut fg, black);
    fg.show();
}

/** 
 * This example does the same thing as the example above,
 * but keeps the informations about which points are related to which shape.
 * This is cool because you can color those points in different colors.
 * */
#[allow(dead_code)]
pub fn alternate_with_color() {

    // Create 4 triangles for the 4 different corners of the canvas
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

    // define the colors
    let red = "red";
    let blue = "blue";
    let green = "green";
    let yellow = "yellow";

    let colors = vec![red, blue, green, yellow];

    // create the starting poin and generate the points
    let start = Vector2::new(0.0, 0.0);
    let paths = get_alternating_paths(
        &[
        &botleft,
        &botright,
        &topleft,
        &topright,
        ], 
        start,
        0.5,
        100000,
        16);

    // define some Gnuplot stuff to make the plot look nicer
    let point = PointSymbol('O');
    let tiny = PointSize(0.01);

    // render all the points
    let mut fg = Figure::new();
    fg.set_bg("#000000");
    for (path, color) in paths.iter().zip(colors) {
        render(path.iter(), &mut fg, &[Color(color), point, tiny])
    }
    render(botleft.iter(), &mut fg, &[Color(red), point]);
    render(botright.iter(), &mut fg, &[Color(blue), point]);
    render(topleft.iter(), &mut fg, &[Color(green), point]);
    render(topright.iter(), &mut fg, &[Color(yellow), point]);

    fg.show();
}

/**
 * This example shows how to generate a 4K image from you beautiful plots
 * */
#[allow(dead_code)]
pub fn generate_4k_image() {
    // Create some shapes
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

    let col2 = "#3D0002";

    let colors = vec![col2, "#881B05"];

    // generate the points
    let start = Vector2::new(0.0, 0.0);
    let paths = get_alternating_paths(
        &[
        &tri1,
        &tri2,
        ], 
        start,
        0.5,
        100000,
        16);

    let point = PointSymbol('O');
    let tiny = PointSize(0.01);
    
    let bgcolor = "#000000";

    // Render the points
    let mut fg = Figure::new4k();
    fg.set_bg(bgcolor);
    for (path, color) in paths.iter().zip(colors) {
        render(path.iter(), &mut fg, &[Color(color), point, tiny])
    }

    fg.show();
}

