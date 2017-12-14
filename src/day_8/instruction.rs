use super::registers::Registers;
use super::conditional::Conditional;

pub struct Instruction {
    pub name_of_register_to_modify: String,
    pub new_value_to_modify_with: isize,
    conditional: Conditional,
}

impl Instruction {
    pub fn new(instruction: &str) -> Instruction {
        unimplemented!();
    }

    pub fn execute(&self, registers: &mut Registers) {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
