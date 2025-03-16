// use std::error::Error;
// use std::ffi::CString;
// use std::os::raw::c_char;
// use libloading::{Library, Symbol};
// use simulation_interface::address_mapping::Mapper;
// use simulation_interface::trace::{CPUTrace, MemoryTrace, TraceReader};
// use crate::trace_reader::plug_in::{TraceReaderPlugin, TraceReaderPtr, DestroyReaderFn, load_symbols, get_trace_reader};
//
// pub unsafe fn get_cpu_trace_reader(plugin_path: &str, trace_path: &str, mapper: Mapper) -> Result<TraceReaderPlugin<CPUTrace>, String> {
//     get_trace_reader::<CPUTrace>(plugin_path, trace_path, mapper,b"create_cpu_trace_reader\0", b"destroy_cpu_trace_reader\0")
// }
//
//
// pub struct CPUTraceReaderPlugin {
//     plugin_reader: TraceReaderPlugin<CPUTrace>,
//     plugin_mapper: Mapper
// }
//
// impl CPUTraceReaderPlugin {
//
//     pub fn new(plugin_path: &str, trace_path: &str, plugin_mapper: Mapper) -> Result<Self, String> {
//         let plugin_reader = unsafe { get_cpu_trace_reader(plugin_path, trace_path, plugin_mapper)? };
//         Ok(CPUTraceReaderPlugin { plugin_reader, plugin_mapper })
//     }
//
//     pub fn next_trace(&self) -> Option<CPUTrace> {
//         unsafe {
//             (*self.plugin_reader.reader).next_trace()
//         }
//     }
//
//     pub fn destroy(self) {
//         unsafe {
//             destroy_trace_reader(self.plugin_reader.reader, self.plugin_reader.destroy_fn);
//         }
//     }
// }
//
// unsafe fn destroy_trace_reader(
//     reader: *mut Box<dyn TraceReader<Trace = CPUTrace>>,
//     destroy_fn: Symbol<'static, unsafe extern "C" fn(*mut Box<dyn TraceReader<Trace = CPUTrace>>)>
// ) {
//     if !reader.is_null() {
//         destroy_fn(reader);
//     }
// }
