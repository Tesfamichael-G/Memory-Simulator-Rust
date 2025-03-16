
// /*** Bank ***/
// {.level = "bank", .preceding = {"ACT"}, .following = {"ACT"}, .latency = V("nRC")},
// {.level = "bank", .preceding = {"ACT"}, .following = {"RD", "RDA", "WR", "WRA"}, .latency = V("nRCD")},
// {.level = "bank", .preceding = {"ACT"}, .following = {"PRE"}, .latency = V("nRAS")},
// {.level = "bank", .preceding = {"PRE"}, .following = {"ACT"}, .latency = V("nRP")},
// {.level = "bank", .preceding = {"RD"},  .following = {"PRE"}, .latency = V("nRTP")},
// {.level = "bank", .preceding = {"WR"},  .following = {"PRE"}, .latency = V("nCWL") + V("nBL") + V("nWR")},
// {.level = "bank", .preceding = {"RDA"}, .following = {"ACT"}, .latency = V("nRTP") + V("nRP")},
// {.level = "bank", .preceding = {"WRA"}, .following = {"ACT"}, .latency = V("nCWL") + V("nBL") + V("nWR") + V("nRP")},

pub struct DDR4BankConstraints {

    pub act_act: u16,

    pub act_rd: u16,
    pub act_rda: u16,
    pub act_wr: u16,
    pub act_wra:u16,

    pub act_pre:u16,

    pub pre_act:u16,

    pub rd_pre: u16,
    pub wr_pre: u16,

    pub rda_act: u16,
    pub wra_act: u16,
}