
use std::cmp::max;
use std::io;

use simulation_interface::mem_request::MemoryAddress;
use simulation_interface::memory_model::ddr::{BankConstraints, BankState};

pub struct Bank {
    pub id:usize,
    cycle: u32,
    act_act: u32,
    act_cas: u32,
    act_pre: u32,
    pre_act: u32,
    rd_pre: u32,
    wr_pre: u32,
    rda_act: u32,
    wra_act: u32,
    // cas count down
    min_read_time: u32,
    min_write_time: u32,
    // pub t: BankConstraints,
    next_act: u32,
    next_pre: u32,
    next_read: u32,
    next_write: u32,
    next_read_ap: u32,
    next_write_ap: u32,
    pub state: BankState,
    next_state:BankState,

    pub active_row: Option<usize>,
    pub is_busy: bool,
    pub count_down: u32,
    pub bit_index: usize,
}
impl Bank {

    pub fn new(bit_index:usize, timing_constraints: BankConstraints, id:usize) -> io::Result<Self> {

        Ok(Self {
            id,
            cycle: 0,
            // t: BankConstraints {},
            act_act: timing_constraints.act_act as u32,
            act_cas: timing_constraints.act_cas as u32,
            act_pre: timing_constraints.act_pre as u32,
            pre_act: timing_constraints.pre_act as u32,
            rd_pre: timing_constraints.rd_pre as u32,
            wr_pre: timing_constraints.wr_pre as u32,
            rda_act: timing_constraints.rda_act as u32,
            wra_act: timing_constraints.wra_act as u32,
            min_read_time: timing_constraints.min_read_time as u32,
            min_write_time: timing_constraints.min_write_time as u32,

            next_act: 0,
            next_pre: 0,
            next_read: 0,
            next_write: 0,
            next_read_ap: 0,
            next_write_ap: 0,
            state: BankState::Idle,
            next_state: BankState::Idle,
            count_down: 0,
            active_row: None,
            is_busy: false,

            bit_index,
        })
    }

    pub fn tick(&mut self, busy_state: &mut u64) -> u32{
        self.cycle += 1;


        if self.count_down > 0 {
            // println!(" *** bank busy for at least {}",  self.count_down);
            self.count_down -= 1;
            return self.count_down;
        }

        self.state = self.next_state;
        self.is_busy = false;
        self.count_down = 0;

        self.count_down
    }

    pub fn precharge(&mut self){

        self.active_row = None;
        self.count_down = self.pre_act;
        self.next_act = max(self.next_act, self.cycle + self.pre_act );
        self.state = BankState::Precharging;
        self.next_state = BankState::Idle;
        self.is_busy = true;

    }

    pub fn activate(&mut self, addr: MemoryAddress ){
        let tmp = self.cycle + self.act_cas as u32;

        self.next_read = max(self.next_read, tmp);
        self.next_write = max(self.next_write, tmp);
        self.next_read_ap = max(self.next_read_ap, tmp);
        self.next_write_ap = max(self.next_write_ap, tmp);

        self.next_act = max(self.next_act, self.cycle + self.act_act);
        self.next_pre = max(self.next_pre, self.cycle + self.act_pre);

        self.state = BankState::Activating;
        self.next_state = BankState::Active;
        self.active_row = Some(addr.row);
        self.count_down = self.act_cas;
        self.is_busy = true;

    }
    // pub fn read(&mut self, addr: MemoryAddress ){

    pub fn read(&mut self, addr: MemoryAddress) {
        self.next_pre = max(self.next_pre, self.cycle + self.rd_pre);

        self.state = BankState::Reading;
        self.next_state = BankState::Active;
        self.count_down = self.min_read_time;
        self.is_busy = true;

    }

    pub fn write(&mut self, addr: MemoryAddress ){

        self.next_pre = max(self.next_pre, self.cycle + self.wr_pre);

        self.state = BankState::Writing;
        self.next_state = BankState::Active;
        self.count_down = self.min_write_time;
        self.is_busy = true;

    }

    pub fn read_ap(&mut self, addr: MemoryAddress ){

        self.next_act = max(self.next_act, self.cycle + self.rda_act);

        self.state = BankState::Reading;
        self.next_state = BankState::Idle;
        self.count_down = self.rda_act;
        self.is_busy = true;

    }
    pub fn write_ap(&mut self, addr: MemoryAddress ){

        self.next_act = max(self.next_act, self.cycle + self.wra_act);

        self.state = BankState::Writing;
        self.next_state = BankState::Idle;
        self.count_down = self.wra_act;
        self.is_busy = true;
    }


    pub fn can_pre(&self) -> bool {
        self.next_pre <= self.cycle
    }

    pub fn can_activate(&self) -> bool {
        // println!("bank-group.can activate: self.next_act[{}] <>>= self.cycle[{}] {}",
        //          self.next_act, self.cycle, self.next_act <= self.cycle);
        self.next_act <= self.cycle
    }

    pub fn can_read(&self, addr: MemoryAddress) -> bool {
        // assert_eq!(self.id, addr.bank, "bank id error");
        self.next_read <= self.cycle && match self.active_row {
            Some(row) => addr.row == row,
            None => false,
        }
    }

    pub fn can_write(&self, addr: MemoryAddress) -> bool {
        // assert_eq!(self.id, addr.bank, "bank id error");
        self.next_write <= self.cycle && match self.active_row {
            Some(row) => addr.row == row,
            None => false,
        }
    }

    pub fn can_ref_b(&self, addr: MemoryAddress) -> bool {
        todo!()
    }

    pub fn is_busy(&self) -> bool {
        self.is_busy
    }

}