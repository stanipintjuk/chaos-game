use nobs::*;

/**
 * This file contains example for how to use the "simpler" NoBS API
 *
 * NoBS API is designed for easier use and less mental overhead and 
 * should be used when you just want to play around with sh apes and colors.
 * 
 * If you want more control then you can checkout the regular examples later
 */

#[allow(dead_code)]
pub fn example1() {
    // Create the shapes by simply writing the points
    let tri1 = shape![
        (0.0, 0.0),
        (0.3, 0.3),
        (0.7, 0.0)
    ];

    let tri2 = shape![
        (0.3, 1.0),
        (1.0, 1.0),
        (1.0, 0.3)
    ];

    let paths = getPaths(&[ &tri1, &tri2, ], 0.50, 100000, 16);

    // Get a figure to draw on, with a specific color
    let mut fg = getFigureCl(Cl::Black);

    // Render the shapes with a specific color
    for path in paths {
        fg.renderCl(path, Cl::Red);
    }

    fg.renderCl(tri1, Cl::Blue);
    fg.renderCl(tri2, Cl::Green);

    // show the window
    fg.show();
}

pub fn example2() {
    let squ1 = shape![
        (0.3, 0.3),
        (0.3, 0.6),
        (0.6, 0.3),
        (0.6, 0.6)
    ];

    let squ2 = shape![
        (0.5, 0.5),
        (0.5, 0.7),
        (0.7, 0.5),
        (0.7, 0.7)
    ];

    let paths = getPaths(
        &[
        &squ1,
        &squ2,
        ], 
        0.60, 
        1_000_000,
        8);

    let mut fg = getFigure4k(Cl::Black);

    let path1 = paths[0].clone();
    let path2 = paths[1].clone();
    fg.renderCl(path2, Cl::Other("#231060"));
    fg.renderCl(path1, Cl::Blue);

    fg.show();
}

pub fn example3() {
    let t1 = shape![
        (0.0, 0.5),
        (1.0/6.0, 1.0),
        (0.5, 0.5)
    ];

    let t2 = shape![
        (1.0/6.0, 1.0),
        (5.0/6.0, 1.0),
        (0.5, 0.5)
    ];

    let t3 = shape![
        (0.5, 0.5),
        (5.0/6.0, 1.0),
        (1.0, 0.5)
    ];

    let t4 = shape![
        (0.5, 0.5),
        (1.0, 0.5),
        (5.0/6.0, 0.0)
    ];

    let t5 = shape![
        (1.0/6.0, 0.0),
        (0.5, 0.5),
        (5.0/6.0, 0.0)
    ];

    let t6 = shape![
        (0.0, 0.5),
        (0.5, 0.5),
        (1.0/6.0, 0.0)
    ];

    let paths  = getPaths(
        &[
        &t1,
        &t2,
        &t3,
        &t4,
        &t5,
        &t6
        ],
        0.75,
        100_000_000,
        16
        );

    let colors = vec![
        Cl::Red,
        Cl::Blue,
        Cl::White,
        Cl::Green,
        Cl::Purple,
        Cl::Yellow
    ];

    let mut fg = getFigure4k(Cl::Black);

    for (path, color) in paths.iter().zip(colors) {
        fg.renderCl(path.clone(), color);
    }

    fg.show();
}

pub fn example4() {
    let p = shape![
        (1.0/6.0, 0.0),
        (0.0, 0.5),
        (1.0/6.0, 1.0),
        (5.0/6.0, 1.0),
        (1.0, 0.5),
        (5.0/6.0, 0.0),
        (0.5, 0.5)
    ];

    let path = getPath(&p, 0.75, 100_000_000);

    let mut fg = getFigureBeyond4k(Cl::Black);

    fg.renderCl(path, Cl::White);

    fg.show();
}

