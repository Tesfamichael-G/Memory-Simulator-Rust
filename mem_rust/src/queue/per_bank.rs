use std::io;
use rand::Rng;
use simulation_interface::mem_request::{MemoryAddress, Request, RequestType};
use simulation_interface::policy::{BusDirection, QueueManager};
use crate::queue::PerRankQueue;

pub struct PerBankQueue{
    pub read_queue: Vec<Vec<Vec<Request>>>,
    pub write_queue: Vec<Vec<Vec<Request>>>,
    pub read_queue_capacity: usize,
    pub write_queue_capacity: usize,
    pub num_reads: usize,
    pub num_writes:usize,
}


impl PerBankQueue {

    pub fn new(read_queue_size: usize, write_queue_size: usize, num_bank_groups:usize, num_banks:usize) -> io::Result<Self>{

        let mut read_queue = Vec::with_capacity(num_bank_groups);
        let mut write_queue = Vec::with_capacity(num_bank_groups);

        for _ in 0..num_bank_groups {
            let mut rd_banks = Vec::with_capacity(num_banks);
            let mut wr_banks = Vec::with_capacity(num_banks);

            for _ in 0..num_banks {
                rd_banks.push(Vec::with_capacity(read_queue_size));
                wr_banks.push(Vec::with_capacity(write_queue_size));
            }
            read_queue.push(rd_banks);
            write_queue.push(wr_banks);
        }

        // println!("*** created read queue: {:?}", read_queue);
        Ok(Self{
            read_queue,
            write_queue,
            read_queue_capacity: read_queue_size ,
            write_queue_capacity: write_queue_size ,
            num_reads: 0,
            num_writes: 0,
        })
    }

    fn enqueue_read(&mut self, mut req: Request) -> Option<Request> {
        let ( g, b) =  (req.memory_address.bank_group, req.memory_address.bank);
        if self.num_reads < self.read_queue_capacity {
            // print!("\nBefore panic... bank group: {g} bank: {b} read_queue {:#?} \n", self.read_queue);
            self.read_queue[g][b].push(req);
            self.num_reads += 1;
            None
        } else {
            // print!("{} | queue: [{} {}] | {:?} {}\n", cycle, self.read_queue_size, self.write_queue_size, req.memory_address, req.stat.arrival);
            Some(req)
        }
    }

    fn dequeue_read(&mut self, block_address: usize, bank_group: usize, bank: usize ) -> Option<u32> {

        if let Some(pos) = self.read_queue[bank_group][bank].iter()
            .position(|item| item.memory_address.block_address == block_address) {
            let queued = self.read_queue[bank_group][bank][pos].stat.queued.clone();
            self.read_queue[bank_group][bank].swap_remove(pos);
            self.num_reads -= 1;
            Some(queued)
        } else {
            None
        }
    }

    fn enqueue_write(&mut self, mut req: Request) -> Option<Request> {
        let (g, b) =  (req.memory_address.bank_group, req.memory_address.bank);
        if self.num_writes < self.write_queue_capacity {
            // req.stat.queued = self.cycle.clone();
            self.write_queue[g][b].push(req);
            self.num_writes += 1;
            None
        } else {
            // print!("{} | queue: [{} {}] | {:?} {}\n", cycle, self.read_queue_size, self.write_queue_size, req.memory_address, req.stat.arrival);
            Some(req)
        }

    }

    fn dequeue_write(&mut self, block_address:usize, bank_group: usize, bank: usize) -> Option<u32> {
        if let Some(pos) = self.write_queue[bank_group][bank].iter()
            .position(|item| item.memory_address.block_address == block_address) {
            let queued = self.write_queue[bank_group][bank][pos].stat.queued.clone();
            self.write_queue[bank_group][bank].swap_remove(pos);
            self.num_writes -= 1;
            Some(queued)
        } else {
            None
        }
    }

}

impl QueueManager for PerBankQueue {

    fn enqueue(&mut self, req: Request) -> Option<Request> {
        return match req.request_type {
            RequestType::Read => { self.enqueue_read(req) }
            RequestType::Write => { self.enqueue_write(req) }
        }
    }

    fn dequeue(&mut self, addr: MemoryAddress, request_type: RequestType) -> Option<u32> {
        return match request_type {
            RequestType::Read => { self.dequeue_read(addr.block_address, addr.bank_group, addr.bank) }
            RequestType::Write => { self.dequeue_write(addr.block_address, addr.bank_group, addr.bank) }
        }
    }
    // fn dequeue_from(&mut self, block_address: usize, bank_group: usize, bank: usize, request_type: RequestType) -> Option<u32> {
    //     return match request_type {
    //         RequestType::Read => { self.dequeue_read(block_address, bank_group, bank) }
    //         RequestType::Write => { self.dequeue_write(block_address, bank_group, bank) }
    //     }
    // }

    fn get_requests(&self, bus_direction: BusDirection, bank_group: usize, bank: usize) -> &[Request] {
        return match bus_direction {
            BusDirection::Read => {
                &self.read_queue[bank_group][bank][..]
            }
            BusDirection::Write => {
                &self.write_queue[bank_group][bank][..]
            }
        }
    }
    fn get_all_requests(&self, bus_direction: BusDirection) -> &[Request] {
       todo!()
    }

    fn get_queue_length(&self) -> (u32, u32) {
        (self.num_reads as u32, self.num_writes as u32)
    }


}

