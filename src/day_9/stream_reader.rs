use super::stream_reader_info::StreamReaderInfo;

pub struct StreamReader {
    pub info: StreamReaderInfo,
}

impl StreamReader {
    pub fn new() -> StreamReader {
        StreamReader {
            info: StreamReaderInfo::new(),
        }
    }

    pub fn read_stream(&mut self, stream: &str) {
        for c in stream.chars() {
            Self::handle_character(c, &mut self.info);
        }
    }

    fn handle_character(c: char, stream_info: &mut StreamReaderInfo) {
        match c {
            _ if stream_info.skip => {
                stream_info.skip = false;
            }
            '{' if !stream_info.is_garbage => {
                stream_info.score += stream_info.depth;
                stream_info.depth += 1;
            }
            '}' if !stream_info.is_garbage => {
                stream_info.depth -= 1;
            }
            '<' if !stream_info.is_garbage => {
                stream_info.is_garbage = true;
            }
            '>' => {
                stream_info.is_garbage = false;
            }
            '!' => {
                stream_info.skip = true;
            }
            _ => if stream_info.is_garbage {
                stream_info.garbage_char_count += 1;
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use file_reader::to_string_vector;

    #[test]
    fn test_total_group_score() {
        let test_data = to_string_vector("test_inputs/day_9_part_1.txt").unwrap();

        let test_scores: Vec<usize> = test_data
            .iter()
            .map(|s| {
                let mut temp_reader = StreamReader::new();
                temp_reader.read_stream(s);
                temp_reader.info.score
            })
            .collect();
        let expected = vec![1, 6, 5, 16, 1, 9, 9, 3];

        assert_eq!(test_scores, expected);
    }

    #[test]
    fn test_total_garbage_chars() {
        let test_data = to_string_vector("test_inputs/day_9_part_2.txt").unwrap();

        let test_scores: Vec<usize> = test_data
            .iter()
            .map(|s| {
                let mut temp_reader = StreamReader::new();
                temp_reader.read_stream(s);
                temp_reader.info.garbage_char_count
            })
            .collect();
        let expected = vec![0, 17, 3, 2, 0, 0, 10];

        assert_eq!(test_scores, expected);
    }

    #[test]
    fn test_handle_character() {
        let mut stream_info = StreamReaderInfo::new();

        StreamReader::handle_character('{', &mut stream_info);
        assert_eq!(stream_info.to_tuple(), (2, 1, 0, false, false));

        StreamReader::handle_character('{', &mut stream_info);
        assert_eq!(stream_info.to_tuple(), (3, 3, 0, false, false));

        StreamReader::handle_character('<', &mut stream_info);
        assert_eq!(stream_info.to_tuple(), (3, 3, 0, true, false));

        StreamReader::handle_character('a', &mut stream_info);
        assert_eq!(stream_info.to_tuple(), (3, 3, 1, true, false));

        StreamReader::handle_character('!', &mut stream_info);
        assert_eq!(stream_info.to_tuple(), (3, 3, 1, true, true));

        StreamReader::handle_character('!', &mut stream_info);
        assert_eq!(stream_info.to_tuple(), (3, 3, 1, true, false));

        StreamReader::handle_character('>', &mut stream_info);
        assert_eq!(stream_info.to_tuple(), (3, 3, 1, false, false));

        StreamReader::handle_character('}', &mut stream_info);
        assert_eq!(stream_info.to_tuple(), (2, 3, 1, false, false));

        StreamReader::handle_character('}', &mut stream_info);
        assert_eq!(stream_info.to_tuple(), (1, 3, 1, false, false));
    }
}
