use simulation_interface::address_mapping::Mapper;
use simulation_interface::memory_model::ddr::DDR4;
use simulation_interface::spec_parser::SpecParser;
use crate::config::load_config;
use crate::trace_reader::{MemoryTraceReader, MemoryTraceReaderPlugin};
// use crate::trace_reader::{CPUTraceReaderPlugin, MemoryTraceReader, MemoryTraceReaderPlugin};

struct MemoryPlugIn{
    pub cycle: u32,
    pub reader: MemoryTraceReaderPlugin,
}

impl MemoryPlugIn{
    pub fn new(trace_reader: MemoryTraceReaderPlugin) -> Self{
        println!("Creating MemoryPlugIn");
        Self {
            cycle: 0,
            reader: trace_reader
        }
    }
}
//
// struct CPUPlugIn{
//     pub cycle: u32,
//     pub reader: CPUTraceReaderPlugin,
// }
//
// impl CPUPlugIn {
//     pub fn new(trace_reader: CPUTraceReaderPlugin) -> Self{
//     Self {
//         cycle: 0,
//         reader: trace_reader
//         }
//     }
// }
//

pub fn test_mem_reader(){

    println!("sim started.");
    let config_path = r"I:\RustroverProjects\mem_rust\config\simulator.json";
    let config = load_config(config_path)
        .expect("Error reading config file");

    // println!("{:#?}", config);

    let mapping_str: &str = &config.address_mapping_string;

    let (org, model, mapping_bits) = DDR4::parse(&config.mem_spec_path).expect("error parsing json string for mem spec");

    let mapper = Mapper::new(mapping_str, mapping_bits);

    let _reader = MemoryTraceReaderPlugin
    ::new(&config.trace_plugin_path, &config.trace_path, mapper)
        .expect("Failed to create memory trace reader. Make sure the trace_file_path and the location of plugin specified in path is correct.");

    print!("{:?}", _reader);
    process_mem_traces_plugin(_reader);


}

fn process_mem_traces_plugin(mut reader: MemoryTraceReaderPlugin){
    loop {
        match reader.next_trace() {
            Some(trace) => {
                println!("{:?}", trace);
                // Process the trace...
            }
            None => break,
        }
    }

    reader.destroy();
}


// pub fn test_cpu_reader(){
//
//     println!("sim started.");
//     let config_path = r"I:\RustroverProjects\mem_rust\config\simulator.json";
//     let config = load_config(config_path)
//         .expect("Error reading config file");
//
//     println!("{:#?}", config);
//
//     let mapping_str: &str = &config.address_mapping_string;
//
//     let (org, model, mapping_bits) = DDR4::parse(&config.mem_spec_path).expect("error parsing json string for mem spec");
//
//     let mapper = Mapper::new(mapping_str, mapping_bits);
//
//
//     let _reader = CPUTraceReaderPlugin
//     ::new(&config.trace_plugin_path, &config.trace_path, mapper)
//         .expect("Failed to create memory trace reader");
//
//     process_cpu_traces_plugin(_reader);
//
// }
// fn process_cpu_traces_plugin(mut reader: CPUTraceReaderPlugin){
//     loop {
//         match reader.next_trace() {
//             Some(trace) => {
//                 println!("{:?}", trace);
//                 // Process the trace...
//             }
//             None => break,
//         }
//     }
//
//     reader.destroy();
// }

