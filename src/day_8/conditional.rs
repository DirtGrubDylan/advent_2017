use super::registers::Registers;

pub struct Conditional {
    register_name: String,
    operation: String,
    value: isize,
}

impl Conditional {
    pub fn new(register_name: &str, operation: &str, value: isize) -> Conditional {
        Conditional {
            register_name: register_name.to_owned(),
            operation: operation.to_owned(),
            value: value,
        }
    }

    pub fn is_true(&self, registers: &Registers) -> bool {
        let register_value = registers.value_of_register(&self.register_name);

        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
