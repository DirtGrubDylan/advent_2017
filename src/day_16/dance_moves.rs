#[derive(Debug, PartialEq)]
pub enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Parter(char, char),
}

impl DanceMove {
    pub fn new(information: &str) -> DanceMove {
        let temp_info = &information[1..];

        match information.chars().nth(0).unwrap() {
            's' => DanceMove::Spin(temp_info.parse().unwrap()),
            'x' => {
                let temp_vec: Vec<usize> =
                    temp_info.split('/').map(|s| s.parse().unwrap()).collect();

                DanceMove::Exchange(temp_vec[0], temp_vec[1])
            }
            'p' => DanceMove::Parter(
                temp_info.chars().nth(0).unwrap(),
                temp_info.chars().nth(2).unwrap(),
            ),
            _ => panic!("Fuck, my dude!"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(DanceMove::new("s1"), DanceMove::Spin(1));
        assert_eq!(DanceMove::new("x3/4"), DanceMove::Exchange(3, 4));
        assert_eq!(DanceMove::new("pe/b"), DanceMove::Parter('e', 'b'));
    }
}
