use std::io;
use std::io::{stdout, Write};
use simulation_interface::mem_request::{Callback, MemoryAddress, Request, RequestStat, RequestType};

use simulation_interface::trace::{MemoryTrace, TraceReader};

use simulation_interface::memory_model::ddr::{BankConstraints, BankGroupConstraints, ddr4, DDR4, DDRConstraints, DDRTimingConstraints, Org, RankConstraints, RankSiblingConstraints};
use simulation_interface::memory_model::{DRAM, DRAMState};
use simulation_interface::policy::{BusDirection, BusDirectionStrategy, Scheduler};
use simulation_interface::policy::QueueManager;


use crate::controller::{MemoryController};
use crate::memory::DDR;

use crate::trace_reader::{MemoryTraceReader};

use std::marker::PhantomData;
use std::time::Instant;

pub struct MemorySimulator<D: DRAM, Q: QueueManager, S:Scheduler, B:BusDirectionStrategy>//
{
    pub cycle: u32,
    pub memory_controller: MemoryController<D, Q, S, B>,
    pub reader: MemoryTraceReader,
    pending_request: Option<Request>,
    t: Instant,
    t0: Instant
}

impl<D: DRAM, Q: QueueManager, S:Scheduler, B:BusDirectionStrategy> MemorySimulator<D, Q, S, B>
{
    pub fn new(
               trace_reader: MemoryTraceReader,
               scheduler: S,
               queue: Q,
               bus_direction_strategy: B,
               dram: D,
               num_channels: usize,

               // num_ranks: usize,
               // num_bank_groups: usize,
               // num_banks: usize,
               // timing_constraints: DDRTimingConstraints,

    ) -> Result<Self, String> {

        let controller = MemoryController::new(
                scheduler,
                queue,
                bus_direction_strategy,
                dram,
                // num_ranks,
                // num_bank_groups,
                // num_banks,
                // timing_constraints,
                );

        return match controller {
            Ok(mem_ctl) => {
                // clear_screen();
                Ok(Self {
                    cycle: 0,
                    memory_controller: mem_ctl,
                    reader: trace_reader,
                    pending_request: None,
                    // phantom_data: Default::default(),
                    t: Instant::now(),
                    t0: Instant::now()
                })
            }
            Err(err) => {
                Err(err.to_string())
            }
        }

    }

    pub fn tick(&mut self) -> bool{

        self.cycle += 1;
        self.memory_controller.tick();
        // self.run_cycle()
        if (self.cycle % 250_000 == 0){
            let duration = Instant::now().duration_since(self.t);
            let cum_duration = Instant::now().duration_since(self.t0);
            println!("Cycle: {}. ... {:?}\t\t{:?}", self.cycle, duration, cum_duration);
            self.t = Instant::now();
        }
        return match self.pending_request.take() {
            None =>  self.fetch_send(),
            Some(req) => self.retry_pending_send(req)
        }
             // match self.memory_controller.tick() {
        //     None => self.run_cycle(),
        //     Some(skip_ahead) =>   return self.skip_cycles(skip_ahead)
        // }

    }

    fn run_cycle(&mut self) -> bool {
        return match self.pending_request.take() {
            None =>  self.fetch_send(),
            Some(req) => self.retry_pending_send(req)
        }
    }

    // fn skip_cycles(&mut self, skip_ahead:u32) -> bool{
    //
    //     println!("skipping {skip_ahead} cycles.");
    //
    //     for i in 0..skip_ahead{
    //         self.run_cycle();
    //     }
    //     self.memory_controller.skip_cycles(skip_ahead);
    //
    //     true
    // }

    pub fn retry_pending_send(&mut self, req: Request) -> bool {
        // println!("Cycle: {} | retry send:  {}", self.cycle, req.memory_address.block_address);
        if let Some(request) = self.send_request(req) {
            self.pending_request = Some(request);
        }
        true
    }

    pub fn fetch_send(&mut self) -> bool {
        let maybe_request = self.fetch_request();

        match maybe_request{
            None => {
                println!("fetch_send. EOF");
                false
            },
            Some(req) => {
                let block_address = req.memory_address.block_address;
                if let Some(request) = self.send_request(req) {
                    // println!("fetch_send. Pending {block_address}");
                    self.pending_request = Some(request);
                    //self.pending_request.unwrap().stat.queued = 0;
                }
                // println!("fetch_send. success {block_address}");
                true
            }
        }
    }

    pub fn send_request(&mut self, req: Request) -> Option<Request>{
       self.memory_controller.enqueue(req)
    }

    pub fn request_completed(){

    }

    pub fn fetch_request(&mut self) -> Option<Request> {

        let trace = self.reader.next_trace();

        return match trace {
            None => {
                // println!("Cycle: {} | Fetched:  None", self.cycle);
                None
            }
            Some(mem_trace) => {
                // println!("Cycle: {} | Fetched:  {:?}", self.cycle, mem_trace.req.memory_address);
                Some(mem_trace.req)
            }
        }

    }


}
