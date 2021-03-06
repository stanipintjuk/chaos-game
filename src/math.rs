use nalgebra::{Vector2 as Vec2, Matrix1};
use std::vec::Vec;
use rand;
use rand::Rng;

pub type Vector2 = Vec2<f64>;
pub type Shape = Vec<Vector2>;

fn get_random_point(shape: &Shape) -> Vector2 {
    rand::thread_rng().choose(shape).expect("could not pick a random vector from shape").clone()
}

pub fn get_next_point(shape: &Shape, pos: Vector2, divisor: Matrix1<f64>) -> Vector2 {
    let dir_vec = get_random_point(shape);
    let walk = dir_vec - pos;
    let short_walk = walk * divisor;
    pos + short_walk
}

fn should_switch(chance: u32) -> bool {
    let r = rand::thread_rng().gen::<u32>();
    r % chance == 0
}

pub fn get_alternating_paths(shapes: &[&Shape], start: Vector2, divide_by: f64, iterations: usize, chance: u32) -> Vec<Shape> {
    let mut pos = start;
    let mut paths = vec![];
    for _ in shapes {
        paths.push(vec![]);
    }

    let divisor = Matrix1::new(divide_by);
    let mut shape_i = 0usize;

    for _ in 1..iterations {
        if should_switch(chance) {
            let r = rand::thread_rng().gen::<usize>();
            shape_i =  r % shapes.len();
        }

        let next = get_next_point(shapes[shape_i] , pos, divisor);
        paths[shape_i].push(next);
        pos = next;
    }

    paths
}

pub fn get_alternating_path(shapes: &[&Shape], start: Vector2, divide_by: f64, iterations: usize, chance: u32) -> Shape {
    let mut pos = start;
    let mut path = vec![];

    let divisor = Matrix1::new(divide_by);
    let mut shape = shapes[0];

    for _ in 1..iterations {
        if should_switch(chance) {
            let i = rand::thread_rng().gen::<usize>() % shapes.len();
            shape = shapes[i];
        }

        let next = get_next_point(shape, pos, divisor);
        path.push(next);
        pos = next;
    }

    path
}

pub fn get_path(shape: &Shape, start: Vector2, divide_by: f64, iterations: usize) -> Shape {

    let mut pos = start;
    let mut path = vec![];

    let divisor = Matrix1::new(divide_by);

    for _ in 1..iterations {
        let next = get_next_point(&shape, pos, divisor);
        path.push(next);
        pos = next;
    }

    path
}
