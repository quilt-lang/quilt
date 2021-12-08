#[derive(PartialEq)]
pub enum Instruction {
    PushA,    // pushes an address into registerA
    PopUntil, // pops until hitting 0
    Road,     // where the program goes
    Push,     // pushes a u16 onto the stack
    Save,     // saves u16 into tape[registerA]
    Add,      // pops the stack twice, adds the numbers & pushes the result
    Start,    // where the program starts
    Output,   // outputs & pops the top of the stack to stdout

    None,
}
