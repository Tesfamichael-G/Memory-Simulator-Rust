// pub mod cpu;
pub mod mem;
pub mod cpu;

pub mod plug_in;
//
pub use crate::trace_reader::plug_in::TraceReaderPlugin;
//
pub use crate::trace_reader::mem::MemoryTraceReader;
pub use crate::trace_reader::cpu::CPUTraceReader;

pub use crate::trace_reader::plug_in::mem::MemoryTraceReaderPlugin;
pub use crate::trace_reader::plug_in::mem::get_memory_trace_reader;



// pub use crate::trace_reader::plug_in::cpu::CPUTraceReaderPlugin;
// pub use crate::trace_reader::plug_in::cpu::get_cpu_trace_reader;

