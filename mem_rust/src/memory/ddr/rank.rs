use std::cmp::max;
use simulation_interface::memory_model::ddr::{BankConstraints, BankGroupConstraints, DDRConstraints, DDRTimingConstraints, RankConstraints, RankSiblingConstraints};
use std::io;
use simulation_interface::mem_request::MemoryAddress;
use crate::memory::ddr::Bank;
use crate::memory::ddr::BankGroup;

pub struct Rank {
    cycle: u32,
    pub bank_groups: Vec<BankGroup>,

    next_act: u32,
    next_pre: u32,
    next_pre_all: u32,
    next_read: u32,
    next_write: u32,
    next_ref_ab: u32,
    next_sre: u32,
    next_srx: u32,
    next_faw:u32,

    // pub t: RankConstraints,
    rd_rd: u32,
    wr_wr: u32,
    rd_wr: u32,
    wr_rd: u32,
    rd_prea: u32,
    wr_prea: u32,
    act_act: u32,
    faw_window: u32,
    act_prea: u32,
    prea_act: u32,
    act_refab: u32,
    pre_refab: u32,
    prea_refab: u32,
    rda_refab: u32,
    wra_refab: u32,
    refab_act: u32,
    refab_prea: u32,
    // pub sib: RankSiblingConstraints,
    sib_rd_rd: u32,
    sib_rd_wr: u32,
    sib_wr_rd: u32,

    first_of_last_four_acts: u32,
    second_of_last_four_acts: u32,
    last_act: u32,
    is_busy: bool
}
impl Rank {

    pub fn new(
               // rank_timing:  &RankConstraints,rank_sibling_constraints: &RankSiblingConstraints,bank_group_constraints: &BankGroupConstraints,  bank_constraints: &BankConstraints
               num_bank_groups: usize,
               num_banks: usize,
               timing_constraints: &DDRTimingConstraints

    ) -> io::Result<Self> {

        let mut bank_groups_list = Vec::with_capacity(num_bank_groups);
        // bank_groups_list.resize_with(num_bank_groups, || BankGroup::new(timing_constraints.bank_group, num_banks, timing_constraints.bank, 0)
        //                                                     .expect("failed to create vec<bank-groups>"));
        for i in 0..num_bank_groups{
            let bg =BankGroup::new(timing_constraints.bank_group, num_banks, timing_constraints.bank, i)
                .expect("failed to create vec<bank-groups>");
            bank_groups_list.push(bg);
        }

        let rank_timing = timing_constraints.rank;
        let rank_sibling_constraints = timing_constraints.rank_sibling;

        Ok(Self {
            cycle: 0,
            bank_groups: bank_groups_list,
            next_act: 0,
            next_pre: 0,
            next_pre_all: 0,
            next_read: 0,
            next_write: 0,
            next_ref_ab: 0,
            next_sre: 0,
            next_srx: 0,
            next_faw: 0,
            rd_rd: rank_timing.rd_rd as u32,
                wr_wr: rank_timing.wr_wr as u32,
                rd_wr: rank_timing.rd_wr as u32,
                wr_rd: rank_timing.wr_rd as u32,
                rd_prea: rank_timing.rd_prea as u32,
                wr_prea: rank_timing.wr_prea as u32,
                act_act: rank_timing.act_act as u32,
                faw_window: rank_timing.faw_window as u32,
                act_prea: rank_timing.act_prea as u32,
                prea_act: rank_timing.prea_act as u32,
                act_refab: rank_timing.act_refab as u32,
                pre_refab: rank_timing.pre_refab as u32,
                prea_refab: rank_timing.prea_refab as u32,
                rda_refab: rank_timing.rda_refab as u32,
                wra_refab: rank_timing.wra_refab as u32,
                refab_act: rank_timing.refab_act as u32,
                refab_prea: rank_timing.refab_prea as u32,

            // sib: RankSiblingConstraints {},
                sib_rd_rd: rank_sibling_constraints.rd_rd as u32,
                sib_rd_wr: rank_sibling_constraints.rd_wr as u32,
                sib_wr_rd: rank_sibling_constraints.wr_rd as u32,

            first_of_last_four_acts: 0,
            second_of_last_four_acts: 0,
            last_act: 0,
            is_busy: false,
        })
    }

    pub fn tick(&mut self, busy_state: &mut u64) -> u32{
        self.cycle += 1;

        // let min = self.bank_groups.iter_mut().map(|bank_group| bank_group.tick()).min().unwrap_or(0);
        // self.is_busy = min > 0;
        //
        // min
        self.bank_groups.iter_mut().for_each(|bank_group| { bank_group.tick(busy_state); });
        0

    }

    pub fn precharge_ab(&mut self){
        for i in 0..self.bank_groups.len(){
            self.bank_groups[i].precharge_ab();
        }
    }

