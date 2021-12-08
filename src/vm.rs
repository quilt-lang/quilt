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

#[derive(Clone, Copy, Debug)]
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

    // prioritize roads over all other instructions
    pub fn get_next_instruction() -> MatrixPoint {
        // TODO get_next_pixel() until we find a road
        // if there is no road, use the first result of get_next_pixel()
        MatrixPoint(0, 0)
    }

    // try the pixel ahead of us. If that doesn't exist,
    // try the pixel to the 'right' (counter-clockwise & opposite). If that doesn't exist,
    // try the pixel to the 'left' (counter-clockwise). If that doesn't exist,
    // go back the way we came
    fn get_next_pixel(&self) -> (Direction, MatrixPoint) {
        let ins = &self.instructions;
        let dir = self.direction;

        if let Some(point) = ins.go(&self.pc, dir) {
            // forward
            (dir, point)
        } else if let Some(point) = ins.go(&self.pc, dir.counter_clockwise().opposite()) {
            // right
            (dir.counter_clockwise().opposite(), point)
        } else if let Some(point) = ins.go(&self.pc, dir.counter_clockwise()) {
            // left
            (dir.counter_clockwise(), point)
        } else if let Some(point) = ins.go(&self.pc, dir.opposite()) {
            // back
            (dir.opposite(), point)
        } else {
            // reaching this means the way we came doesn't exist
            // which should be impossible
            unreachable!()
        }
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
