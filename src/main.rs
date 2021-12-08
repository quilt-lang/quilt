mod hsl;
mod instruction;
mod matrix;
mod parser;
mod pixel;
mod vm;

pub use instruction::Instruction;
pub use matrix::{Matrix, MatrixPoint};
pub use pixel::Pixel;
pub use vm::VM;

use parser::{parse, pixels};

fn main_old() {
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
fn main() {
    parse(pixels("examples/hello_world.png").unwrap());
}
