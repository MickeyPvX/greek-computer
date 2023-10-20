#[derive(Debug)]
pub struct PuzzleSlice {
    pub inner: u8,
    pub inner_middle: u8,
    pub outer_middle: u8,
    pub outer: u8,
}

impl PuzzleSlice {
    pub fn sum(self) -> u8 {
        let slice: [u8; 4] = [self.inner, self.inner_middle, self.outer_middle, self.outer];
        slice.iter().sum()
    }
}
