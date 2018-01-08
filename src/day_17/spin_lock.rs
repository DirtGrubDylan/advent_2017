#[derive(Debug, Clone)]
pub struct SpinLock {
    pub inner: Vec<usize>,
    step: usize,
    current_index: usize,
    current_element: usize,
}

impl Iterator for SpinLock {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.insert(self.current_index, self.current_element);

        self.current_element += 1;
        self.current_index = (self.current_index + self.step) % self.inner.len() + 1;

        Some(self.inner.clone())
    }
}

impl SpinLock {
    pub fn new(step: usize) -> SpinLock {
        SpinLock {
            inner: Vec::new(),
            step: step,
            current_index: 0,
            current_element: 0,
        }
    }

    pub fn short_cicuit_after_index(&self, steps: usize, index: usize) -> Option<usize> {
        let mut temp_vec = Vec::new();
        let mut current_index = 0;
        let mut span = 0;

        for current_element in 0..(steps + 1) {
            if current_index <= (index + 1) {
                temp_vec.insert(current_index, current_element);
            }

            span += 1;
            current_index = (current_index + self.step) % span + 1;
        }

        match temp_vec.get(index + 1) {
            None => None,
            Some(&number) => Some(number),
        }
    }

    pub fn short_cicuit_after_value(&self, steps: usize, value: usize) -> Option<usize> {
        let temp_vec = self.clone().nth(steps).unwrap();

        match temp_vec.iter().position(|&x| x == value) {
            None => None,
            Some(index) => Some(temp_vec[(index + 1) % temp_vec.len()]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next() {
        assert_eq!(SpinLock::new(3).nth(5).unwrap(), vec![0, 5, 2, 4, 3, 1]);
    }

    #[test]
    fn test_short_cicuit_after_index() {
        assert_eq!(SpinLock::new(3).short_cicuit_after_index(5, 2), Some(4));
    }

    #[test]
    fn test_short_cicuit_after_value() {
        assert_eq!(
            SpinLock::new(3).short_cicuit_after_value(2017, 2017),
            Some(638)
        );
    }
}
