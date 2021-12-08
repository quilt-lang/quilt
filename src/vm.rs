use crate::matrix::{Matrix, MatrixPoint};

const TAPE_SIZE: usize = 360;

pub struct VM {
    stack: Vec<u16>,
    registerA: u16,
    tape: [u16; TAPE_SIZE],
    direction: Direction,
    instructions: Matrix<Pixel>,
    program_counter: MatrixPoint,
}

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
            Direction::West  => Direction::East,
            Direction::South => Direction::North,
            Direction::East  => Direction::West,
        }
    }

    pub fn counter_clockwise(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West  => Direction::South,
            Direction::South => Direction::East,
            Direction::East  => Direction::North,
        }
    }
}

#[derive(Debug)]
pub struct Pixel {
    value: u16,
}

impl Pixel {
    pub fn new(value: u16) -> Pixel {
        Pixel { value }
    }

    pub fn as_instruction(&self) -> Instruction {
        match self.value {
            0..=8 => Instruction::PushA,
            18..=26 => Instruction::PopUntil,
            36..=44 => Instruction::Push,
            108..=116 => Instruction::Add,
            180..=188 => Instruction::Road,
            300 => Instruction::Start,
            306..=314 => Instruction::Output,
            _ => todo!("{}", self.value),
        }
    }

    pub fn as_data(&self) -> u16 {
        self.value
    }
}

#[derive(PartialEq)]
pub enum Instruction {
    PushA,    // pushes an address into registerA
    PopUntil, // pops until hitting 0
    Road,     // where the program goes
    Push,     // pushes a u16 onto the stack
    Save,     // saves u16 into tape[registerA]
    Add,      // pops the stack twice, adds the numbers & pushes the result
    Start,    // where the program starts
    Output,
}

impl Default for VM {
    fn default() -> VM {
        VM {
            stack: vec![],
            registerA: 0,
            tape: [0; TAPE_SIZE],
            direction: Direction::East,
            instructions: Matrix::new(vec![]),
            program_counter: MatrixPoint(0, 0),
        }
    }
}

impl VM {
    pub fn new() -> VM {
        VM::default()
    }

    pub fn execute(&mut self, instructions: Matrix<Pixel>) {
        self.instructions = instructions;
        self.program_counter = self.find_start();

        loop { // TODO change condition
        }
    }

    // prioritize roads over all other instructions
    pub fn get_next_instruction() -> MatrixPoint {
        MatrixPoint(0, 0)
    }

    // check pixel in same-direction for a road
    // then search counter-clockwise for a road, starting from
    // current direction
    fn get_next_pixel(&self) -> (Direction, MatrixPoint) {
        let MatrixPoint(x, y) = self.program_counter;

        match self.direction {
            Direction::North => {
                /*
                let point = if y == 0 { // make sure we don't underflow
                    MatrixPoint(x, 1)
                } else {
                    MatrixPoint(x, y + 1)
                }

                if self.instructions.cell_exists(point) {
                    (self.direction, point)
                } else {
                    (self.direction.opposite(), )
                }
                */
                Direction::West
            },
            
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        };

        (Direction::North, MatrixPoint(0, 0))
    }

    fn find_start(&self) -> MatrixPoint {
        for (rowIdx, row) in self.instructions.matrix.iter().enumerate() {
            for (colIdx, pixel) in row.iter().enumerate() {
                if pixel.as_instruction() == Instruction::Start {
                    return MatrixPoint(rowIdx, colIdx);
                }
            }
        }

        // default start coordinates
        MatrixPoint(0, 0)
    }
}
