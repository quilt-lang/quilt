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

use clap::Parser as ClapParser;
use parser::{parse, pixels};

/// Run a quilt program
#[derive(ClapParser)]
#[clap(about, version, author)]
struct Args {
    /// A quilt program
    file: String,

    /// Pixel size
    #[clap(short, long, default_value_t = 1)]
    pixel_size: u8,
}

fn main() {
    let args = Args::parse();
    let program = parse(pixels(args.file, args.pixel_size as u32).unwrap());
    let mut vm = VM::new();
    vm.execute(program);
}
