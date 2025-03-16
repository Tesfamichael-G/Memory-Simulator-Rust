use std::io;
use std::marker::PhantomData;

use rand::Rng;
use simulation_interface::mem_request::{Callback, MemoryAddress, MemoryCommand, Request, RequestType};

use simulation_interface::memory_model::{DRAM, DRAMState};
use simulation_interface::memory_model::ddr::DDRTimingConstraints;
use simulation_interface::policy::{BusDirection, BusDirectionStrategy, Scheduler};
use simulation_interface::policy::QueueManager;
use crate::bus::{Bus};
use crate::memory::DDR;

pub struct MemoryController<D: DRAM, Q: QueueManager, S:Scheduler, B:BusDirectionStrategy>
{
    cycle: u32,
    queue_manager: Q,
    dram: D,
    scheduler: S,
    records: u32,
    bus: Bus<B>,
}

impl<D: DRAM, Q: QueueManager, S:Scheduler, B:BusDirectionStrategy> MemoryController<D, Q, S, B>
{
    pub fn new(
        scheduler: S,
        queue_manager: Q,
        bus_direction_strategy: B,
        dram: D,
        // num_ranks: usize,
        // num_bank_groups: usize,
        // num_banks: usize,
        // timing_constraints: DDRTimingConstraints,
    ) -> io::Result<Self>{

        let bus = Bus::new(bus_direction_strategy)?;
        // let dram: DDR = DDR::new(num_ranks, num_bank_groups,  num_banks, timing_constraints).expect("Failed to create DRAM");

        Ok(Self{
            cycle: 0,
            records: 0,
            dram,
            queue_manager,
            scheduler,
            bus,
        })
    }


    pub fn tick(&mut self) -> Option<u32> {
        self.cycle += 1;

         let (num_read_requests, num_write_requests) = self.queue_manager.get_queue_length();
         let bus_direction = self.bus.get_direction(num_read_requests, num_write_requests);

        // self.select_and_issue(bus_direction);
        let maybe_request = self.scheduler.select(&self.queue_manager, &self.dram, bus_direction);

        if let Some((req, cmd)) = maybe_request {
            self.issue(req.memory_address, cmd); // Mutable borrow starts here
        }
        self.dram.tick();

        // if let Some(addr) = self.dram.tick(){
        //     println!("#DRAM completed read for {:?}!", addr);
        // }

        None
    }


    // fn select_and_issue(&mut self, bus_direction: BusDirection) {
    //
    //     let maybe_request = self.scheduler.select(&self.queue_manager, &self.dram, bus_direction);
    //     // println!("selecting....direction {:?}", bus_direction);
    //     // Handle the request after the immutable borrow ends
    //     if let Some((req, cmd)) = maybe_request {
    //         // println!("{:?}", req);
    //         self.issue(req.memory_address, cmd); // Mutable borrow starts here
    //     }
    // }

    pub fn enqueue(&mut self, mut req: Request) -> Option<Request>{
        // println!("records: {}", self.records);
        // let block_address = req.memory_address.block_address;
        // let req_type = match req.request_type{
        //     RequestType::Read => {"read"}
        //     RequestType::Write => {"write"}
        // };
        req.stat.queued = self.cycle.clone();
        let failure = self.queue_manager.enqueue(req);

        match failure {
            None => {
                self.records +=1;
                None
            }
            Some(request) => {
                Some(request)
            }
        }


    }

    pub fn dequeue(&mut self, addr: MemoryAddress, request_type:RequestType) -> u32{

        let success = self.queue_manager.dequeue(addr, request_type);

        match success {
            None => {
                // println!("dequeue not succeeded.");
                0
            }
            Some(queued) => {
                // println!("request: {} dequeued.", {block_address});
                queued
            }
        }

    }

    pub fn issue(&mut self, memory_address: MemoryAddress, cmd: MemoryCommand) -> bool{

        match cmd {
            MemoryCommand::Pre_b => {self.dram.precharge_b(memory_address);}
            MemoryCommand::Pre_ab => {self.dram.precharge_ab(memory_address.rank);}
            MemoryCommand::Activate => {self.dram.activate(memory_address);}
            MemoryCommand::Read => {
                let queued = self.dequeue(memory_address, RequestType::Read);
                // println!("#ISSUE READ for {}. QUEUED: {}", memory_address.block_address, queued);
                self.dram.read(memory_address, queued);
            }
            MemoryCommand::Write => {
                let queued = self.dequeue(memory_address, RequestType::Write);
                self.dram.write(memory_address, queued);
            }
            MemoryCommand::Refresh => {self.dram.refresh_ab(memory_address)}
            MemoryCommand::ReadAP => {
                let queued = self.dequeue(memory_address, RequestType::Read);
                self.dram.read_ap(memory_address, queued);
            }
            MemoryCommand::WriteAP => {
                let queued = self.dequeue(memory_address, RequestType::Write);
                self.dram.write_ap(memory_address, queued);
            }
        }

        true
    }

}
impl<D: DRAM, Q: QueueManager, S:Scheduler, B:BusDirectionStrategy> Callback for MemoryController<D, Q, S, B>
{
    fn invoke(&self, block_address: usize) {
        println!("MemoryController: Read completed for {} at cycle {}", block_address, self.cycle);
    }
}