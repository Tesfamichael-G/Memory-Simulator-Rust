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
//
// pub struct FromWRA{
//     pub to_act: u16,
// }
//
// pub struct FromWR{
//     pub to_pre: u16,
// }
//
// pub struct FromRDA{
//     pub to_act: u16,
// }
//
// pub struct FromRD{
//     pub to_pre: u16,
// }
//
