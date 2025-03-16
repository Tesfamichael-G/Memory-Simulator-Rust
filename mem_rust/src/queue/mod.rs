mod per_rank;
mod per_dram;
mod per_bank_group;
mod per_bank;

use simulation_interface::mem_request::{Request, RequestType};
use simulation_interface::policy::{BusDirection, QueueManager};

pub use per_dram::PerDRAMQueue;
pub use per_rank::PerRankQueue;
pub use per_bank_group::PerBankGroupQueue;
pub use per_bank::PerBankQueue;

//
// pub enum QueueingPolicy {
//     PerDRAM(PerDRAMQueue),
//     PerRank(PerRankQueue),
//     PerBankGroup(PerBankGroupQueue),
//     PerBank(PerBankQueue)
//     // Add other queue managers here
// }
//
//
// impl QueueManager for  QueueingPolicy {
//
//     fn enqueue(&mut self, req: Request) -> Option<Request> {
//         match self {
//             QueueingPolicy::PerDRAM(queue) => {None}
//             QueueingPolicy::PerRank(queue) => {queue.enqueue(req)}
//             QueueingPolicy::PerBankGroup(queue) => {None}
//             QueueingPolicy::PerBank(queue) => {None}
//         }
//     }
//
//     fn dequeue(&mut self, block_address: usize, request_type:RequestType) -> Option<u32> {
//         match self {
//             QueueingPolicy::PerDRAM(queue) => None,
//             QueueingPolicy::PerRank(queue) => queue.dequeue(block_address, request_type),
//             QueueingPolicy::PerBankGroup(queue) => None,
//             QueueingPolicy::PerBank(queue) => None
//         }
//     }
//
//     fn get_requests(&self, bus_direction: BusDirection) -> &[Request] {
//
//         match self {
//             QueueingPolicy::PerDRAM(queue) => panic!("No requests PerDRAM"),
//             QueueingPolicy::PerRank(queue) => {queue.get_requests(bus_direction)},
//             QueueingPolicy::PerBankGroup(queue) => panic!("No requests PerBankGroup"),
//             QueueingPolicy::PerBank(queue) => panic!("No requests PerBank")
//         }
//     }
//
//     fn get_queue_length(&self) -> (u32, u32) {
//         match self {
//             QueueingPolicy::PerDRAM(queue) => panic!("No requests PerDRAM"),
//             QueueingPolicy::PerRank(queue) => {queue.get_queue_length()},
//             QueueingPolicy::PerBankGroup(queue) => panic!("No requests PerBankGroup"),
//             QueueingPolicy::PerBank(queue) => panic!("No requests PerBank")
//         }
//     }
// }