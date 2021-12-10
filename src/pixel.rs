use crate::{Condition, Hsl, Instruction, MatrixPoint};

pub const START: u16 = 300;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pixel {
    pub hsl: Hsl,
    pub point: MatrixPoint,
}

impl Pixel {
    pub fn new(hsl: Hsl, point: MatrixPoint) -> Pixel {
        Pixel { hsl, point }
    }

    pub fn as_instruction(&self) -> Instruction {
        match self.value() {
            0..=8 => Instruction::PushA,
            18..=26 => Instruction::PopUntil,
            36..=44 => Instruction::Push,
            54..=62 => Instruction::Save,
            72..=80 => Instruction::MovA,
            90..=98 => Instruction::PopA,
            108..=116 => Instruction::Add,
            126..=134 => Instruction::Sub,
            144..=152 => Instruction::Mult,
            162..=170 => Instruction::Div,
            180..=188 => Instruction::Road,
            198..=206 => Instruction::LeftShift,
            216..=224 => Instruction::RightShift,
            234..=242 => Instruction::And,
            252..=260 => Instruction::Or,
            270..=278 => Instruction::Not,
            288..=296 => Instruction::Xor,
            306..=314 => Instruction::Output,
            324..=332 => Instruction::OutputUntil,
            342..=350 => Instruction::Modulo,
            START => Instruction::Start,
            _ => Instruction::None,
        }
    }

    #[inline]
    pub fn value(&self) -> u16 {
        self.hsl.h
    }

    pub fn as_data(&self) -> u16 {
        self.value()
    }

    pub fn as_condition(&self) -> Condition {
        match self.value() {
            0..=8 => Condition::NotEqual,
            72..=80 => Condition::Less,
            144..=152 => Condition::LessEqual,
            216..=224 => Condition::Greater,
            288..=296 => Condition::GreaterEqual,
            _ => Condition::Equal,
        }
    }
}
