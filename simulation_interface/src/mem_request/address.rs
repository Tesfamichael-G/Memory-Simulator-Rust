
#[derive(Debug,Copy, Clone)]
pub struct MemoryAddress {
    pub block_address: usize,
    pub channel: usize,
    pub rank: usize,
    pub bank_group: usize,
    pub bank: usize,
    pub row: usize,
    pub column: usize,
}
