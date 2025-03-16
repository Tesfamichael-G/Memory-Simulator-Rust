// /*** Rank (or different BankGroup) ***/
// // CAS <-> CAS
// /// nCCDS is the minimal latency for column commands
// {.level = "rank", .preceding = {"RD", "RDA"}, .following = {"RD", "RDA"}, .latency = V("nCCDS")},
// {.level = "rank", .preceding = {"WR", "WRA"}, .following = {"WR", "WRA"}, .latency = V("nCCDS")},

// /// RD <-> WR, Minimum Read to Write, Assuming tWPRE = 1 tCK
// {.level = "rank", .preceding = {"RD", "RDA"}, .following = {"WR", "WRA"}, .latency = V("nCL") + V("nBL") + 2 - V("nCWL")},

// /// WR <-> RD, Minimum Read after Write
// {.level = "rank", .preceding = {"WR", "WRA"}, .following = {"RD", "RDA"}, .latency = V("nCWL") + V("nBL") + V("nWTRS")},

// /// CAS <-> PREab ~ab = all banks | pb = per bank
// {.level = "rank", .preceding = {"RD"}, .following = {"PREA"}, .latency = V("nRTP")},
// {.level = "rank", .preceding = {"WR"}, .following = {"PREA"}, .latency = V("nCWL") + V("nBL") + V("nWR")},

// /// RAS <-> RAS
// {.level = "rank", .preceding = {"ACT"}, .following = {"ACT"}, .latency = V("nRRDS")},
// {.level = "rank", .preceding = {"ACT"}, .following = {"ACT"}, .latency = V("nFAW"), .window = 4},
// {.level = "rank", .preceding = {"ACT"}, .following = {"PREA"}, .latency = V("nRAS")},
// {.level = "rank", .preceding = {"PREA"}, .following = {"ACT"}, .latency = V("nRP")},

// /// RAS <-> REF
// {.level = "rank", .preceding = {"ACT"}, .following = {"REFab"}, .latency = V("nRC")},
// {.level = "rank", .preceding = {"PRE", "PREA"}, .following = {"REFab"}, .latency = V("nRP")},
// {.level = "rank", .preceding = {"RDA"}, .following = {"REFab"}, .latency = V("nRP") + V("nRTP")},
// {.level = "rank", .preceding = {"WRA"}, .following = {"REFab"}, .latency = V("nCWL") + V("nBL") + V("nWR") + V("nRP")},
// {.level = "rank", .preceding = {"REFab"}, .following = {"ACT", "PREA"}, .latency = V("nRFC")},

pub struct DDR4RankConstraints {
    // // CAS <-> CAS /*** Rank (or different BankGroup) ***/
    rd_rd: u16,
    rd_rda: u16,
    rda_rd: u16,
    rda_rda:u16,

    wr_wr: u16,
    wr_wra: u16,
    wra_wr: u16,
    wra_wra: u16,

    // /// RD <-> WR, Minimum Read to Write, Assuming tWPRE = 1 tCK
    rd_wr: u16,
    rd_wra: u16,
    rda_wr: u16,
    rda_wra:u16,

    // /// WR <-> RD, Minimum Read after Write
    wr_rd: u16,
    wr_rda: u16,
    wra_rd: u16,
    wra_rda: u16,

    // /// CAS <-> PREab ~ab = all banks | pb = per bank
    rd_prea: u16,
    wr_prea: u16,

    // /// RAS <-> RAS
    act_act: u16,
    faw_window:u16,
    act_prea:u16,
    prea_act:u16,

    // /// RAS <-> REF
    act_refab: u16,
    pre_refab:u16,
    prea_refab:u16,
    rda_refab:u16,
    wra_refab:u16,
    refab_act:u16,
    refab_prea:u16,

}


// /// CAS <-> CAS between sibling ranks, nCS (rank switching) is needed for new DQS
// {.level = "rank", .preceding = {"RD", "RDA"}, .following = {"RD", "RDA", "WR", "WRA"}, .latency = V("nBL") + V("nCS"), .is_sibling = true},
// {.level = "rank", .preceding = {"WR", "WRA"}, .following = {"RD", "RDA"}, .latency = V("nCL")  + V("nBL") + V("nCS") - V("nCWL"), .is_sibling = true},

pub struct DDR4RankSiblingConstraints {

    rd_rd: u16,
    rd_rda: u16,
    rda_rd: u16,
    rda_rda:u16,

    rd_wr: u16,
    rd_wra: u16,
    rda_wr: u16,
    rda_wra:u16,

    wr_rd: u16,
    wr_rda: u16,
    wra_rd: u16,
    wra_rda: u16,

}