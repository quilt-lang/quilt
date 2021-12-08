mod hsl;
mod matrix;
mod parser;
mod vm;

use crate::matrix::Matrix;
use crate::parser::parse;
use crate::vm::{Pixel, VM};

fn main() {
    let program = Matrix::new(vec![vec![
        Pixel::new(300),
        Pixel::new(180),
        Pixel::new(180),
        Pixel::new(36),
        Pixel::new(1),
        Pixel::new(36),
        Pixel::new(2),
        Pixel::new(108),
        Pixel::new(36),
        Pixel::new(48),
        Pixel::new(108),
        Pixel::new(306),
    ]]);

    let mut vm = VM::new();
    vm.execute(program);
}

#[allow(unused)]
fn main_old() {
    parse().unwrap();
}
