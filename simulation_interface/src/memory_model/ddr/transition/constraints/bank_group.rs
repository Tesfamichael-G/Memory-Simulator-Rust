#[derive(Debug, Clone, Copy)]
pub struct BankGroupConstraints{
    pub act_act: u16,

    pub rd_rd:u16,
    pub wr_wr:u16,

    pub wr_rd:u16,

}
