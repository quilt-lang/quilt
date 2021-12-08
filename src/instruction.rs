#[derive(Debug, PartialEq)]
pub enum Instruction {
    PushA,       // push tape[registerA] to the stack
    PopUntil,    // pops until hitting 0
    Road,        // where the program goes
    Push,        // pushes a u16 onto the stack
    Save,        // saves u16 into tape[registerA]
    MovA,        // moves an address into the registerA
    Add,         // pops the stack twice, adds the numbers & pushes the result
    Start,       // where the program starts
    Output,      // outputs & pops the top of the stack to stdout
    OutputUntil, // outputs & pops the top of the stack to stdout until a 0 is reached

    None,
}

impl Instruction {
    pub fn takes_arg(&self) -> bool {
        matches!(self, Self::Push | Self::MovA | Self::Save)
    }
}
