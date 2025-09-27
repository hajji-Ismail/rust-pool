pub struct StepIterator<T> {
beg: T,
end: T, 
step: T
}

use std::ops::Add;

impl<T> StepIterator<T>
where
    T: Copy + Add<Output = T> + PartialOrd,
{
    pub fn new(beg: T, end: T, step: T) -> Self {
        StepIterator {
            beg,
            end,
            step,
        }
    }
}

impl<T> Iterator for StepIterator<T>
where
       T: Copy + Add<Output = T> + PartialOrd + Add<Output = T> + From<i32>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
      
        
       
        let finished = if self.step > T::from(0) {
            self.beg > self.end
        } else {
            self.beg < self.end
        };

        if finished {
            return None;
        }
        

        let result = self.beg;
        self.beg = self.beg + self.step;
        Some(result)
    }
}