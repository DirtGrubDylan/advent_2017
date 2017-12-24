#[derive(Debug, PartialEq)]
pub struct Pipe {
    pub source: i32,
    pub destinations: Vec<i32>,
}

impl Pipe {
    pub fn new(description: &str) -> Pipe {
        let temp_vec: Vec<i32> = description
            .split(|c| c == ',' || c == ' ')
            .filter(|&s| s != "" && s != "<->")
            .map(|s| s.parse().unwrap())
            .collect();

        Pipe {
            source: temp_vec[0],
            destinations: temp_vec[1..].to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            Pipe::new("1 <-> 1"),
            Pipe {
                source: 1,
                destinations: vec![1],
            }
        );
        assert_eq!(
            Pipe::new("2 <-> 0, 3, 4"),
            Pipe {
                source: 2,
                destinations: vec![0, 3, 4],
            }
        );
    }
}
