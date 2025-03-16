
use simulation_interface::trace::{MemoryTrace, TraceReader};
use simulation_interface::address_mapping::{AddressTranslation, Mapper};
use simulation_interface::spec_parser::SpecParser;
use simulation_interface::memory_model::ddr::{BankConstraints, BankGroupConstraints, ddr4, DDR4, DDRConstraints, Org, RankConstraints, RankSiblingConstraints};


use crate::config::{Config, load_config};
// use crate::spec_parser::ddr::ddr4::{parse_ddr4_spec};

use crate::trace_reader;
use crate::trace_reader::{MemoryTraceReader};
// use crate::trace_reader::get_cpu_trace_reader;
// use crate::trace_reader::get_memory_trace_reader;


use libloading::{Library, Symbol};
use std::ffi::{c_ushort, CString};
use std::io::{stdout, Write};
use std::os::raw::c_char;
use simulation_interface::mem_request::{Callback};
use simulation_interface::memory_model::DRAM;
use simulation_interface::policy::{BusDirectionStrategy, QueueManager, Scheduler};

use crate::bus::{Bus, ThresholdStrategy};
use crate::memory::DDR;

use crate::queue::{PerBankGroupQueue, PerBankQueue, PerDRAMQueue, PerRankQueue};// QueueingPolicy};
use crate::scheduler::FRFCFS;
use crate::scheduler::PerBankFRFCFS;
use crate::simulation::system::MemorySimulator;

// use crate::trace_reader::plug_in::cpu::CPUTraceReaderPlugin;
// use crate::trace_reader::plug_in::mem::MemoryTraceReaderPlugin;
// use crate::trace_reader::plug_in::TraceReaderPlugin;


