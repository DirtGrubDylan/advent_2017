pub struct StreamReaderInfo {
    pub depth: usize,
    pub score: usize,
    pub garbage_char_count: usize,
    pub is_garbage: bool,
    pub skip: bool,
}

impl StreamReaderInfo {
    pub fn new() -> StreamReaderInfo {
        StreamReaderInfo {
            depth: 1,
            score: 0,
            garbage_char_count: 0,
            is_garbage: false,
            skip: false,
        }
    }

    pub fn to_tuple(&self) -> (usize, usize, usize, bool, bool) {
        (
            self.depth,
            self.score,
            self.garbage_char_count,
            self.is_garbage,
            self.skip,
        )
    }
}
