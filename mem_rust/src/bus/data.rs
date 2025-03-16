// use simulation_interface::mem_request::{Callback, ReadCompleteCallback};
// use simulation_interface::memory_model::{DRAM};
// use simulation_interface::policy::{BusDirectionStrategy, QueueManager, Scheduler};
// use crate::controller::{MemoryController};
//
// pub struct DataBus<'a, S, Q, BD, D, C>
//     where
//         S: Scheduler<Q, D>,
//         Q: QueueManager,
//         // DS: DRAMState,
//         BD: BusDirectionStrategy,
//         // D: DRAM<CallbackType = C>,
//         // C: Fn(usize) + 'static,
//         D: DRAM<CallbackType = ReadCompleteCallback<C>>,
//         C: Callback
// {
//     pub controller: &'a MemoryController<S, Q, BD, D, C>,
//     pub dram: &'a D
// }