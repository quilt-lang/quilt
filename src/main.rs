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

fn main() {
    parse(pixels("examples/hello_world_x20.png", 20).unwrap());
    let program = parse(pixels("examples/hello_world.png", 1).unwrap());
    let mut vm = VM::new();
    vm.execute(program);
}
