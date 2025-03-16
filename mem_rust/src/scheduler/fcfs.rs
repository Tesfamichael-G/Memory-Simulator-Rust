use simulation_interface::mem_request::{MemoryCommand, Request, RequestType};
use simulation_interface::memory_model::{DRAM, DRAMState};
use simulation_interface::policy::{BusDirection, QueueManager, Scheduler};

pub struct FCFS;

impl Scheduler for FCFS {

    fn select<'a>(&self, queue: &'a impl QueueManager, _dram: &'a impl DRAMState, bus_direction: BusDirection) -> Option<(&'a Request, MemoryCommand)> {

        // let requests = queue.get_requests(bus_direction);
        // let cmd = match bus_direction {
        //     BusDirection::Read => {MemoryCommand::Read}
        //     BusDirection::Write => {MemoryCommand::Write}
        // };
        // let first_request = requests.first();
        // return match first_request {
        //     None => None,
        //     Some(req) => Some((req, cmd))
        // }
        None
    }
}


