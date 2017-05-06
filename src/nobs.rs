use math::{ Shape, Vector2, get_alternating_paths, get_path };
use render::{render as rend, Figure};
use gnuplot::{PlotOption, Color, PointSize, PointSymbol, FillRegion, Above};
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
                temp_vec.push(newPoint($x, $y));
            )*
            temp_vec
        }
    };
}

pub fn newPoint(x: f64, y: f64) -> Vector2 {
    Vector2::new(x, y)
}

#[derive(Clone)]
pub enum Cl<'l> {
    White,
    Red,
    Blue,
    Green,
    Black,
    Yellow,
    Purple,
    Other(&'l str)
}

fn getColor<'l>(color: Cl<'l>) -> &'l str{
    match color {
        Cl::White => "white",
        Cl::Red => "#FF4132",
        Cl::Blue => "#398EE0",
        Cl::Green => "#70FA35",
        Cl::Black => "black",
        Cl::Yellow => "#FFF44B",
        Cl::Purple => "#9C45FA",
        Cl::Other(color) => color,
    }
}

pub trait Renderer<'l> {
    fn render(&mut self, shape: Shape);
    fn renderCl(&mut self, shape: Shape, color: Cl<'l>);

    fn setBg(&mut self, color: Cl<'l>);
}

impl <'l>Renderer<'l> for Figure<'l> {
    fn render(&mut self, shape: Shape) {
        self.renderCl(shape, Cl::Black)
    }

    fn renderCl(&mut self, shape: Shape, color: Cl<'l>) {

        rend(shape.iter(), self, &[Color(getColor(color)), PointSize(0.0005), PointSymbol('O'), FillRegion(Above)])
    }

    fn setBg(&mut self, color: Cl<'l>) {
        self.set_bg(getColor(color));
    }

}

#[allow(dead_code)]
pub fn getFigure<'l>() -> Figure<'l> {
    Figure::new()
}

#[allow(dead_code)]
pub fn getFigureCl<'l>(bgcolor: Cl<'l>) -> Figure<'l> {
    let mut fg = getFigure();
    fg.setBg(bgcolor);
    fg
}

#[allow(dead_code)]
pub fn getFigure4k<'l>(bgcolor: Cl<'l>) -> Figure<'l> {
    let mut fg = Figure::new4k();
    fg.setBg(bgcolor);
    fg
}

#[allow(dead_code)]
pub fn getFigureBeyond4k<'l>(bgcolor: Cl<'l>) -> Figure<'l> {
    let mut fg = Figure::new_beyond_4k();
    fg.setBg(bgcolor);
    fg
}

#[allow(dead_code)]
pub fn getPaths(shapes: &[&Shape], divisor: f64, iterations: usize, chance: u32) -> Vec<Shape> {

    let start = Vector2::new(0.0, 0.0);
    get_alternating_paths(
        shapes,
        start,
        divisor,
        iterations,
        chance
        )
}

#[allow(dead_code)]
pub fn getPath(shape: &Shape, divisor: f64, iterations: usize) -> Shape {

    let start = Vector2::new(0.0, 0.0);
    get_path(shape, start, divisor, iterations)
}
