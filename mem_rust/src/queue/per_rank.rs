use std::io;
use rand::Rng;

use simulation_interface::mem_request::{MemoryAddress, Request, RequestType};
use simulation_interface::policy::{BusDirection, QueueManager};
// use crate::queue::QueueingPolicy;


pub struct PerRankQueue{
    pub read_queue: Vec<Request>,
    pub write_queue: Vec<Request>,
    pub read_queue_capacity: usize,
    pub write_queue_capacity: usize,
    pub num_reads: usize,
    pub num_writes:usize,
}


impl PerRankQueue {

    pub fn new(read_queue_size: usize, write_queue_size: usize) -> io::Result<Self>{
        Ok(Self{
            read_queue: Vec::with_capacity(read_queue_size),
            write_queue: Vec::with_capacity(write_queue_size),
            read_queue_capacity: read_queue_size,
            write_queue_capacity: write_queue_size,
            num_reads: 0,
            num_writes: 0,
        })
    }
    fn enqueue_read(&mut self, mut req: Request) -> Option<Request> {
        if self.num_reads < self.read_queue_capacity {
            // req.stat.queued = self.cycle.clone();
            self.read_queue.push(req);
            self.num_reads += 1;
            None
        } else {
            // print!("{} | queue: [{} {}] | {:?} {}\n", cycle, self.read_queue_size, self.write_queue_size, req.memory_address, req.stat.arrival);
            Some(req)
        }

    }

    fn dequeue_read(&mut self, block_address: usize) -> Option<u32> {
        if let Some(pos) = self.read_queue.iter()
            .position(|item| item.memory_address.block_address == block_address) {
            let queued = self.read_queue[pos].stat.queued.clone();
            self.read_queue.swap_remove(pos);
            self.num_reads -= 1;
            Some(queued)
        } else {
            None
        }
    }

    fn enqueue_write(&mut self, mut req: Request) -> Option<Request> {
        if self.num_writes < self.write_queue_capacity {
            // req.stat.queued = self.cycle.clone();
            self.write_queue.push(req);
            self.num_writes += 1;
            None
        } else {
            // print!("{} | queue: [{} {}] | {:?} {}\n", cycle, self.read_queue_size, self.write_queue_size, req.memory_address, req.stat.arrival);
            Some(req)
        }

    }

    fn dequeue_write(&mut self, block_address:usize) -> Option<u32> {
        if let Some(pos) = self.write_queue.iter()
            .position(|item| item.memory_address.block_address == block_address) {
            let queued = self.write_queue[pos].stat.queued.clone();
            self.write_queue.swap_remove(pos);
            self.num_writes -= 1;
            Some(queued)
        } else {
           None
        }
    }

    fn remove_random_request(&mut self) -> Option<Request> {
        let mut rng = rand::thread_rng();

        // Separate checks for read and write queues
        if !self.read_queue.is_empty() && !self.write_queue.is_empty() {
            if rng.gen() {
                return self.remove_random_from_read_queue();
            } else {
                return self.remove_random_from_write_queue();
            }
        } else if !self.read_queue.is_empty() {
            return self.remove_random_from_read_queue();
        } else if !self.write_queue.is_empty() {
            return self.remove_random_from_write_queue();
        }
        None
    }

