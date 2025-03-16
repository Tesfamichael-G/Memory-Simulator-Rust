use crate::mem_request::{MemoryAddress, Request, RequestType};
use crate::policy::BusDirection;

pub trait  QueueManager {

    fn enqueue(&mut self, req: Request) -> Option<Request> ;

    fn dequeue(&mut self, address: MemoryAddress, request_type: RequestType) -> Option<u32>;

    // fn dequeue_from(&mut self, block_address: usize,  bank_group:usize, bank:usize, request_type: RequestType) -> Option<u32>;

    fn get_all_requests(&self, bus_direction: BusDirection) -> &[Request];

    fn get_requests(&self, bus_direction: BusDirection, bank_group:usize, bank:usize) -> &[Request];

    fn get_queue_length(&self) -> (u32, u32);
}