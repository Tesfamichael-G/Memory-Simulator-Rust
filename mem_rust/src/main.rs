use std::time::{Duration, Instant};
use simulation_interface::memory_model;
use simulation_interface::trace;
use simulation_interface::address_mapping;
use simulation_interface::mem_request;


mod config;
mod simulation;
mod trace_reader;

mod memory;
mod scheduler;
mod controller;
mod queue;

mod refresher;
mod bus;

fn measure_runtime(f: impl FnOnce()) -> Duration{
    let start = Instant::now();
    f();
    let duration = Instant::now().duration_since(start);
    duration
}

fn main() {


    println!("Starting ...");


    let start = Instant::now();
    simulation::runner::run();
    let duration = Instant::now().duration_since(start);
    println!("Elapsed time: {:?}", duration);



}
