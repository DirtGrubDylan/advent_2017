#[derive(Debug)]
pub struct Captcha {
    value: String,
}

impl Captcha {
    pub fn new(captch_value: &str) -> Captcha {
        Captcha {
            value: captch_value.to_owned(),
        }
    }

    pub fn sum(&self, step_size: usize) -> u32 {
        let chars = self.value.chars().collect::<Vec<char>>();

        chars.iter().enumerate().fold(0, |acc, (index, &c)| {
            let next_index = (index + step_size) % self.value.len();

            let next_c = chars[next_index];

            if c == next_c {
                acc + c.to_digit(10).unwrap()
            } else {
                acc
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use file_reader::to_string_vector;

    #[test]
    fn test_new() {
        let test_captchas: Vec<Captcha> = to_string_vector("test_inputs/day_1_part_1.txt")
            .unwrap()
            .into_iter()
            .map(|value| Captcha::new(&value))
            .collect();
        let expected_values = vec!["1122", "1111", "1234", "91212129"];

        for (captcha, expected_value) in test_captchas.iter().zip(expected_values.into_iter()) {
            assert_eq!(captcha.value, expected_value);
        }
    }

    #[test]
    fn test_sum_adjacent() {
        let test_captchas: Vec<Captcha> = to_string_vector("test_inputs/day_1_part_1.txt")
            .unwrap()
            .into_iter()
            .map(|value| Captcha::new(&value))
            .collect();
        let expected_values = vec![3, 4, 0, 9];

        for (captcha, expected_value) in test_captchas.iter().zip(expected_values.into_iter()) {
            assert_eq!(captcha.sum(1), expected_value);
        }
    }

    #[test]
    fn test_sum_half() {
        let test_captchas: Vec<Captcha> = to_string_vector("test_inputs/day_1_part_2.txt")
            .unwrap()
            .into_iter()
            .map(|value| Captcha::new(&value))
            .collect();
        let expected_values = vec![6, 0, 4, 12, 4];

        for (captcha, expected_value) in test_captchas.iter().zip(expected_values.into_iter()) {
            let step_size = captcha.value.len() / 2;

            assert_eq!(captcha.sum(step_size), expected_value);
        }
    }
}
