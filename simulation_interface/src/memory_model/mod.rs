

pub mod ddr;

use std::todo;
use ddr::BankState;
use crate::mem_request::{Callback, MemoryAddress, MemoryCommand, Request, RequestStatus, RequestType};
use crate::policy::CommandBus;//ReadCompleteCallback

pub trait DRAMState {

    fn get_valid_command(&self, req: &Request) -> Option<MemoryCommand>;
    fn is_bank_busy(&self, bank_group:usize,  bank:usize) -> bool;

    fn can_ref_b(&self, addr: MemoryAddress) -> bool;
    fn can_ref_ab(&self, rank: usize) -> bool;

}


pub trait DRAM: DRAMState + CommandBus {
    fn tick(&mut self) -> Option<usize>;
}


// pub trait DRAM2 {
//
//     type CallbackType; //: Callback;    //Fn(usize) + 'static;
//
//     fn get_bank_state(&self, addr: &MemoryAddress) -> BankState;
//     fn get_active_row(&self, addr: &MemoryAddress) -> Option<usize>;
//     fn is_hot(&self, addr: &MemoryAddress) -> bool;
//     fn is_issuable(&self, req: &Request) -> bool;
//     fn get_request_status(&self, req: &Request) -> RequestStatus;
//     fn get_valid_command(&self, req: &Request) -> Option<MemoryCommand>;
//
//     fn is_busy(&self) -> bool;
//
//     fn can_issue(&self, req: &Request) -> bool;
//     fn can_ref_b(&self, addr: MemoryAddress) -> bool;
//     fn can_ref_ab(&self, rank: usize) -> bool;
//
//     // ================================================================
//
//     fn tick(&mut self) -> u32;
//
//     // ================================================================
//
//     fn precharge_b(&mut self, addr: MemoryAddress );
//     fn precharge_ab(&mut self, rank: usize);
//
//     fn activate(&mut self, addr: MemoryAddress );
//
//     // fn read<F: FnOnce(i32)>(&mut self, addr: MemoryAddress, queued:u32, callback: F);
//     fn read(&mut self, addr: MemoryAddress, queued:u32, callback: Self::CallbackType);
//         // where Self::CallbackType: Fn(usize) + 'static ;
//
//     fn read_ap(&mut self, addr: MemoryAddress, queued:u32, callback: Self::CallbackType);
//         // where Self::CallbackType: Fn(usize) + 'static ;
//
//     fn write(&mut self, addr: MemoryAddress, queued:u32);
//
//     fn write_ap(&mut self, addr: MemoryAddress, queued:u32);
//
//     fn refresh_ab(&mut self, addr: MemoryAddress );
//
// }