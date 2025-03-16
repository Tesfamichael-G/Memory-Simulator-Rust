use std::io;
use crate::stat::Average;
use crate::stat::Count;

pub struct Stat {
    pub basic: Count,

    pub read_latency: Average,
    pub write_latency: Average,

    pub read_queue_latency: Average,
    pub write_queue_latency: Average,
}

impl Stat{
    pub fn new() -> io::Result<Self>{
        Ok( Self{
            basic: Count {
                pre: 0,
                act: 0,
                act_for_read: 0,
                act_for_write: 0,
                read: 0,
                write: 0,
                read_hits: 0,
                write_hits: 0,
            },
            read_latency: Average::new()?,
            write_latency: Average::new()?,
            read_queue_latency: Average::new()?,
            write_queue_latency: Average::new()?,
        })
    }
    pub fn read_hit_rate(&mut self) -> Option<f64>{
        // public float READ_HitRate { get; set; }
        // public float READ_MissRate { get; set; }
        //
      Some(0.0)
    }
    pub fn read_miss_rate(&mut self) -> Option<f64>{
        Some(0.0)
    }
    pub fn write_hit_rate(&mut self) -> Option<f64>{
        // public float WRITE_HitRate { get; set; }
        // public float WRITE_MissRate { get; set; }
        Some(0.0)
    }
    pub fn write_miss_rate(&mut self) -> Option<f64>{
        // public float WRITE_HitRate { get; set; }
        // public float WRITE_MissRate { get; set; }
        Some(0.0)
    }
}