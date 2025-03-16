use serde::{Deserialize, Serialize};
use crate::address_mapping::MappingBits;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Org {
    pub size: u32,
    pub dq: u8,
    pub channel: u8,
    pub rank: u8,
    pub bank_group:u8,
    pub bank: u8,
    pub row: u32,
    pub column: u16,
}

//DDR4R[x8-8Gb]
impl Default for Org {
    fn default() -> Self {

        Org {
            size: 8192,
            dq: 8,
            channel: 1,
            rank: 1,
            bank_group: 8,
            bank: 8,
            row: 65536,
            column: 2048
        }
    }
}

impl Org {
    pub fn default_mapping_bits(&self) -> MappingBits {
        let  m_bits = MappingBits{
            row_bits: f64::log2(self.row as f64).ceil() as usize,
            rank_bits: f64::log2(self.rank as f64).ceil() as usize,
            bank_bits: f64::log2(self.bank as f64).ceil() as usize,
            bank_group_bits: f64::log2(self.bank_group as f64).ceil() as usize,
            channel_bits: f64::log2(self.channel as f64).ceil() as usize,
            col_bits: f64::log2(self.column as f64).ceil() as usize,
            tx_offset: 0,
        };
        m_bits
    }
}
// impl Org {
//     // Constructor function to compute trace parameters from Org
//     pub fn default_mapping_from_org(org: &Org, mapping_string: &str) -> MappingBits {
//         // Default to 1 if Bank Group Bits are not provided
//         let bank_group_bits = 1;
//
//         // Calculate tx_bits
//         let prefetch_size:u32 = 8;
//         let channel_width:u32 = 64;
//         let tx:u32 = (prefetch_size * channel_width as u32) / org.dq as u32;
//         let tx_bits = (tx as f64).log2() as u8;
//
//         // Calculate other bits
//         let channel_bits = (org.channel as f64).log2() as u8;
//         let rank_bits = (org.rank as f64).log2() as u8;
//         let bank_bits = (org.bank as f64).log2() as u8;
//         let row_bits = (org.row as f64).log2() as u32;
//         let col_bits = (org.column as f64).log2() as u16;
//
//         // Adjust col_bits
//         let col_bits_adjusted = col_bits.saturating_sub((prefetch_size as f64).log2() as u16);
//
//         // let mapping_string =  ROW_RANK_BANK_CHAN_COL;
//
//         // let precomputed_mapping = initialize_mapping(mapping_string, row_bits, rank_bits, bank_bits, bank_group_bits, channel_bits, col_bits, tx_bits);
//         let precomputed_mapping = initialize_mapping(mapping_string, row_bits, rank_bits, bank_bits, bank_group_bits, channel_bits, col_bits, tx_bits);
//
//         return precomputed_mapping;
//
//     }
//
//
//
// }