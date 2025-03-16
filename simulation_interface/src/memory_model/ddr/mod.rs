pub mod ddr2;
pub mod ddr3;
pub mod ddr4;
pub mod ddr5;

pub mod common;
pub mod transition;

use std::fmt::Debug;
use std::io;
pub use crate::memory_model::ddr::ddr4::DDR4;

pub use common::power::Power;
pub use common::org::Org;

pub use transition::RankConstraints;
pub use transition::RankSiblingConstraints;
pub use transition::BankGroupConstraints;
pub use transition::BankConstraints;


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BankState {
    Precharging = 0,
    Idle = 1,
    Activating = 2,
    Active = 3,
    Reading = 4,
    Writing = 5,
    Refreshing = 6,
}

#[derive(Copy, Clone, Debug)]
pub struct DDRTimingConstraints {

    pub read_delay: u32,
    pub write_delay: u32,

    pub transport_latency:u32,
    pub rank: RankConstraints,
    pub rank_sibling: RankSiblingConstraints,
    pub bank_group: BankGroupConstraints,
    pub bank: BankConstraints

}


pub trait DDRConstraints {
    fn get_constraints(&self) -> io::Result<DDRTimingConstraints>;

}
