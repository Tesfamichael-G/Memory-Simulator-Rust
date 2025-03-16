use crate::address_mapping::AddressTranslation;
use crate::mem_request::address::MemoryAddress;

#[derive(Debug)]
pub struct MappingBits {
    pub row_bits : usize,
    pub rank_bits : usize,
    pub bank_bits : usize,
    pub bank_group_bits : usize,
    pub channel_bits : usize,
    pub col_bits : usize,
    pub tx_offset : usize
}
#[derive(Debug, Clone)]
pub struct Mapper {
    pub row_mask: usize,
    pub row_shift: usize,

    pub rank_mask: usize,
    pub rank_shift: usize,

    pub bank_mask: usize,
    pub bank_shift: usize,

    pub bank_group_mask: usize,
    pub bank_group_shift: usize,

    pub channel_mask: usize,
    pub channel_shift: usize,

    pub col_mask: usize,
    pub col_shift: usize,

    pub tx_offset: usize
}

impl Mapper {

    pub fn new(mapping_str: &str, address_bits: MappingBits) -> Mapper {
        let mut shift :usize  = 0;
        let mut row_mask = 0;
        let mut row_shift :usize = 0;
        let mut rank_mask :usize  = 0;
        let mut rank_shift :usize  = 0;
        let mut bank_mask :usize  = 0;
        let mut bank_shift :usize  = 0;
        let mut bank_group_mask :usize  = 0;
        let mut bank_group_shift :usize  = 0;
        let mut channel_mask :usize  = 0;
        let mut channel_shift :usize  = 0;
        let mut col_mask :usize = 0;
        let mut col_shift :usize = 0;

        for field in mapping_str.split('_').rev() {
            match field.to_uppercase().as_str() {
                "ROW" => {
                    row_mask = ((1 << address_bits.row_bits) - 1) << shift;
                    row_shift = shift;
                    shift += address_bits.row_bits;
                }
                "RANK" => {
                    rank_mask = ((1 << address_bits.rank_bits) - 1) << shift;
                    row_shift = shift;
                    shift += address_bits.rank_bits;
                }
                "BANK-GROUP" => {
                    bank_group_mask = ((1 << address_bits.bank_group_bits) - 1) << shift;
                    bank_group_shift = shift;
                    shift += address_bits.bank_group_bits;
                }
                "BANK" => {
                    bank_mask = ((1 << address_bits.bank_bits) - 1) << shift;
                    bank_shift = shift;
                    shift += address_bits.bank_bits;
                }
                "CHAN" => {
                    channel_mask = ((1 << address_bits.channel_bits) - 1) << shift;
                    channel_shift = shift;
                    shift += address_bits.channel_bits;
                }
                "COL" => {
                    col_mask = ((1 << address_bits.col_bits) - 1) << shift;
                    col_shift = shift;
                    shift += address_bits.col_bits;
                }
                _ => {}
            }
        }

        Mapper {
            row_mask,
            row_shift ,
            rank_mask,
            rank_shift ,
            bank_mask,
            bank_shift ,
            bank_group_mask,
            bank_group_shift ,
            channel_mask,
            channel_shift ,
            col_mask,
            col_shift ,
            tx_offset: address_bits.tx_offset
        }
    }

}
impl AddressTranslation for Mapper {

    fn translate(&self, physical_address: usize) -> MemoryAddress {
        MemoryAddress {
            block_address: (physical_address >> self.tx_offset),
            row: ((physical_address & self.row_mask ) >> self.row_shift),
            rank: ((physical_address & self.rank_mask ) >> self.rank_shift),
            bank: ((physical_address & self.bank_mask ) >> self.bank_shift),
            bank_group: ((physical_address & self.bank_group_mask ) >> self.bank_group_shift),
            channel: ((physical_address & self.channel_mask) >> self.channel_shift),
            column: ((physical_address & self.col_mask ) >> self.col_shift),
        }
    }

}