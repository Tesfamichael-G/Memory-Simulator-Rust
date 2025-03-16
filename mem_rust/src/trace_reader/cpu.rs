use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
//
// use crate::address_mapping::data::MappingData;
// use crate::address_mapping::mapping_trait::IAddressMapping;

// use crate::mem_request::address::MemoryAddress;
// use crate::mem_request::request::Request;
// use crate::mem_request::request_type::RequestType;
// use crate::mem_request::stat::RequestStat;

use simulation_interface::address_mapping::{Mapper, AddressTranslation};

use simulation_interface::trace::{CPUTrace, MemoryTrace, TraceReader};
use simulation_interface::mem_request::{Request, MemoryAddress, RequestType, RequestStat};

// use crate::trace_reader::mem::traits::reader::MemoryTraceReader;

pub struct CPUTraceReader{
    reader: BufReader<File>,
    mapper: Mapper
}

impl CPUTraceReader {
    pub fn new(file_path: &str, mapper: Mapper) -> io::Result<Self> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        Ok(Self {
            reader, mapper
        })
    }

}
//
// impl TraceReader for CPUTraceReader {
//     type Trace = CPUTrace;
//
//     fn next_trace<M: IAddressMapping>(&mut self, mapper: &M) -> Option<CPUTrace> {
//
//         let mut line = String::new();
//
//         let res = &self.reader.read_line(&mut line).unwrap_or(0);
//         if *res == 0 {
//             return None;
//         }
//
//         let parts: Vec<&str> = line.trim().split_whitespace().collect();
//         if parts.len() >= 2 {
//             return None;
//         }
//
//         let request_type = RequestType::Read;
//         let cpu_cycles = parts[0].parse::<u32>()?;
//
//         let address_value = parts[1].parse::<u32>()?;
//
//         let memory_address = mapper.translate(address_value);
//
//         let wb_req = if parts.len() == 3 {
//             // Parse write back request details if 3 parts
//             let wb_type = RequestType::Write;
//             let wb_address_value =parts[2].parse::<u32>()?;
//             let wb_memory_address = mapper.translate(wb_address_value);
//             Some(Request {
//                 request_type: wb_type,
//                 memory_address: wb_memory_address,
//                 stat: RequestStat {
//                     arrival: 0,
//                     queued: 0,
//                     issued: 0,
//                     completed: 0,
//                 },
//             })
//         } else {
//             None
//         };
//
//             Some(CPUTrace {
//                 req: Request {
//                     request_type,
//                     memory_address,
//                     stat: RequestStat {
//                         arrival: 0,
//                         queued: 0,
//                         issued: 0,
//                         completed: 0,
//                     },
//             },
//                 cpu_cycles,
//                     wb_req: wb_req?,
//                 })
//         }
//     }
//
