use simulation_interface::policy::{BusDirection, BusDirectionStrategy};

pub struct ThresholdStrategy {
    read_queue_threshold: u32,
    write_queue_threshold: u32,
    current_mode: BusDirection,
}

impl ThresholdStrategy{
    pub fn new(read_queue_threshold:u32, write_queue_threshold:u32) -> Self {
        Self{
            read_queue_threshold,
            write_queue_threshold,
            current_mode: BusDirection::Read,
        }
    }
}

impl BusDirectionStrategy for ThresholdStrategy {

    fn select_direction(&mut self, read_queue_size: u32, write_queue_size: u32) -> BusDirection {

        if read_queue_size >= self.read_queue_threshold {
            return BusDirection::Read;
        } else if write_queue_size >= self.write_queue_threshold {
            return BusDirection::Write;
        } else {
            return BusDirection::Read;
        }
    }

}