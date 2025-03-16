pub mod ddr4;

use std::fmt::Debug;
use crate::address_mapping::MappingBits;
use crate::memory_model::ddr::Org;



pub trait SpecParser{
    type Model : Debug;
    fn parse(spec_path: &str) -> Result<(Org, Self::Model, MappingBits), serde_json::Error>;
}
