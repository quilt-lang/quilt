#[derive(Debug, PartialEq)]
pub enum Instruction {
    PushA,       // push tape[registerA] to the stack
    PopUntil,    // pops until hitting 0
    Push,        // pushes a u16 onto the stack
    Save,        // saves u16 into tape[registerA]
    MovA,        // moves an address into the registerA
    Pop,
    Add,         // pops the stack twice, adds the numbers & pushes the result
    Sub,
    Mult,
    Div,
    Road,        // where the program goes
    LeftShift,
    RightShift,
    And,
    Or,
    Not,
    Xor,
    Output,      // outputs & pops the top of the stack to stdout
    OutputUntil, // outputs & pops the top of the stack to stdout until a 0 is reached
    Modulo,
    Start,       // where the program starts

    None,
}

impl Instruction {
    pub fn takes_arg(&self) -> bool {
        matches!(self, Self::Push | Self::MovA | Self::Save)
    }
}
