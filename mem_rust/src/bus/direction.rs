use std::io;
use simulation_interface::policy::{BusDirection, BusDirectionStrategy, DataBus};

pub struct Bus<DS: BusDirectionStrategy> {
    data: Option<u32>,
    direction_strategy: DS
}

impl<DS: BusDirectionStrategy> DataBus for  Bus<DS>  {
    fn write(&mut self, data: u32) {
        self.data = Some(data);
    }

    fn read(&mut self) -> Option<u32> {
        self.data.take()
    }
}

impl<DS: BusDirectionStrategy> Bus<DS> {

    pub fn new(direction_strategy: DS) -> io::Result<Self>{

        Ok(Self{
            data: None,
            direction_strategy,
        })
    }

    pub fn get_direction(&mut self, num_read_requests:u32, num_write_requests:u32) -> BusDirection{
        self.direction_strategy.select_direction(num_read_requests, num_write_requests)
    }

}