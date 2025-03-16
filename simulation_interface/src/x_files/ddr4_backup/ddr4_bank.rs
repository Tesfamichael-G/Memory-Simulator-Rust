// // /*** Bank ***/
// // to_act: 0
//
//
//
// use crate::memory_model::ddr::DDR4;
// use crate::memory_model::ddr::from_cas::{FromRD, FromRDA, FromWR, FromWRA};
// use crate::memory_model::ddr::from_ras::{FromACT, FromPRE, FromPREA};
//
// // /*** Bank ***/
// // {.level = "bank", .preceding = {"ACT"}, .following = {"ACT"}, .latency = V("nRC")},
// // {.level = "bank", .preceding = {"ACT"}, .following = {"RD", "RDA", "WR", "WRA"}, .latency = V("nRCD")},
// // {.level = "bank", .preceding = {"ACT"}, .following = {"PRE"}, .latency = V("nRAS")},
//
// // {.level = "bank", .preceding = {"PRE"}, .following = {"ACT"}, .latency = V("nRP")},
//
// pub fn get_all_transitions_from_act(t: &DDR4) -> FromACT {
//     FromACT{
//         to_act: t.rc,
//         to_rd: t.rcd,
//         to_rda: t.rcd,
//         to_wr: t.rcd,
//         to_wra: t.rcd,
//         to_pre: t.ras,
//         to_prea: t.ras,
//     }
// }
//
// pub fn get_all_transitions_from_pre(t: &DDR4) -> FromPRE {
//     FromPRE{
//         to_act: t.rp,
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
// // {.level = "bank", .preceding = {"RD"},  .following = {"PRE"}, .latency = V("nRTP")},
// // {.level = "bank", .preceding = {"WR"},  .following = {"PRE"}, .latency = V("nCWL") + V("nBL") + V("nWR")},
// // {.level = "bank", .preceding = {"RDA"}, .following = {"ACT"}, .latency = V("nRTP") + V("nRP")},
// // {.level = "bank", .preceding = {"WRA"}, .following = {"ACT"}, .latency = V("nCWL") + V("nBL") + V("nWR") + V("nRP")},
//
// pub fn get_all_transitions_from_rd(t: &DDR4) -> FromRD {
//     FromRD{
//         to_pre: t.rtp,
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