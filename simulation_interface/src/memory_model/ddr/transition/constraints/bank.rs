
#[derive(Debug, Clone, Copy)]
pub struct BankConstraints{
    pub act_act: u16,
    pub act_cas: u16,

    pub act_pre:u16,

    pub pre_act:u16,

    pub rd_pre: u16,
    pub wr_pre: u16,

    pub rda_act: u16,
    pub wra_act: u16,

    pub min_read_time: u16,
    pub min_write_time: u16
}
