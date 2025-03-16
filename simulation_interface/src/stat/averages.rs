use std::io;

pub struct Average{
    sum: u64,
    count:u32,
}
impl Average{
    pub fn new() -> io::Result<Self>{
        Ok(Self {
            sum: 0,
            count: 0,
        })
    }

    pub fn get_average(&mut self) -> Option<f64>{
        if self.count == 0 {
            return None;
        }
        Some(self.sum as f64 / self.count as f64)
    }

    pub fn accumulate(&mut self, value:u32) {
        self.sum = self.sum + value as u64;
        self.count +=1;
    }

}