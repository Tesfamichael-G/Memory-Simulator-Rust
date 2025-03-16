
mod trace;

pub use trace::MemoryTrace;
pub use trace::CPUTrace;
pub use trace::TraceType;

use std::fmt::Debug;

pub trait TraceReader {
    type Trace : Debug;

     fn next_trace(&mut self) -> Option<Self::Trace>;
    }