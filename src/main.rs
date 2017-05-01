extern crate gnuplot;
extern crate nalgebra;
extern crate rand;
use gnuplot::*;

mod render;
use render::*;
mod math;
use math::*;
mod shapes;
use shapes::point;
mod examples;

fn main() {
    examples::sierpinski_triangle();
    //examples::alternate_two_shapes();
    //examples::alternate_with_color();
    //examples::generate_4k_image();
}

