mod bus;
mod queueing;
mod scheduling;

use crate::mem_request::{MemoryAddress, MemoryCommand, Request, RequestType};
use crate::memory_model::DRAM;

pub use bus::BusDirection;
pub use bus::BusState;
pub use bus::BusDirectionStrategy;
pub use bus::CommandBus;
pub use bus::DataBus;

pub use queueing::QueueManager;
pub use scheduling::Scheduler;


pub trait Refresher {
    fn select() -> Option<Request>;
}
