use std::cmp::min;
use std::fmt::Debug;
use std::io;
use serde::Deserialize;

use crate::memory_model::ddr::{
    DDRConstraints,DDRTimingConstraints,
    BankConstraints, BankGroupConstraints, RankConstraints, RankSiblingConstraints
};
use crate::memory_model::DRAM;

#[derive(Debug, Deserialize)]
pub struct DDR4 {

    pub rate: u16,
    pub freq: u16,
    pub t_ck: f32,
    pub bl: u16,
    pub cl: u16,
    pub rcd: u16,
    pub rp: u16,
    pub ras: u16,
    pub rc: u16,
    pub wr: u16,
    pub rtp: u16,
    pub cwl: u16,
    pub rtrs: u16,

    pub xp: u16,
    pub ckesr: u16,
    pub cke: u16,
    pub ccds: u16,
    pub ccdl: u16,

    pub rrds: u16,
    pub rrdl: u16,

    pub wtrs: u16,
    pub wtrl: u16,

    pub faw: u16,

    pub rfc: u16,
    pub rfc2:u16,
    pub rfc4: u16,

    pub refi: u32,

    pub xs: u16,
    // pub pd: u16,
}

impl DDRConstraints  for DDR4{

    fn get_constraints(&self) -> io::Result<DDRTimingConstraints> {

        let rank_constraints = RankConstraints{
            rd_rd:  self.ccds,
            wr_wr:  self.ccds,
            rd_wr:  self.cl +  self.bl +  self.rtrs -  self.cwl,
            wr_rd:  self.cwl +  self.bl +  self.wtrs,

            rd_prea:  self.rtp,
            wr_prea:  self.cwl +  self.bl +  self.wr,

            act_act:  self.rrds,
            faw_window:  self.faw,
            act_prea:  self.ras,
            prea_act:  self.rp,

            act_refab:  self.rc,
            pre_refab:  self.rp,
            prea_refab:  self.rp ,

            rda_refab:  self.rp +  self.rtp,
            wra_refab:  self.cwl +  self.bl +  self.wr +  self.rp,

            refab_act:  self.rfc,
            refab_prea:  self.rfc,
        };

        let rank_sibling_constraints = RankSiblingConstraints{
            rd_rd:  self.bl +  self.rtrs,
            rd_wr:  self.bl +  self.rtrs,
            wr_rd:  self.cl +  self.bl +  self.rtrs -  self.cwl,
        };

        let bank_group_constraints = BankGroupConstraints{
            act_act: self.rrdl,
            rd_rd: self.ccdl,
            wr_wr: self.ccdl,
            wr_rd: self.cwl + self.bl + self.wtrl,
        };

        let bank_constraints = BankConstraints {
            act_act: self.rc,
            act_cas: self.rcd,

            act_pre: self.ras,
            pre_act: self.rp,

            rd_pre: self.rtp,
            wr_pre: self.cwl + self.bl + self.wr,
            rda_act: self.rtp + self.rp,
            wra_act: self.cwl + self.bl + self.wr + self.rp,

            min_read_time: min(min(self.cl, self.ccdl), self.rtp),
            min_write_time: min(self.cwl, self.ccdl),
        };

        let timing_constraints = DDRTimingConstraints {
            read_delay: (self.cl + self.bl) as u32,
            write_delay: (self.cwl + self.bl) as u32,

            rank: rank_constraints,
            rank_sibling: rank_sibling_constraints,
            bank_group: bank_group_constraints,
            bank: bank_constraints,
            transport_latency: self.bl as  u32,
        };

        Ok(timing_constraints)

    }

}









