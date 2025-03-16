// use crate::memory_model::ddr::DDR4;
// use crate::memory_model::ddr::from_cas::{FromRD, FromRDA, FromWR, FromWRA};
// use crate::memory_model::ddr::from_ras::{FromACT, FromPRE, FromPREA};
// use crate::memory_model::ddr::transitions::timing::Component;
//
// pub fn get_all_transitions_from_act(t: &DDR4) -> FromACT {
//     FromACT{
//         to_act: 0,
//         to_rd: 0,
//         to_rda: 0,
//         to_wr: 0,
//         to_wra: 0,
//         to_pre: 0,
//         to_prea: 0,
//     }
// }
// pub fn get_all_transitions_from_pre(t: &DDR4) -> FromPRE {
//     FromPRE{
//         to_act: 0,
//         to_refab: 0,
//     }
// }
//
// pub fn get_all_transitions_from_prea(t: &DDR4) -> FromPREA {
//     FromPREA{
//         to_act: 0,
//         to_refab: 0,
//     }
// }
//
// pub fn get_all_transitions_from_rd(t: &DDR4) -> FromRD {
//     FromRD{
//         to_pre: 0,
//         to_prea: 0,
//         to_rd: 0,
//         to_rda: 0,
//         to_wr: 0,
//         to_wra: 0,
//     }
// }
//
// pub fn get_all_transitions_from_rda(t: &DDR4) -> FromRDA {
//     FromRDA{
//         to_act: 0,
//         to_rd: 0,
//         to_rda: 0,
//         to_wr: 0,
//         to_wra: 0,
//         to_refab: 0,
//     }
// }
//
// pub fn get_all_transitions_from_wr(t: &DDR4) -> FromWR {
//     FromWR{
//         to_pre: 0,
//         to_prea: 0,
//         to_rd: 0,
//         to_rda: 0,
//         to_wr: 0,
//         to_wra: 0,
//     }
// }
//
// pub fn get_all_transitions_from_wra(t: &DDR4) -> FromWRA {
//     FromWRA{
//         to_act: 0,
//         to_rd: 0,
//         to_rda: 0,
//         to_wr: 0,
//         to_wra: 0,
//         to_refab: 0,
//     }
// }