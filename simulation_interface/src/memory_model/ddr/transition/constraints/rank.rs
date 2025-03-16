
#[derive(Debug, Clone, Copy)]
pub struct RankConstraints{
    pub rd_rd: u16,
    pub wr_wr: u16,
    pub rd_wr: u16,
    pub wr_rd: u16,

    pub rd_prea: u16,
    pub wr_prea: u16,

    pub act_act: u16,

    pub faw_window:u16,

    pub act_prea:u16,
    pub prea_act:u16,

    pub act_refab: u16,
    pub pre_refab:u16,
    pub prea_refab:u16,
    pub rda_refab:u16,
    pub wra_refab:u16,
    pub refab_act:u16,
    pub refab_prea:u16,
}
