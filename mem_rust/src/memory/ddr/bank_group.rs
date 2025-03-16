use std::cmp::{max, min};
use std::io;
use simulation_interface::mem_request::MemoryAddress;
use simulation_interface::memory_model::ddr::{BankConstraints, BankGroupConstraints};
use crate::memory::ddr::Bank;

pub struct BankGroup {
    id:usize,
    cycle: u32,
    pub banks: Vec<Bank>,
    // pub t: BankGroupConstraints
    act_act: u32,
    rd_rd: u32,
    wr_wr: u32,
    wr_rd: u32,
    // timing const
    next_act: u32,
    next_pre: u32,
    next_read: u32,
    next_write: u32,
    next_read_ap: u32,
    next_write_ap: u32,

    is_busy: bool,
}

impl BankGroup {

    pub fn new(timing_constraints: BankGroupConstraints, num_banks: usize, bank_constraints: BankConstraints, id:usize) -> io::Result<Self> {
        let mut bank_list = Vec::with_capacity(num_banks);
        // bank_list.resize_with(num_banks, || Bank::new(bank_constraints).expect("failed to create vec<banks>"));

        for i in 0..num_banks{
            let bank_index = id*num_banks + i;
            let b= Bank::new(bank_index, bank_constraints, i).expect("failed to create vec<banks>");
            bank_list.push(b) ;
        }
        Ok(Self {
            // t: BankGroupConstraints { },
            id,
            cycle: 0,
            banks: bank_list,

            act_act: timing_constraints.act_act as u32,
                rd_rd: timing_constraints.rd_rd as u32,
                wr_wr: timing_constraints.wr_wr as u32,
                wr_rd: timing_constraints.wr_rd as u32,

            next_act: 0,
            next_pre: 0,
            next_read: 0,
            next_write: 0,
            next_read_ap: 0,
            next_write_ap: 0,

            is_busy: false,
        })
    }


    pub fn tick(&mut self,  busy_state: &mut u64) -> u32{
        self.cycle += 1;

        // let min = self.banks.iter_mut().map(|bank| bank.tick()).min().unwrap_or(0);
        // self.is_busy = min > 0;
        //
        // min
        self.banks.iter_mut().for_each(|bank| { bank.tick(busy_state); });
        0
    }

    pub fn precharge_b(&mut self, addr: MemoryAddress ){
        self.banks[addr.bank].precharge();
    }

    pub fn precharge_ab(&mut self){
        for bank in self.banks.iter_mut(){
            bank.precharge();
        }
    }

    pub fn activate(&mut self, addr: MemoryAddress ){
        self.banks[addr.bank].activate(addr);
        self.next_act = max(self.next_act, self.cycle + self.act_act);

    }
    // pub fn read(&mut self, addr: MemoryAddress ){
    pub fn read(&mut self, addr: MemoryAddress) {
        self.banks[addr.bank].read(addr);
        self.next_read = max(self.next_read, self.cycle + self.rd_rd);

    }
    pub fn write(&mut self, addr: MemoryAddress ){
        self.banks[addr.bank].write(addr);
        self.next_write = max(self.next_write, self.cycle + self.wr_wr);
    }

    pub fn read_ap(&mut self, addr: MemoryAddress ){
        self.banks[addr.bank].read_ap(addr);
        self.next_read = max(self.next_read, self.cycle + self.rd_rd);

    }
    pub fn write_ap(&mut self, addr: MemoryAddress ){
        self.banks[addr.bank].write_ap(addr);
        self.next_write = max(self.next_write, self.cycle + self.wr_wr);
        self.next_read = max(self.next_read, self.cycle + self.wr_rd);
    }

    pub fn can_pre_all(&self) -> bool {
        for bank in self.banks.iter() {
            if !bank.can_pre(){
                return false;
            }
        }
        true
    }

    pub fn can_pre(&self, addr: MemoryAddress) -> bool {
        self.next_pre <= self.cycle && self.banks[addr.bank].can_pre()
    }

    pub fn can_activate(&self, addr: MemoryAddress) -> bool {
        // println!("bank-group.can activate: self.next_act[{}] <>>= self.cycle[{}] {}",
        //          self.next_act, self.cycle, self.next_act <= self.cycle);

        self.next_act <= self.cycle && self.banks[addr.bank].can_activate()
    }

    pub fn can_read(&self, addr: MemoryAddress) -> bool {
        self.next_read <= self.cycle && self.banks[addr.bank].can_read(addr)
    }

    pub fn can_write(&self, addr: MemoryAddress) -> bool {
        self.next_write <= self.cycle && self.banks[addr.bank].can_write(addr)
    }

    pub fn can_ref_b(&self, addr: MemoryAddress) -> bool {
        self.banks[addr.bank].can_ref_b(addr)
    }

}