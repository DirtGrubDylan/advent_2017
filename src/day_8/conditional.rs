use std::collections::HashMap;

#[derive(Debug, PartialEq)]
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

    pub fn is_true(&self, registers: &HashMap<String, isize>) -> bool {
        let register_value = *(registers.get(&self.register_name).unwrap_or(&0));

        match self.operation.as_str() {
            ">" => register_value > self.value,
            ">=" => register_value >= self.value,
            "<" => register_value < self.value,
            "<=" => register_value <= self.value,
            "==" => register_value == self.value,
            "!=" => register_value != self.value,
            _ => false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_true() {
        let mut registers = HashMap::new();

        registers.insert(String::from("a"), 1);

        let test_data: Vec<bool> = [">", ">=", "<", "<=", "==", "!="].into_iter()
            .map(|s| Conditional::new("a", s, 1).is_true(&registers))
            .collect();
        let expected = vec![false, true, false, true, true, false];

        assert_eq!(test_data, expected);
    }
}
