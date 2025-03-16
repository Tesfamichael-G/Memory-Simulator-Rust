use simulation_interface::mem_request::{MemoryCommand, Request, RequestType};
use simulation_interface::memory_model::{DRAM, DRAMState};
use simulation_interface::policy::{BusDirection, QueueManager, Scheduler};

pub struct FRFCFS;

impl Scheduler for FRFCFS {

    fn select<'a>(&self, queue: &'a impl QueueManager, dram: &'a impl DRAMState, bus_direction: BusDirection) -> Option<(&'a Request, MemoryCommand)> {
        let mut best_request: Option<(&'a Request, bool)> = None; // bool indicates if the command is CAS

        for req in queue.get_all_requests(bus_direction) {
            if (dram.is_bank_busy(req.memory_address.bank_group, req.memory_address.bank)){
                continue;
            }
            if let Some(command) = dram.get_valid_command(req) {
                // println!("command {:?}", command);
                let is_cas_command = matches!(command, MemoryCommand::Read | MemoryCommand::Write);

                match best_request {
                    Some((best_req, best_is_cas)) => {
                        let should_replace = match (is_cas_command, best_is_cas) {
                            (true, false) => true, // CAS is preferred over RAS
                            (false, true) => false, // RAS is not preferred over CAS
                            (true, true) | (false, false) => req.stat.queued < best_req.stat.queued,
                            _ => false,
                        };

                        if should_replace {
                            best_request = Some((req, is_cas_command));
                        }
                    }
                    None => {
                        best_request = Some((req, is_cas_command));
                    }
                }
            }
        }

        best_request
            .map(|(req, _)| (req, dram.get_valid_command(req).unwrap()))
    }

}
