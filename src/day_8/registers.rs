use std::collections::HashMap;

use super::instruction::Instruction;

pub struct Registers {
    pub inner: HashMap<String, isize>,
    instructions: Instruction,
}

impl Registers {
    pub fn new(instructions: &[String]) -> Registers {
        unimplemented!();
    }

    pub fn run_instructions(&mut self) {
        unimplemented!();
    }

    pub fn value_of_register(&self, register_name: &str) -> isize {
        unimplemented!();
    }

    pub fn add_offset_to_register(&mut self, register_name: &str, offset: isize) {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
