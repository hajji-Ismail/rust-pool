#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl<'a> Numbers<'a> {
    pub fn new(numbers: &'a [u32]) -> Self {
        Numbers { numbers }
    }

    pub fn list(&self) -> &[u32] {
        self.numbers
    }

    pub fn latest(&self) -> Option<u32> {
        self.numbers.last().cloned()
    }

    pub fn highest(&self) -> Option<u32> {
        self.numbers.iter().max().cloned()
    }

    pub fn highest_three(&self) -> Vec<u32> {
        let mut sorted: Vec<u32> = self.numbers.to_vec(); // create owned, mutable Vec
        sorted.sort_by(|a, b| b.cmp(a)); // sort descending
        sorted.truncate(3); // keep only top 3
        sorted
    }
}
