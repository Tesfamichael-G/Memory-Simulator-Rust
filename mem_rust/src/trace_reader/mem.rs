use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use memmap2::Mmap;

use simulation_interface::address_mapping::{AddressTranslation, Mapper};
use simulation_interface::trace::{MemoryTrace, TraceReader};
use simulation_interface::mem_request::{Request, MemoryAddress, MemoryCommand};


use crate::mem_request::request_type::RequestType;
use crate::mem_request::stat::RequestStat;



pub struct MemoryTraceReader{
    // reader: BufReader<File>,
    mapper: Mapper,
    // buffer: String,
    mmap: Mmap,
    position: usize,
}
impl MemoryTraceReader {
    pub fn new(file_path: &str, mapper: Mapper) -> io::Result<Self> {
        let file = File::open(file_path)?;
        // let reader = BufReader::new(file);
        let mmap = unsafe { Mmap::map(&file)? };
        Ok(Self {
            // reader, 
            mapper,
            // buffer: String::new(),
            mmap,
            position: 0,
        })
    }

   }

impl TraceReader for MemoryTraceReader {
    type Trace = MemoryTrace;
    // type Mapper = ();

    // fn next_trace<M: IAddressMapping>(&mut self, mapper: &M) -> Option<MemoryTrace> {
    // fn next_trace (&mut self, mapper: &dyn IAddressMapping) -> Option<MemoryTrace> {
    fn next_trace (&mut self) -> Option<MemoryTrace> {
        let data = &self.mmap[self.position..];
        let newline_pos = data.iter().position(|&b| b == b'\n')?;

        let line = &data[..newline_pos];
        self.position += newline_pos + 1;  // Move position after the newline

        // Process the line as bytes without allocating a String
        let line_str = std::str::from_utf8(line).ok()?.trim();

        let (address_str, request_str) = line_str.split_once(' ')?;

        let request_type = match request_str {
            "R" => RequestType::Read,
            "W" => RequestType::Write,
            _ => return None,
        };

        let address_value = usize::from_str_radix(&address_str[2..], 16).ok()?;
        let memory_address = self.mapper.translate(address_value);

        Some(MemoryTrace {
            req: Request {
                request_type,
                memory_address,
                stat: RequestStat {
                    created: 0,
                    queued: 0,
                    issued: 0,
                },
            }
        })
         }



}

//=======================
// self.buffer.clear();
//
// // let mut line = String::new();
//
// if self.reader.read_line(&mut self.buffer).ok()? == 0 {
// return None;
// }
// let line = self.buffer.trim();
//
// // let parts: Vec<&str> = line.trim().split_whitespace().collect();
// // if parts.len() != 2 {
// //     return None;
// // }
//
// // let request_type = match parts[1] {
// //     "R" => RequestType::Read,
// //     "W" => RequestType::Write,
// //     _ => return None,
// // };
//
// let mut parts = line.split_whitespace();
// let address_str = parts.next()?;
// let request_str = parts.next()?;
//
// // Ensure there are exactly two parts
// if parts.next().is_some() {
// return None;
// }
//
// let request_type = match request_str {
// "R" => RequestType::Read,
// "W" => RequestType::Write,
// _ => return None,
// };
// // let address_value =  usize ::from_str_radix(&parts[0][2..], 16).ok()?;
// let address_value = usize::from_str_radix(&address_str[2..], 16).ok()?;
// let memory_address = self.mapper.translate(address_value);
//
// Some(MemoryTrace {
// req: Request {
// request_type,
// memory_address,
// stat: RequestStat {
// created: 0,
// queued: 0,
// issued: 0,
// },
// }
// })
