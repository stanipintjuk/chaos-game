use math::{Shape, Vector2};

pub fn point(x: f64, y: f64) -> Vector2 {
    Vector2::new(x, y)
}

#[allow(dead_code)]
pub fn triangle() -> Shape {
    vec![
        point(0.0, 0.0),
        point(0.5, 1.0),
        point(1.0, 0.0),
    ]
}

#[allow(dead_code)]
pub fn square() -> Shape {
    vec![
        point(0.0, 0.0),
        point(0.0, 1.0),
        point(1.0, 0.0),
        point(1.0, 1.0),
    ]
}

#[allow(dead_code)]
pub fn pentagon() -> Shape {
    vec![
        point(0.25, 0.0),
        point(0.05, 0.6),
        point(0.5, 1.0),
        point(0.95, 0.6),
        point(0.75, 0.0),
    ]
}

#[allow(dead_code)]
pub fn hexagon() -> Shape {
    vec![
        point(0.30, 0.0),
        point(0.0, 0.5),
        point(0.30, 1.0),
        point(0.70, 1.0),
        point(1.0, 0.5),
        point(0.70, 0.0),
    ]
}
