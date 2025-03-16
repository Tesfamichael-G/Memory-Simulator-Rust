use std::collections::VecDeque;
use std::io;
use serde::de::value::UsizeDeserializer;
use simulation_interface::Stat;
use simulation_interface::mem_request::{Callback, MemoryAddress, MemoryCommand, Request, RequestStatus, RequestType, ReadCallback};//RequestTransaction,
use simulation_interface::memory_model::ddr::{ DDRTimingConstraints, BankState};
use simulation_interface::memory_model::{DRAM, DRAMState};
use simulation_interface::policy::{CommandBus, DataBus};

use crate::memory::ddr::Rank;

pub struct DDR {

    cycle: u32,
    ranks: Vec<Rank>,
    callback_queue: VecDeque<ReadCallback>,
    stat: Stat,
    read_delay:u32,
    write_delay:u32,
    pub is_busy: bool,

    num_ranks:usize,
    num_bank_groups: usize,
    num_banks: usize,

    queue_size:usize,
    pub busy_state: u64,
}

impl DDR {

    pub fn new(num_ranks: usize,
               num_bank_groups: usize,
               num_banks: usize,
               timing_constraints: DDRTimingConstraints
               ) -> io::Result<Self> {

        let mut ranks_list = Vec::with_capacity(num_ranks);

        ranks_list.resize_with(num_ranks, || Rank::new(
                                    num_bank_groups,
                                    num_banks,
                                    &timing_constraints
                                   ).expect("failed to create vec<ranks>"));

        let stat: Stat = Stat::new().expect("failed to create stat object");

        let read_delay = timing_constraints.read_delay.clone();
        let write_delay = timing_constraints.write_delay.clone();

        Ok(Self {
            cycle: 0,
            ranks: ranks_list,
            is_busy: false,
            num_ranks,
            num_bank_groups,
            stat,
            read_delay,
            write_delay,
            num_banks,
            queue_size:0,
            callback_queue: VecDeque::new(),
            busy_state: 0,
        })
    }

    fn get_flat_index(&self, addr: &MemoryAddress) -> usize {
        let bank_group_shift = (self.num_banks as u32).trailing_zeros();
        let rank_shift = (self.num_bank_groups as u32).trailing_zeros();

        let rank_idx = addr.rank;
        let bank_group_idx = addr.bank_group;
        let bank_idx = addr.bank;

        let flat_index = (rank_idx << (rank_shift + bank_group_shift))
            + (bank_group_idx << bank_group_shift)
            + bank_idx;

        flat_index
    }
}

impl  CommandBus for DDR  {
    // type CallbackType = impl Callback;

    fn precharge_b(&mut self, addr: MemoryAddress ){
        self.ranks[addr.rank].precharge_b(addr);
    }

    fn precharge_ab(&mut self, rank: usize){
        self.ranks[rank].precharge_ab();
    }
    fn activate(&mut self, addr: MemoryAddress ){
        self.ranks[addr.rank].activate(addr);
    }

    fn read(&mut self, addr: MemoryAddress, queued: u32) {//, bus: &impl DataBus
        let queue_latency = self.cycle - queued;
        let read_latency = queue_latency + self.read_delay;
        self.stat.basic.read +=1;
        self.stat.read_queue_latency.accumulate(queue_latency);
        self.stat.read_latency.accumulate(read_latency);

        // let cb = ReadCompleteCallback::new(callback);
        let transaction_callback = ReadCallback{
            block_address: addr.block_address,
            completion_time: self.cycle + self.read_delay,
            data: 50
        };

        // println!("#{}-- READING -- ADDR: {}. ETA: {} + {} => {}", self.cycle, addr.block_address, self.cycle, self.read_delay, transaction_callback.completion_time) ;
        self.callback_queue.push_back(transaction_callback);

        for rank_id in 0..self.ranks.len() {
            if rank_id == addr.rank{
                self.ranks[rank_id].read(addr);
            }else {
                self.ranks[rank_id].on_sibling_read();
            }
        }
    }

    fn read_ap(&mut self, addr: MemoryAddress, queued:u32) {//, bus: &impl DataBus
        let queue_latency = self.cycle - queued;
        let read_latency = queue_latency + self.read_delay;
        self.stat.basic.read +=1;
        self.stat.read_queue_latency.accumulate(queue_latency);
        self.stat.read_latency.accumulate(read_latency);

        let transaction_callback = ReadCallback{
            block_address: addr.block_address,
            completion_time: self.cycle + read_latency,
            data: 50
        };

        self.callback_queue.push_back(transaction_callback);

        for rank_id in 0..self.ranks.len() {
            if rank_id == addr.rank{
                self.ranks[rank_id].read_ap(addr);
            }else {
                self.ranks[rank_id].on_sibling_read_ap();
            }
        }
    }
    fn write(&mut self, addr: MemoryAddress, queued:u32){
        let queue_latency = self.cycle - queued;
        let write_latency = queue_latency + self.read_delay;
        self.stat.basic.write +=1;
        self.stat.write_latency.accumulate(queue_latency);
        self.stat.read_latency.accumulate(write_latency);

        for rank_id in 0..self.ranks.len() {

            if rank_id == addr.rank{
                self.ranks[rank_id].write(addr);
            }else {
                self.ranks[rank_id].on_sibling_write();
            }
        }
    }

