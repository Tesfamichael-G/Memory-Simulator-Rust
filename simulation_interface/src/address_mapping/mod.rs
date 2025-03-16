
use crate::mem_request::MemoryAddress;
pub use crate::address_mapping::mapper::MappingBits;
mod mapper;

pub use mapper::Mapper;


pub trait AddressTranslation {
    fn translate(&self, physical_address: usize) -> MemoryAddress;
}

