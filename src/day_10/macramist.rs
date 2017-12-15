pub struct Macramist {
    pub inner: Vec<usize>,
}

impl Macramist {
    pub fn new() -> Macramist {
        Macramist {
            inner: (0..256).collect(),
        }
    }

    pub fn tie_with_lengths(&mut self, lengths: &[usize]) {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
