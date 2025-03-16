// // /*** Channel ***/
// // // CAS <-> CAS
// // /// Data bus occupancy
// // {.level = "channel", .preceding = {"RD", "RDA"}, .following = {"RD", "RDA"}, .latency = V("nBL")},
// // {.level = "channel", .preceding = {"WR", "WRA"}, .following = {"WR", "WRA"}, .latency = V("nBL")},
// //
// // /*** Rank (or different BankGroup) ***/
// // // CAS <-> CAS
// // /// nCCDS is the minimal latency for column commands
// // {.level = "rank", .preceding = {"RD", "RDA"}, .following = {"RD", "RDA"}, .latency = V("nCCDS")},
// // {.level = "rank", .preceding = {"WR", "WRA"}, .following = {"WR", "WRA"}, .latency = V("nCCDS")},
//
// // /// RD <-> WR, Minimum Read to Write, Assuming tWPRE = 1 tCK
// // {.level = "rank", .preceding = {"RD", "RDA"}, .following = {"WR", "WRA"}, .latency = V("nCL") + V("nBL") + 2 - V("nCWL")},
//
// // /// WR <-> RD, Minimum Read after Write
// // {.level = "rank", .preceding = {"WR", "WRA"}, .following = {"RD", "RDA"}, .latency = V("nCWL") + V("nBL") + V("nWTRS")},
// // /// CAS <-> CAS between sibling ranks, nCS (rank switching) is needed for new DQS
// // {.level = "rank", .preceding = {"RD", "RDA"}, .following = {"RD", "RDA", "WR", "WRA"}, .latency = V("nBL") + V("nCS"), .is_sibling = true},
// // {.level = "rank", .preceding = {"WR", "WRA"}, .following = {"RD", "RDA"}, .latency = V("nCL")  + V("nBL") + V("nCS") - V("nCWL"), .is_sibling = true},
//
// // /// CAS <-> PREab
// // {.level = "rank", .preceding = {"RD"}, .following = {"PREA"}, .latency = V("nRTP")},
// // {.level = "rank", .preceding = {"WR"}, .following = {"PREA"}, .latency = V("nCWL") + V("nBL") + V("nWR")},
//
// // /// RAS <-> RAS
// // {.level = "rank", .preceding = {"ACT"}, .following = {"ACT"}, .latency = V("nRRDS")},
// // {.level = "rank", .preceding = {"ACT"}, .following = {"ACT"}, .latency = V("nFAW"), .window = 4},
// // {.level = "rank", .preceding = {"ACT"}, .following = {"PREA"}, .latency = V("nRAS")},
// // {.level = "rank", .preceding = {"PREA"}, .following = {"ACT"}, .latency = V("nRP")},
//
// // /// RAS <-> REF
// // {.level = "rank", .preceding = {"ACT"}, .following = {"REFab"}, .latency = V("nRC")},
// // {.level = "rank", .preceding = {"PRE", "PREA"}, .following = {"REFab"}, .latency = V("nRP")},
// // {.level = "rank", .preceding = {"RDA"}, .following = {"REFab"}, .latency = V("nRP") + V("nRTP")},
// // {.level = "rank", .preceding = {"WRA"}, .following = {"REFab"}, .latency = V("nCWL") + V("nBL") + V("nWR") + V("nRP")},
// // {.level = "rank", .preceding = {"REFab"}, .following = {"ACT", "PREA"}, .latency = V("nRFC")},
// //
// // /*** Same Bank Group ***/
// // /// CAS <-> CAS
// // {.level = "bankgroup", .preceding = {"RD", "RDA"}, .following = {"RD", "RDA"}, .latency = V("nCCDL")},
// // {.level = "bankgroup", .preceding = {"WR", "WRA"}, .following = {"WR", "WRA"}, .latency = V("nCCDL")},
// // {.level = "bankgroup", .preceding = {"WR", "WRA"}, .following = {"RD", "RDA"}, .latency = V("nCWL") + V("nBL") + V("nWTRL")},
// // /// RAS <-> RAS
// // {.level = "bankgroup", .preceding = {"ACT"}, .following = {"ACT"}, .latency = V("nRRDL")},
//
// pub struct FromPRE{
//     pub to_act: u16,
//     pub to_refab: u16,
// }
//
// pub struct FromPREA{
//     pub to_act: u16,
//     pub to_refab: u16,
// }
//
// pub struct FromACT{
//     pub to_act: u16,
//
//     pub to_rd: u16,
//     pub to_rda: u16,
//     pub to_wr: u16,
//     pub to_wra: u16,
//
//     pub to_pre: u16,
//     pub to_prea: u16,
//
// }
//
//
// // /*** Bank ***/
// // {.level = "bank", .preceding = {"ACT"}, .following = {"ACT"}, .latency = V("nRC")},
// // {.level = "bank", .preceding = {"ACT"}, .following = {"RD", "RDA", "WR", "WRA"}, .latency = V("nRCD")},
// // {.level = "bank", .preceding = {"ACT"}, .following = {"PRE"}, .latency = V("nRAS")},
// // {.level = "bank", .preceding = {"PRE"}, .following = {"ACT"}, .latency = V("nRP")},
// // {.level = "bank", .preceding = {"RD"},  .following = {"PRE"}, .latency = V("nRTP")},
// // {.level = "bank", .preceding = {"WR"},  .following = {"PRE"}, .latency = V("nCWL") + V("nBL") + V("nWR")},
// // {.level = "bank", .preceding = {"RDA"}, .following = {"ACT"}, .latency = V("nRTP") + V("nRP")},
// // {.level = "bank", .preceding = {"WRA"}, .following = {"ACT"}, .latency = V("nCWL") + V("nBL") + V("nWR") + V("nRP")},
// // }
// // );
