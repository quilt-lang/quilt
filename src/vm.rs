const TAPE_SIZE: usize = 360;

pub struct VM {
    stack: Vec<u16>,
    registerA: u16,
    tape: [u16; TAPE_SIZE],
    instructions: Vec<Vec<Pixel>>,
    program_counter: (usize, usize),
    direction: Direction,
}

pub enum Direction {
    North,
    East,
    South,
    West,
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
            0..=8     => Instruction::PushA,
            18..=26   => Instruction::PopUntil,
            36..=44   => Instruction::Push,
            108..=116 => Instruction::Add,
            180..=188 => Instruction::Road,
            300       => Instruction::Start,
            306..=314 => Instruction::Output,
            _         => todo!("{}", self.value),
        }
    }

    pub fn as_data(&self) -> u16 {
        self.value
    }
}

pub enum Instruction {
    PushA,      // pushes an address into registerA
    PopUntil,   // pops until hitting 0
    Road,       // where the program goes
    Push,       // pushes a u16 onto the stack
    Save,       // saves u16 into tape[registerA]
    Add,        // pops the stack twice, adds the numbers & pushes the result
    Start,      // where the program starts
    Output,
}

impl Default for VM {
    fn default() -> VM {
        VM {
            stack: vec![],
            registerA: 0,
            tape: [0; TAPE_SIZE],
            instructions: vec![],
            program_counter: (0, 0),
            direction: Direction::East,
        }
    }
}

impl VM {
    pub fn new(instructions: Vec<Vec<Pixel>>) -> VM {
        VM {
            instructions,
            ..VM::default()
        }
    }

    pub fn execute(self) {
        let start = self.find_start();
    }

    fn find_start(&self) -> (usize, usize) {
        for (rowIdx, row) in self.instructions.iter().enumerate() {
            for (colIdx, pixel) in row.iter().enumerate() {
                if pixel.as_instruction() == Instruction::Start {
                    return (rowIdx, colIdx)
                }
            }
        }

        // default start coordinates
        (0,0)
    }
}
