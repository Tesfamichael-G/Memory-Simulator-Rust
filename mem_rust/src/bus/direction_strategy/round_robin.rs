use simulation_interface::policy::{BusDirection, BusDirectionStrategy};

pub struct RoundRobinStrategy {
    current_direction: BusDirection,
}



impl BusDirectionStrategy for RoundRobinStrategy {
    fn select_direction(&mut self, read_queue_size: u32, write_queue_size: u32) -> BusDirection {
        match self.current_direction {
            BusDirection::Read => {
                self.current_direction = BusDirection::Write;
                BusDirection::Read
            }
            BusDirection::Write => {
                self.current_direction = BusDirection::Read;
                BusDirection::Write
            }
        }
    }
}