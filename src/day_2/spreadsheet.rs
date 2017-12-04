#[derive(Debug)]
pub struct SpreadSheet {
    pub matrix: Vec<Vec<u32>>,
}

impl SpreadSheet {
    pub fn new(input: &[String]) -> Result<SpreadSheet, String> {
        let mut matrix = Vec::new();

        for row in input {
            let temp_row = row.split_whitespace()
                .map(|substr| substr.parse::<u32>().expect("Need a number!"))
                .collect();

            matrix.push(temp_row);
        }

        Ok(SpreadSheet { matrix: matrix })
    }

    pub fn min_max_checksum(&self) -> u32 {
        self.matrix.iter().fold(0, |acc, row| {
            acc + (row.iter().max().unwrap() - row.iter().min().unwrap())
        })
    }

    pub fn division_checksum(&self) -> u32 {
        let mut sum = 0;

        for row in &self.matrix {
            let mut division_found = false;

            for number in row {
                for other_number in row {
                    if number == other_number {
                        continue;
                    }

                    if (number % other_number) == 0 {
                        sum += number / other_number;
                        division_found = true;
                    } else if (other_number % number) == 0 {
                        sum += other_number / number;
                        division_found = true;
                    }

                    if division_found {
                        break;
                    }
                }

                if division_found {
                    break;
                }
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use file_reader::to_string_vector;

    #[test]
    fn test_new() {
        let test_input = to_string_vector("test_inputs/day_2_part_1.txt").unwrap();
        let test_spreadsheet = SpreadSheet::new(&test_input).unwrap();

        let expected_matrix: Vec<Vec<u32>> =
            vec![vec![5, 1, 9, 5], vec![7, 5, 3], vec![2, 4, 6, 8]];

        assert_eq!(test_spreadsheet.matrix, expected_matrix);
    }

    #[test]
    fn test_min_max_checksum() {
        let test_input = to_string_vector("test_inputs/day_2_part_1.txt").unwrap();
        let test_spreadsheet = SpreadSheet::new(&test_input).unwrap();

        assert_eq!(test_spreadsheet.min_max_checksum(), 18);
    }

    #[test]
    fn test_division_checksum() {
        let test_input = to_string_vector("test_inputs/day_2_part_2.txt").unwrap();
        let test_spreadsheet = SpreadSheet::new(&test_input).unwrap();

        assert_eq!(test_spreadsheet.division_checksum(), 9);
    }
}