    fn write_ap(&mut self, addr: MemoryAddress, queued:u32){
        let queue_latency = self.cycle - queued;
        let write_latency = queue_latency + self.read_delay;
        self.stat.basic.write +=1;
        self.stat.write_latency.accumulate(queue_latency);
        self.stat.read_latency.accumulate(write_latency);
        for rank_id in 0..self.ranks.len() {
            if rank_id == addr.rank{
                self.ranks[rank_id].write_ap(addr);
            }else {
                self.ranks[rank_id].on_sibling_write_ap();
            }
        }
    }

    fn refresh_ab(&mut self, addr: MemoryAddress ){
        self.ranks[addr.rank].refresh_ab();
    }

}

impl DRAMState for DDR {

    fn get_valid_command(&self, req: &Request) -> Option<MemoryCommand> {

        let addr = req.memory_address;
        let rank = &self.ranks[addr.rank];
        let bank = &self.ranks[addr.rank].bank_groups[addr.bank_group].banks[addr.bank];

        let is_hot = matches!(bank.active_row, Some(row) if row == addr.row);

        let command = match bank.state {
            BankState::Idle => {
                if rank.can_activate(addr) {
                    Some(MemoryCommand::Activate)
                }else { None }
            },

            BankState::Active => {
                if is_hot {
                    match req.request_type {
                        RequestType::Read => {
                            if rank.can_read(addr){
                                Some(MemoryCommand::Read)
                            }else {  None}
                        },
                        RequestType::Write => {
                            if rank.can_write(addr) {
                                Some(MemoryCommand::Write)
                            } else { None }
                        },
                    }
                } else {
                    if rank.can_pre(addr) {
                        Some(MemoryCommand::Pre_b)
                    }else { None }
                }
            },
            _ => None,
        };

        command

    }

    fn is_bank_busy(&self, bank_group: usize,  bank: usize) -> bool {

        let rank = &self.ranks[0];
        let bank_group = &rank.bank_groups[bank_group];
        let bank = &bank_group.banks[bank];
        bank.is_busy()
    }

    fn can_ref_b(&self, addr: MemoryAddress) -> bool {
        self.ranks[addr.rank].can_ref_b(addr)
    }

    fn can_ref_ab(&self, rank: usize) -> bool {
        self.ranks[rank].can_ref_ab()
    }

}

impl DRAM for DDR {

    fn tick(&mut self) -> Option<usize>{
        self.cycle += 1;
        //
        // let min = self.ranks.iter_mut().map(|rank| rank.tick()).min().unwrap_or(0);
        // self.is_busy = min > 0;

        self.ranks.iter_mut().for_each(|rank| { rank.tick(&mut self.busy_state); });

        // let _ = self.ranks.iter_mut().map(|rank| rank.tick());
        
        if let Some(transaction) = self.callback_queue.front() {
            if self.cycle >= transaction.completion_time  {
                return Some(self.callback_queue.pop_front().unwrap().block_address);
            }
        }
        None
    }
    }

// let addr = req.memory_address;
// let rank = &self.ranks[addr.rank];
// let bank_group = &rank.bank_groups[addr.bank_group];
// let bank = &bank_group.banks[addr.bank];
//
// if bank.is_busy {
//     return None;
// }
//
// let is_hot = matches!(bank.active_row, Some(row) if row == addr.row);
//
// let mut may_be_command = None;
// let is_issuable = match bank.state {
//     BankState::Idle => {
//         // print!("idle: ");
//         may_be_command = Some(MemoryCommand::Activate);
//         rank.can_activate(addr)
//     },
//     BankState::Active => {
//         // print!("active: ");
//         if is_hot {
//             // print!(" and hot. ");
//             match req.request_type {
//                 RequestType::Read => {
//                     may_be_command = Some(MemoryCommand::Read);
//                     rank.can_read(addr)
//                 },
//                 RequestType::Write => {
//                     may_be_command = Some(MemoryCommand::Write);
//                     rank.can_write(addr)
//                 },
//             }
//         } else {
//             // print!(" but not hot. ");
//             may_be_command = Some(MemoryCommand::Pre_b);
//             rank.can_pre(addr)
//         }
//     },
//     _ => false,
// };
//
// return  if is_issuable {
//     may_be_command
// }else{
//     None
// }