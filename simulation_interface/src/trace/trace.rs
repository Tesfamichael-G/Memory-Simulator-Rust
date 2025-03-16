use crate::mem_request::Request;

#[derive(Debug)]
pub struct CPUTrace {
    pub req: Request,
    pub cpu_cycles: u32,
    pub wb_req: Option<Request>
}

#[derive(Debug)]
pub struct MemoryTrace {
    pub req: Request,
}

pub enum TraceType {
    Memory,
    CPU,
}

