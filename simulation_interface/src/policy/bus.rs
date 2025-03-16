use crate::mem_request::{Callback, MemoryAddress};

#[derive(Debug, Copy, Clone)]
pub enum BusState {
    Transporting = 0,
    Idle = 1,
}

#[derive(Debug, Copy, Clone)]
pub enum BusDirection {
    Read = 0,
    Write = 1,
}

pub trait BusDirectionStrategy {
    fn select_direction(&mut self, read_queue_size: u32, write_queue_size:u32) -> BusDirection;
}

pub trait DataBus {

    // fn tick(&mut self)  ;
    fn write(&mut self, data: u32)  ;
    fn read(&mut self) -> Option<u32> ;

}

pub trait CommandBus {

    fn precharge_b(&mut self, addr: MemoryAddress );

    fn precharge_ab(&mut self, rank: usize);

    fn activate(&mut self, addr: MemoryAddress );

    fn read(&mut self, addr: MemoryAddress, queued:u32);//), bus: &impl DataBus);

    fn read_ap(&mut self, addr: MemoryAddress, queued:u32);//, bus: &impl DataBus);

    fn write(&mut self, addr: MemoryAddress, queued:u32);

    fn write_ap(&mut self, addr: MemoryAddress, queued:u32);

    fn refresh_ab(&mut self, addr: MemoryAddress );

}
