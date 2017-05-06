use gnuplot::{Figure as GnuPlotFigure, PlotOption, AxesCommon, Fix, Below, Color, FillRegion};
use nalgebra::{Vector2 as Vec2};
use std::vec::Vec;

type Vector2 = Vec2<f64>;

pub struct Figure<'l> {
    fg: GnuPlotFigure,
    bgcolor: &'l str,
    x_range: (f64, f64),
    y_range: (f64, f64),
    is_4k: bool,
    is_b4k: bool
}

impl <'l>Figure<'l> {
    pub fn new() -> Figure<'l> {
        Figure{
            fg : GnuPlotFigure::new(),
            bgcolor : "white",
            x_range : (0.0, 1.0),
            y_range : (0.0, 1.0),
            is_4k: false,
            is_b4k: true,
        }
    }

    pub fn new4k() -> Figure<'l> {
        let mut fg = Figure{
            fg : GnuPlotFigure::new(),
            bgcolor : "white",
            x_range : (-0.38888888888888884, 1.0 + 0.38888888888888884),
            y_range : (0.0, 1.0),
            is_4k: true,
            is_b4k: false,
        };

        stdout_in_4k(&mut fg);

        fg
    }

    pub fn new_beyond_4k() -> Figure<'l> {
        let mut fg = Figure{
            fg : GnuPlotFigure::new(),
            bgcolor : "white",
            x_range : (-0.38888888888888884, 1.0 + 0.38888888888888884),
            y_range : (0.0, 1.0),
            is_4k: false,
            is_b4k: true,
        };

        stdout_in_beyond_4k(&mut fg);

        fg
    }
    
    pub fn set_bg(&mut self, color: &'l str) {
        self.bgcolor = color;
        set_bg(self);

        if self.is_4k {
            stdout_in_4k(self)
        } else if self.is_b4k {
            stdout_in_beyond_4k(self)
        }
    }

    pub fn show(&mut self) {
        self.fg.show();
    }
}

pub fn render<'a, T: Iterator<Item=&'a Vector2>>(points: T, fg: &mut Figure, options: &[PlotOption]) {

    let mut xs = Vec::new();
    let mut ys = Vec::new();

    for point in points {
        xs.push(point.x.clone());
        ys.push(point.y.clone());
    }

    fg.fg.axes2d()
        .set_border(false, &[], &[])
        .set_x_range(Fix(fg.x_range.0), Fix(fg.x_range.1))
        .set_y_range(Fix(fg.y_range.0), Fix(fg.y_range.1))
        .set_x_ticks(None, &[], &[])
        .set_y_ticks(None, &[], &[])
        .points(xs, ys, options);
}

fn set_bg(fg: &mut Figure) {
    fg.fg.axes2d()
        .set_border(false, &[], &[])
        .set_x_range(Fix(fg.x_range.0), Fix(fg.x_range.1))
        .set_y_range(Fix(fg.y_range.0), Fix(fg.y_range.1))
        .set_x_ticks(None, &[], &[])
        .set_y_ticks(None, &[], &[])
        .boxes(&[0.0, 0.0, 1.0, 1.0], 
               &[0.0, 1.0, 1.0, 0.0], 
               &[Color(fg.bgcolor), FillRegion(Below)]);
}

fn stdout_in_4k(fg: &mut Figure) {

    let msg = "pngcairo size 3840, 2160 background rgb '".to_owned();
    let msg = msg + fg.bgcolor;
    let msg = msg +  "'";
    fg.fg.set_terminal(&msg, "");
}

fn stdout_in_beyond_4k(fg: &mut Figure) {
    let msg = "pngcairo size 30720, 17280 background rgb '".to_owned();
    let msg = msg + fg.bgcolor;
    let msg = msg +  "'";
    fg.fg.set_terminal(&msg, "");
}
