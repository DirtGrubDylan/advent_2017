use super::registry::Registry;
use super::instruction::Instruction;

#[derive(Debug)]
pub struct Program {
    pub first_registry: Registry,
    pub second_registry: Registry,
}

impl Program {
    pub fn new(instructions: &[Instruction]) -> Program {
        Program {
            first_registry: Registry::new_with_instructions(0, instructions),
            second_registry: Registry::new_with_instructions(1, instructions),
        }
    }
}