    pub fn precharge_b(&mut self, addr: MemoryAddress ){
        self.bank_groups[addr.bank_group].precharge_b(addr);
    }

    pub fn activate(&mut self, addr: MemoryAddress ){

        self.first_of_last_four_acts = self.second_of_last_four_acts;
        self.second_of_last_four_acts = self.last_act;
        self.last_act = self.cycle;

        self.next_faw = max(self.next_faw, self.first_of_last_four_acts + self.faw_window);

        self.bank_groups[addr.bank_group].activate(addr);

        self.next_act = max(self.next_act, self.cycle + self.act_act);
        self.next_ref_ab = max(self.next_ref_ab, self.cycle + self.act_refab);
        self.next_pre_all = max(self.next_pre_all, self.cycle + self.act_prea);

    }

    // pub fn read(&mut self, addr: MemoryAddress ){
    pub fn read(&mut self, addr: MemoryAddress) {
        self.bank_groups[addr.bank_group].read(addr);

        self.next_read = max(self.next_read, self.cycle + self.rd_rd);
        self.next_write = max(self.next_write, self.cycle + self.rd_wr);
        self.next_pre_all = max(self.next_pre_all, self.cycle + self.rd_prea);
    }

    pub fn read_ap(&mut self, addr: MemoryAddress ){

        self.bank_groups[addr.bank_group].read_ap(addr);

        self.next_read = max(self.next_read, self.cycle + self.rd_rd);
        self.next_write = max(self.next_write, self.cycle + self.rd_wr);
        self.next_ref_ab = max(self.next_ref_ab, self.cycle + self.rda_refab);

    }

    pub fn write(&mut self, addr: MemoryAddress ){
        self.bank_groups[addr.bank_group].write(addr);

        self.next_write = max(self.next_write, self.cycle + self.wr_wr);
        self.next_read = max(self.next_read, self.cycle + self.wr_rd);
        self.next_pre_all = max(self.next_pre_all, self.cycle + self.wr_prea);

    }

    pub fn write_ap(&mut self, addr: MemoryAddress ){
        self.bank_groups[addr.bank_group].write_ap(addr);

        self.next_write = max(self.next_write, self.cycle + self.wr_wr);
        self.next_read = max(self.next_read, self.cycle + self.wr_rd);

        self.next_ref_ab = max(self.next_ref_ab, self.cycle + self.wra_refab);

    }

    pub fn refresh_ab(&mut self ){

    }

    pub fn on_sibling_read(&mut self){
        self.next_read = max(self.next_read, self.cycle + self.sib_rd_rd);
        self.next_write = max(self.next_write, self.cycle + self.sib_rd_wr);
    }

    pub fn on_sibling_read_ap(&mut self){
        self.next_read = max(self.next_read, self.cycle + self.sib_rd_rd);
        self.next_write = max(self.next_write, self.cycle + self.sib_rd_wr);
    }

    pub fn on_sibling_write(&mut self){
        self.next_read = max(self.next_read, self.cycle + self.sib_wr_rd);
    }

    pub fn on_sibling_write_ap(&mut self){
        self.next_read = max(self.next_read, self.cycle + self.sib_rd_wr);
    }


    pub fn can_pre(&self, addr: MemoryAddress) -> bool {
        self.next_pre <= self.cycle && self.bank_groups[addr.bank_group].can_pre(addr)
    }

    pub fn can_activate(&self, addr: MemoryAddress) -> bool {

        // println!("rank.can activate: self.next_act[{}] <>>= self.cycle[{}] and  self.next_faw [{}] <>>= self.cycle [{}] {}",
        //          self.next_act, self.cycle,
        //          self.next_faw,  self.cycle,
        //          self.next_act <= self.cycle && self.next_faw <= self.cycle);
        self.next_act <= self.cycle &&
            self.next_faw <= self.cycle &&
            self.bank_groups[addr.bank_group].can_activate(addr)
    }

    pub fn can_read(&self, addr: MemoryAddress) -> bool {
        self.next_read <= self.cycle && self.bank_groups[addr.bank_group].can_read(addr)
    }

    pub fn can_write(&self, addr: MemoryAddress) -> bool {
        self.next_write <= self.cycle && self.bank_groups[addr.bank_group].can_write(addr)
    }


    pub fn can_pre_all(&self) -> bool {
        for _bank_group in self.bank_groups.iter() {
            if !_bank_group.can_pre_all() {
                return false
            }
        }
        true
    }

    pub fn can_ref_ab(&self) -> bool {
        self.cycle >= self.next_ref_ab
    }

    pub fn can_ref_b(&self, addr: MemoryAddress) -> bool {
        self.bank_groups[addr.bank_group].can_ref_b(addr)
    }

}