pub fn run() {

    println!("sim started.");
    let config_path = "./config/simulator.json";
    // let config_path = r"I:\RustroverProjects\mem_rust\config\simulator.json";

    let config = load_config(config_path)
        .expect("Error reading config file");

    // println!("{:#?}", config);

    let mapping_str: &str = &config.address_mapping_string;

    let (org, model, mapping_bits) = DDR4::parse(&config.mem_spec_path).expect("error parsing json string for mem spec");

    let num_channels= org.channel.clone() as usize;
    let num_ranks= org.rank.clone() as usize;
    let num_bank_groups = org.bank_group.clone() as usize;
    let num_banks = org.bank.clone() as usize;

    let mapper = Mapper::new(mapping_str, mapping_bits);

    // println!("{:?}", mapper);

    let timing_constraints=
        model.get_constraints().expect("Error getting timing constraints");

    let reader = MemoryTraceReader
    ::new(&config.trace_path, mapper)
        .expect("error initializing a memory trace reader");

    // let queue_manager = PerRankQueue::new(64,64).unwrap();
    // let scheduler = FRFCFS;

    let queue_manager = PerBankQueue::new(64,64, num_bank_groups, num_banks).unwrap();
    let scheduler = PerBankFRFCFS::new( num_bank_groups, num_banks).expect("Failed to create per bank scheduler");

    let bus_direction_strategy = ThresholdStrategy::new(8, 48);

    let dram: DDR = DDR::new(num_ranks, num_bank_groups,  num_banks, timing_constraints).expect("Failed to create DRAM");

    let mut mem_sys =  MemorySimulator::new(
                                           reader,
                                           scheduler,
                                           queue_manager,
                                           bus_direction_strategy,
                                           dram,

                                           num_channels,
                                           // num_ranks,
                                           // num_bank_groups,
                                           // num_banks,
                                           // timing_constraints,

                                           )
                                        .expect("error initializing a memory system");

    // print!("\x1B[2J\x1B[1;1H");
    // stdout().flush().unwrap();

    println!("simulation started.");

    while mem_sys.tick() {

    }
    use std::io::{self, BufRead};
    println!("Press Enter to exit...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    println!("simulation ended.");
}



























































//
// unsafe fn run_trace_reader(reader: *mut Box<dyn TraceReader<Trace = MemoryTrace>>, mapper: &Mapper) {
//     while let Some(trace) = (*reader).next_trace(mapper) {
//         println!("{:?}", trace);
//     }
// }
//
// unsafe fn destroy_trace_reader(
//     reader: *mut Box<dyn TraceReader<Trace = MemoryTrace>>,
//     destroy_fn: Symbol<'static, unsafe extern "C" fn(*mut Box<dyn TraceReader<Trace = MemoryTrace>>)>
// ) {
//     if !reader.is_null() {
//         destroy_fn(reader);
//     }
// }
//
// unsafe fn run_cpu_trace_reader(config: &Config, mapper: &Mapper){
//
//     // run_x(&config.trace_plugin_path, &config.trace_path, &mapper);
//     // run_y(&config.trace_plugin_path, &config.trace_path, &mapper);
//     // unsafe {}
//     match trace_reader::get_cpu_trace_reader(&config.trace_plugin_path, &config.trace_path) {
//         Some(plugin_reader) => {
//             run_trace_reader(plugin_reader.reader, &mapper);
//             destroy_trace_reader(plugin_reader.reader, plugin_reader.destroy_fn);
//         }
//         None => eprintln!("Failed to create trace reader"),
//     }
// }
// unsafe  fn run_x(plugin_path: &str, trace_path: &str, mapper: &Mapper) {
//
//         let lib = Library::new(plugin_path).expect("Failed to load plugin");
//
//         unsafe {
//             let create_reader: Symbol<unsafe extern "C" fn(*const c_char) -> *mut Box<dyn TraceReader<Trace=MemoryTrace>>> =
//                 lib.get(b"create_x_trace_reader\0").expect("Failed to load symbol");
//             let destroy_reader: Symbol<unsafe extern "C" fn(*mut Box<dyn TraceReader<Trace=MemoryTrace>>)> =
//                 lib.get(b"destroy_x_trace_reader\0").expect("Failed to load symbol");
//
//             let file_path = CString::new(trace_path).expect("Failed to convert file path to CString");
//             let reader = create_reader(file_path.as_ptr());
//             if reader.is_null() {
//                 eprintln!("Failed to create trace reader");
//                 return;
//             }
//
//             // Use the reader
//             while let Some(trace) = (*reader).next_trace(mapper) {
//                 println!("{:?}", trace);
//             }
//
//             destroy_reader(reader);
//         }
// }
// unsafe  fn run_y(plugin_path: &str, trace_path: &str, mapper: &Mapper) {
//
//     let lib = Library::new(plugin_path).expect("Failed to load plugin");
//
//     unsafe {
//         let create_reader: Symbol<unsafe extern "C" fn(*const c_char) -> *mut Box<dyn TraceReader<Trace=MemoryTrace>>> =
//             lib.get(b"create_y_trace_reader\0").expect("Failed to load symbol");
//         let destroy_reader: Symbol<unsafe extern "C" fn(*mut Box<dyn TraceReader<Trace=MemoryTrace>>)> =
//             lib.get(b"destroy_y_trace_reader\0").expect("Failed to load symbol");
//
//         let file_path = CString::new(trace_path).expect("Failed to convert file path to CString");
//         let reader = create_reader(file_path.as_ptr());
//         if reader.is_null() {
//             eprintln!("Failed to create trace reader");
//             return;
//         }
//
//         // Use the reader
//         while let Some(trace) = (*reader).next_trace(mapper) {
//             println!("{:?}", trace);
//         }
//
//         destroy_reader(reader);
//     }
// }
// unsafe fn run_rusted_xy(plugin_path: String, trace_path: String, mapper: &Mapper){
//     let lib = Library::new(plugin_path).expect("Failed to load plugin");
//
//     unsafe {
//         let create_reader: Symbol<unsafe extern "C" fn(&str) -> *mut dyn TraceReader<Trace = MemoryTrace>> =
//             lib.get(b"create_memory_trace_reader\0")
//                 .expect("Failed to load symbol");
//
//         let destroy_reader: Symbol<unsafe extern "C" fn(*mut dyn TraceReader<Trace = MemoryTrace>)> =
//             lib.get(b"destroy_memory_trace_reader\0")
//                 .expect("Failed to load symbol");
//
//         let reader = create_reader(&trace_path);
//         if reader.is_null() {
//             eprintln!("Failed to create trace reader");
//             return;
//         }
//
//         // Use the reader
//         while let Some(trace) = (*reader).next_trace(mapper) {
//             println!("{:?}", trace);
//         }
//
//         destroy_reader(reader);
//     }
// }
// unsafe fn run_x_reader(plugin_path: String, trace_path: String, mapper: &Mapper){
//     let lib = Library::new(plugin_path).expect("Failed to load library");
//
//     unsafe {
//         let func: Symbol<unsafe extern "C" fn(*const c_char) -> Option<*mut dyn TraceReader<Trace = MemoryTrace>>> =
//             lib.get(b"create_memory_trace_reader").expect("Failed to load function");
//
//         let c_trace_file = CString::new(trace_path).expect("CString::new failed");
//         let reader = func(c_trace_file.as_ptr()).expect("Failed to get trace reader");
//
//         // Use the reader...
//         // let mut mapper = IAddressMapping::new();
//         while let Some(trace) = (*reader).next_trace(mapper) {
//             println!("{:?}", trace);
//         }
//     }
// }
// unsafe fn run_y_reader(plugin_path: String, trace_path: String, mapper: &Mapper){
//     let lib = Library::new(plugin_path).expect("Failed to load library");
//
//     unsafe {
//         let func: Symbol<unsafe extern "C" fn(*const c_char) -> Option<*mut dyn TraceReader<Trace = MemoryTrace>>> =
//             lib.get(b"create_memory_trace_reader").expect("Failed to load function");
//
//         let c_trace_file = CString::new(trace_path).expect("CString::new failed");
//         let reader = func(c_trace_file.as_ptr()).expect("Failed to get trace reader");
//
//         // Use the reader...
//         // let mut mapper = IAddressMapping::new();
//         while let Some(trace) = (*reader).next_trace(mapper) {
//             println!("{:?}", trace);
//         }
//     }
// }















// let queue_policy: String = config.queueing_policy.to_lowercase();
// let scheduling_policy = config.scheduling_policy.to_lowercase();

// let queue_manager: QueueingPolicy =
//
//     match queue_policy.as_str() {
//     "perdram" => QueueingPolicy::PerRank(PerRankQueue::new(64,64).unwrap()),
//     "perrank" => QueueingPolicy::PerRank(PerRankQueue::new(64,64).unwrap()),
//     "perbankgroup" => QueueingPolicy::PerRank(PerRankQueue::new(64,64).unwrap()),
//     "perbank" => QueueingPolicy::PerRank(PerRankQueue::new(64,64).unwrap()),
//      _ => QueueingPolicy::PerRank(PerRankQueue::new(64,64).unwrap()),
// };

// let scheduler: SchedulingPolicy = match scheduling_policy.as_str() {
//     "fcfs" => SchedulingPolicy::FRFCFS(FRFCFS),
//     "frfcfs" =>  SchedulingPolicy::FRFCFS(FRFCFS),
//     "custom" => SchedulingPolicy::FRFCFS(FRFCFS),
//
//     _ => SchedulingPolicy::FRFCFS(FRFCFS),
// };








// fn process_traces<R: TraceReader>(mut reader: R, mapper: &dyn AddressMapper) {
//     while let Some(trace) = reader.next_trace(mapper) {
//         println!("{:?}", trace);
//     }
// }

//============================================================
// let lib = Library::new(config.trace_plugin_path.unwrap()).expect("Failed to load plugin");
//         // unsafe {
//         //     let create_reader: Symbol<unsafe fn(&str) -> Box<dyn TraceReader<Trace = MemoryTrace>>> =
//         //         lib.get(b"create_memory_trace_reader").expect("Failed to load symbol");
//         //     let memory_reader = create_reader(&config.trace_path);
//         //     process_traces(*memory_reader, &mapping_data);
//         // }