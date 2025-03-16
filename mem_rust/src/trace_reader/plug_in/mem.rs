
use libloading::{Library, Symbol};
use simulation_interface::address_mapping::Mapper;
use simulation_interface::trace::{MemoryTrace, TraceReader};
use crate::trace_reader;
use crate::trace_reader::plug_in::{get_trace_reader,  TraceReaderPlugin};

pub unsafe fn get_memory_trace_reader(plugin_path: &str, trace_path: &str, mapper: &Mapper) -> Result<TraceReaderPlugin<MemoryTrace>, String> {
    get_trace_reader::<MemoryTrace>(plugin_path,
                                    trace_path,
                                    mapper,
                                    b"create_memory_trace_reader\0",
                                    b"destroy_memory_trace_reader\0")
}

#[derive(Debug)]
pub struct MemoryTraceReaderPlugin {
    plugin_reader: TraceReaderPlugin<MemoryTrace>,
    plugin_mapper: Mapper
}

impl MemoryTraceReaderPlugin {

    pub fn new(plugin_path: &str, trace_path: &str, plugin_mapper: Mapper) -> Result<Self, String> {
        println!("try create plugin");
        let plugin_reader = unsafe {
            get_memory_trace_reader(plugin_path, trace_path, &plugin_mapper)?
        };
        Ok(MemoryTraceReaderPlugin { plugin_reader, plugin_mapper })
    }

    pub fn next_trace(&self) -> Option<MemoryTrace> {
        unsafe {
            (*self.plugin_reader.reader).next_trace()
        }
    }

    pub fn destroy(self) {
        unsafe {
            destroy_trace_reader(self.plugin_reader.reader, self.plugin_reader.destroy_fn);
        }
    }
}

unsafe fn destroy_trace_reader(
    reader: *mut Box<dyn TraceReader<Trace = MemoryTrace>>,
    destroy_fn: Symbol<'static, unsafe extern "C" fn(*mut Box<dyn TraceReader<Trace = MemoryTrace>>)>
) {
    if !reader.is_null() {
        destroy_fn(reader);
    }
}


















