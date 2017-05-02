use math::{ Shape, Vector2, get_alternating_paths };
use render::{ render as rend, set_bg, stdout_in_4k };
use gnuplot::{Figure, PlotOption, Color, PointSize, PointSymbol, FillRegion, Above};
use std::vec::Vec;

/**
 * NoBS API
 *
 * This contains helper functions and macros 
 * for less mental overhead when you want to just play around.
 */

#[macro_export]
macro_rules! shape {
    [ $( ( $x:expr, $y:expr ) ),* ] => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(Vector2::new($x, $y));
            )*
            temp_vec
        }
    };
}


#[derive(Clone)]
pub enum Cl {
    White,
    Red,
    Blue,
    Green,
    Black,
    Yellow,
}

fn getColor<'l>(color: Cl) -> &'l str{
    match color {
        Cl::White => "white",
        Cl::Red => "red",
        Cl::Blue => "blue",
        Cl::Green => "green",
        Cl::Black => "black",
        Cl::Yellow => "yellow",
    }
}

pub trait Renderer {
    fn render(&mut self, shape: Shape);
    fn renderCl(&mut self, shape: Shape, color: Cl);

    fn setBg(&mut self, color: Cl);
    fn set4k(&mut self, color: Cl);
}

impl Renderer for Figure {
    fn render(&mut self, shape: Shape) {
        self.renderCl(shape, Cl::Black)
    }

    fn renderCl(&mut self, shape: Shape, color: Cl) {
        rend(shape.iter(), self, &[Color(getColor(color)), PointSize(0.1), PointSymbol('O'), FillRegion(Above)])
    }

    fn setBg(&mut self, color: Cl) {
        set_bg(self, getColor(color));
    }

    fn set4k(&mut self, color: Cl) {
        self.setBg(color.clone());
        stdout_in_4k(self, getColor(color));
    }
}

pub fn getFigure() -> Figure {
    Figure::new()
}

pub fn getFigureCl(bgcolor: Cl) -> Figure {
    let mut fg = getFigure();
    fg.setBg(bgcolor);
    fg
}

pub fn getFigure4k(bgcolor: Cl) -> Figure {
    let mut fg = getFigure();
    fg.set4k(bgcolor);
    fg
}

pub fn getPaths(shapes: &[&Shape], divisor: f64, iterations: usize) -> Vec<Shape> {
    let start = Vector2::new(0.0, 0.0);

    get_alternating_paths(
        shapes,
        start,
        divisor,
        iterations
        )
}