    fn remove_random_from_read_queue(&mut self) -> Option<Request> {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..self.read_queue.len());
        self.num_reads -= 1;
        let block_address = self.read_queue[idx].memory_address.block_address.clone();
        println!("removing {}", block_address);
        Some(self.read_queue.swap_remove(idx))
    }

    fn remove_random_from_write_queue(&mut self) -> Option<Request> {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..self.write_queue.len());
        self.num_writes -= 1;
        let block_address = self.write_queue[idx].memory_address.block_address.clone();
        println!("removing {}", block_address);
        Some(self.write_queue.swap_remove(idx))
    }

    fn print_req(&self, block_address: usize) {

        if let Some(pos) = self.read_queue.iter()
            .position(|item| item.memory_address.block_address == block_address) {

            let req = &self.read_queue[pos];
            println!("queue[{}]: ({}, {}) | {:?}", pos, self.num_reads, self.num_writes , req); //self.cycle,
        }else {
            println!(" queue: [{} {}] | {block_address} NOT FOUND!", self.num_reads, self.num_writes);

        }
    }

    fn print_write_req(&self, block_address: usize) {

        if let Some(pos) = self.write_queue.iter()
            .position(|item| item.memory_address.block_address == block_address) {

            let req = &self.write_queue[pos];
            println!("queue[{}]: ({}, {}) | {:?}", pos, self.num_reads, self.num_writes , req);// self.cycle,
        }else {
            println!(" queue: [{} {}] | {block_address} NOT FOUND!", self.num_reads, self.num_writes);

        }
    }

}


impl QueueManager for PerRankQueue {

    fn enqueue(&mut self, req: Request) -> Option<Request> {
        return match req.request_type {
            RequestType::Read => { self.enqueue_read(req) }
            RequestType::Write => { self.enqueue_write(req) }
        }
    }

    fn dequeue(&mut self, addr:MemoryAddress, request_type: RequestType) -> Option<u32> {
        return match request_type {
            RequestType::Read => { self.dequeue_read(addr.block_address) }
            RequestType::Write => { self.dequeue_write(addr.block_address) }
        }
    }

    // fn dequeue_from(&mut self, block_address: usize, bank_group: usize, bank: usize, request_type: RequestType) -> Option<u32> {
    //     todo!()
    // }

    fn get_all_requests(&self, bus_direction: BusDirection) -> &[Request] {
        return match bus_direction {
            BusDirection::Read => { &self.read_queue }
            BusDirection::Write => { &self.write_queue }
        }
    }

    fn get_requests(&self, bus_direction: BusDirection, bank_group: usize, bank: usize) -> &[Request] {
        todo!()
    }

    fn get_queue_length(&self) -> (u32, u32) {
        (self.num_reads as u32, self.num_writes as u32)
    }

}







//
// fn enqueue(&mut self, req: Request) -> Option<Request> {
//     // print!("{} | queue: [{} {}] | {:?}\n", cycle, self.read_queue_size, self.write_queue_size, req);
//
//     let add_request = |queue: &mut Vec<Request>, counter: &mut u8, max_queue_size: u8, mut req: Request| -> Option<Request> {
//         if *counter < max_queue_size {
//             req.stat.arrival = self.cycle.clone();
//             queue.push(req);
//             *counter += 1;
//             None
//         } else {
//             // print!("{} | queue: [{} {}] | {:?} {}\n", cycle, self.read_queue_size, self.write_queue_size, req.memory_address, req.stat.arrival);
//             Some(req)
//         }
//     };
//
//     match req.request_type {
//         RequestType::Read => add_request(&mut self.read_queue, &mut self.num_reads, self.read_queue_size, req),
//         RequestType::Write => add_request(&mut self.write_queue, &mut self.num_writes, self.write_queue_size, req),
//     }
//
// }
//
// fn dequeue(&mut self, req: Request) -> Option<Request> {
//     fn remove_request(queue: &mut Vec<Request>, block_address: u32, counter: &mut u8) -> bool {
//         if let Some(pos) = queue.iter()
//             .position(|item| item.memory_address.block_address == block_address) {
//             queue.swap_remove(pos);
//             *counter -= 1;
//             true
//         } else {
//             false
//         }
//     }
//
//     let result = match req.request_type {
//         RequestType::Read => remove_request(&mut self.read_queue, req.memory_address.block_address, &mut self.num_reads),
//         RequestType::Write => remove_request(&mut self.write_queue, req.memory_address.block_address, &mut self.num_writes),
//     };
//     match result {
//         true => None,
//         false => Some(req)
//     }
// }
