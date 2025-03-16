// /*** Same Bank Group ***/
// /// CAS <-> CAS
// {.level = "bankgroup", .preceding = {"RD", "RDA"}, .following = {"RD", "RDA"}, .latency = V("nCCDL")},
// {.level = "bankgroup", .preceding = {"WR", "WRA"}, .following = {"WR", "WRA"}, .latency = V("nCCDL")},
// {.level = "bankgroup", .preceding = {"WR", "WRA"}, .following = {"RD", "RDA"}, .latency = V("nCWL") + V("nBL") + V("nWTRL")},
// /// RAS <-> RAS
// {.level = "bankgroup", .preceding = {"ACT"}, .following = {"ACT"}, .latency = V("nRRDL")},

pub struct DDR4BankGroupConstraints {

    pub act_act: u16,

    pub rd_rd: u16,
    pub rd_rda: u16,
    pub rda_rd: u16,
    pub rda_rda:u16,

    pub wr_wr: u16,
    pub wr_wra: u16,
    pub wra_wr: u16,
    pub wra_wra: u16,

    pub wr_rd: u16,
    pub wr_rda: u16,
    pub wra_rd: u16,
    pub wra_rda: u16,

}