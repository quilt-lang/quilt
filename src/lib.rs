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

pub fn run(file: &str, pixel_size: u32) {
    let program = parse(pixels(file, pixel_size).unwrap());
    let mut vm = VM::new();
    vm.execute(program);
}
