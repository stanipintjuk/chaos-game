use math::*;
use nobs::*;

/**
 * This file contains example for how to use the "simpler" NoBS API
 *
 * NoBS API is designed for easier use and less mental overhead and 
 * should be used when you just want to play around with sh apes and colors.
 * 
 * If you want more control then you can checkout the regular examples later
 */

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

    let paths = getPaths(&[ &tri1, &tri2, ], 0.5, 100000);

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
