use simulation_interface::policy::{BusDirection, BusDirectionStrategy};

pub struct CreditBasedStrategy {
    read_credits: u32,
    write_credits: u32,
    credit_increment: u32,
}

impl BusDirectionStrategy for CreditBasedStrategy {

    fn select_direction(&mut self, read_queue_size: u32, write_queue_size: u32) -> BusDirection {
        if self.read_credits > self.write_credits {
            self.read_credits -= self.credit_increment;
            BusDirection::Read
        } else {
            self.write_credits -= self.credit_increment;
            BusDirection::Write
        }
    }
}