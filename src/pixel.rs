use crate::hsl::Hsl;
use crate::{Instruction, MatrixPoint};

pub const START: u16 = 300;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pixel {
    pub value: u16,
    pub point: MatrixPoint,
}

impl Pixel {
    pub fn new(value: u16, point: MatrixPoint) -> Pixel {
        Pixel { value, point, }
    }

    pub fn as_instruction(&self) -> Instruction {
        match self.value {
            0..=8 => Instruction::PushA,
            18..=26 => Instruction::PopUntil,
            36..=44 => Instruction::Push,
            108..=116 => Instruction::Add,
            180..=188 => Instruction::Road,
            START => Instruction::Start,
            306..=314 => Instruction::Output,
            324..=332 => Instruction::OutputUntil,
            _ => Instruction::None,
        }
    }

    pub fn as_data(&self) -> u16 {
        self.value
    }
}
