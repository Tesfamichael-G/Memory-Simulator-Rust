use crate::mem_request::{MemoryCommand, Request, RequestType};
use crate::memory_model::{DRAM, DRAMState};
use crate::policy::{BusDirection, QueueManager};

pub trait Scheduler {
    fn select<'a>(&self,
                  queue: &'a impl QueueManager,
                  dram:  &'a impl DRAMState,
                  bus_direction: BusDirection) -> Option<(&'a Request, MemoryCommand)>;
}