//
// impl DDRBankConstraints  for DDR4{
//     // type Model = DDR4;
//     fn get_constraints(&self) -> BankConstraints {
//         BankConstraints{
//             act_act: self.rc,
//             act_cas: self.rcd,
//
//             act_pre: self.ras,
//             pre_act: self.rp,
//
//             rd_pre: self.rtp,
//             wr_pre: self.cwl + self.bl + self.wr,
//             rda_act: self.rtp + self.rp,
//             wra_act: self.cwl + self.bl + self.wr + self.rp,
//         }
//     }
//
// }
//
// impl DDRBankGroupConstraints for DDR4 {
//     // type Model = DDR4;
//     fn get_constraints(&self) -> BankGroupConstraints {
//         BankGroupConstraints{
//             act_act: self.rrdl,
//             rd_rd: self.ccdl,
//             wr_wr: self.ccdl,
//             wr_rd: self.cwl + self.bl + self.wtrl,
//         }
//     }
// }
//
// impl DDRRankConstraints for DDR4 {
//
//     fn get_constraints(&self) -> RankConstraints {
//         RankConstraints{
//             rd_rd:  self.ccds,
//             wr_wr:  self.ccds,
//             rd_wr:  self.cl +  self.bl +  self.rtrs -  self.cwl,
//             wr_rd:  self.cwl +  self.bl +  self.wtrs,
//
//             rd_prea:  self.rtp,
//             wr_prea:  self.cwl +  self.bl +  self.wr,
//
//             act_act:  self.rrds,
//             faw_window:  self.faw,
//             act_prea:  self.ras,
//             prea_act:  self.rp,
//
//             act_refab:  self.rc,
//             pre_refab:  self.rp,
//             prea_refab:  self.rp ,
//
//             rda_refab:  self.rp +  self.rtp,
//             wra_refab:  self.cwl +  self.bl +  self.wr +  self.rp,
//
//             refab_act:  self.rfc,
//             refab_prea:  self.rfc,
//         }
//     }
// }
//
// impl DDRBankSiblingConstraints for DDR4 {
//     fn get_constraints(&self) -> RankSiblingConstraints {
//         RankSiblingConstraints{
//             rd_rd:  self.bl +  self.rtrs,
//             rd_wr:  self.bl +  self.rtrs,
//             wr_rd:  self.cl +  self.bl +  self.rtrs -  self.cwl,
//         }
//     }
// }















// impl_ddr_transitions!(DDR4, ddr4_rank, ddr4_bank, ddr4_bank_group);

//
// impl Transition for DDR4{
//     fn get_transition_from_act(&self, component: Component) -> FromACT {
//         match component {
//             Component::Rank => ddr4_rank::get_all_transitions_from_act(),
//             Component::Bank => ddr4_bank::get_all_transitions_from_act(),
//             Component::BankGroup => ddr4_bank_group::get_all_transitions_from_act(),
//         }
//     }
//
//     fn get_transition_from_pre(&self, component: Component) -> FromPRE {
//         match component {
//             Component::Rank => ddr4_rank::get_all_transitions_from_pre(),
//             Component::Bank => ddr4_bank::get_all_transitions_from_pre(),
//             Component::BankGroup => ddr4_bank_group::get_all_transitions_from_pre(),
//         }
//     }
//
//     fn get_transition_from_prea(&self, component: Component) -> FromPREA {
//         match component {
//             Component::Rank => ddr4_rank::get_all_transitions_from_prea(),
//             Component::Bank => ddr4_bank::get_all_transitions_from_prea(),
//             Component::BankGroup => ddr4_bank_group::get_all_transitions_from_prea(),
//         }
//     }
//
//     fn get_transition_from_rd(&self, component: Component) -> FromRD {
//         match component {
//             Component::Rank => ddr4_rank::get_all_transitions_from_rd(),
//             Component::Bank => ddr4_bank::get_all_transitions_from_rd(),
//             Component::BankGroup => ddr4_bank_group::get_all_transitions_from_rd(),
//         }
//     }
//
//     fn get_transition_from_rda(&self, component: Component) -> FromRDA {
//         match component {
//             Component::Rank => ddr4_rank::get_all_transitions_from_rda(),
//             Component::Bank => ddr4_bank::get_all_transitions_from_rda(),
//             Component::BankGroup => ddr4_bank_group::get_all_transitions_from_rda(),
//         }
//     }
//
//     fn get_transition_from_wr(&self, component: Component) -> FromWR {
//         match component {
//             Component::Rank => ddr4_rank::get_all_transitions_from_wr(),
//             Component::Bank => ddr4_bank::get_all_transitions_from_wr(),
//             Component::BankGroup => ddr4_bank_group::get_all_transitions_from_wr(),
//         }
//     }
//
//     fn get_transition_from_wra(&self, component: Component) -> FromWRA {
//         match component {
//             Component::Rank => ddr4_rank::get_all_transitions_from_wra(),
//             Component::Bank => ddr4_bank::get_all_transitions_from_wra(),
//             Component::BankGroup => ddr4_bank_group::get_all_transitions_from_wra(),
//         }
//     }
// }
