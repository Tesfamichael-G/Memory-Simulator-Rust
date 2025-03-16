
// // CAS <-> CAS
// /// Data bus occupancy
// {.level = "channel", .preceding = {"RD", "RDA"}, .following = {"RD", "RDA"}, .latency = V("nBL")},
// {.level = "channel", .preceding = {"WR", "WRA"}, .following = {"WR", "WRA"}, .latency = V("nBL")},

pub struct DDR4ChannelConstraints {

    pub rd_rd: u16,
    pub rd_rda: u16,
    pub rda_rd: u16,
    pub rda_rda:u16,

    pub wr_wr: u16,
    pub wr_wra: u16,
    pub wra_wr: u16,
    pub wra_wra: u16,

}