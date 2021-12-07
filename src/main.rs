use quilt;
use quilt::vm::{Pixel, VM};

fn main() {
    let program: Vec<Vec<Pixel>> = vec![
        vec![
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
        ],
    ];

    let vm = VM::new(program);
    vm.execute();
}
