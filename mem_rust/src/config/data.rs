
use serde::{Deserialize, Serialize};
// use crate::scheduler::SchedulingPolicy;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config{
    pub trace_type: String,
    pub trace_path: String,
    pub mem_spec_path: String,
    pub cpu_spec_path: String,
    pub address_mapping_string: String,
    pub use_trace_plugin: bool,
    // pub trace_plugin_path: Option<String>,
    pub trace_plugin_path: String,
    pub queueing_policy: String,
    pub scheduling_policy: String
}
