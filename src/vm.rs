use crate::{Instruction, Pixel};
use crate::{Matrix, MatrixPoint};

const TAPE_SIZE: usize = 360;

pub struct VM {
    stack: Vec<u16>,
    register_a: u16,
    tape: [u16; TAPE_SIZE],
    direction: Direction,
    instructions: Matrix<Pixel>,
    pc: MatrixPoint,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::West => Direction::East,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
        }
    }

    pub fn counter_clockwise(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
}

impl Default for VM {
    fn default() -> VM {
        VM {
            stack: vec![],
            register_a: 0,
            tape: [0; TAPE_SIZE],
            direction: Direction::East,
            instructions: Matrix::new(vec![]),
            pc: MatrixPoint(0, 0),
        }
    }
}

impl VM {
    pub fn new() -> VM {
        VM::default()
    }

    pub fn execute(&mut self, instructions: Matrix<Pixel>) {
        self.instructions = instructions;
        self.pc = self.find_start();

        loop { // TODO change condition
        }
    }

    // prioritize roads over all other instructions besides the one in front of us
    pub fn get_next_instruction(&self) -> Pixel {
        let next_pixels = self.get_next_pixels();
        let (first_dir, first_pixel) = next_pixels[0];
        let first_road = next_pixels
            .iter()
            .filter(|(_dir, pixel)| matches!(pixel.as_instruction(), Instruction::Road))
            .next();
        if let Some((dir, road)) = first_road {
            if *dir != self.direction.opposite() {
                return *road;
            } else if first_dir == self.direction {
                return first_pixel;
            }
        }
        first_pixel
    }

    // try the pixel ahead of us. If that doesn't exist,
    // try the pixel to the 'right' (counter-clockwise & opposite). If that doesn't exist,
    // try the pixel to the 'left' (counter-clockwise). If that doesn't exist,
    // go back the way we came
    fn get_next_pixels(&self) -> Vec<(Direction, Pixel)> {
        let ins = &self.instructions;
        let dir = self.direction;
        let mut next_pixels = vec![];
        if let Some(point) = ins.go(self.pc, dir) {
            // forward
            next_pixels.push((dir, point));
        }
        if let Some(point) = ins.go(self.pc, dir.counter_clockwise().opposite()) {
            // right
            next_pixels.push((dir.counter_clockwise().opposite(), point));
        }
        if let Some(point) = ins.go(self.pc, dir.counter_clockwise()) {
            // left
            next_pixels.push((dir.counter_clockwise(), point));
        }
        if let Some(point) = ins.go(self.pc, dir.opposite()) {
            // back
            next_pixels.push((dir.opposite(), point));
        }
        next_pixels
    }

    fn get_next_pixel(&self) -> (Direction, Pixel) {
        self.get_next_pixels()[0]
    }

    fn find_start(&self) -> MatrixPoint {
        for (row_idx, row) in self.instructions.matrix.iter().enumerate() {
            for (col_idx, pixel) in row.iter().enumerate() {
                if pixel.as_instruction() == Instruction::Start {
                    return MatrixPoint(row_idx, col_idx);
                }
            }
        }

        // default start coordinates
        MatrixPoint(0, 0)
    }
}
