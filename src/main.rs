extern crate gnuplot;
extern crate nalgebra;
extern crate rand;

mod render;
mod math;
mod shapes;
mod examples;

fn main() {
    examples::sierpinski_triangle();
    //examples::squares();
    //examples::alternate_two_shapes();
    //examples::alternate_with_color();
    //examples::generate_4k_image();
}

