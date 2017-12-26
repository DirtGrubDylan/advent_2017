#[derive(Debug)]
pub struct DiskGrid {
    pub key: String,
}

impl DiskGrid {
    pub fn new(key: &str) -> DiskGrid {
        DiskGrid {
            key: key.to_owned(),
        }
    }

    pub fn number_of_used_squares(&self) -> usize {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_used_squares() {
        unimplemented!();
    }
}
