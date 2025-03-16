pub mod address;
pub mod request_type;
pub mod stat;
pub mod mem_command;
mod request_status;
mod callback;

pub use address::MemoryAddress;
pub use request_type::RequestType;
pub use stat::RequestStat;
pub use mem_command::MemoryCommand;
pub use request_status::RequestStatus;

pub  use callback::Callback;
pub use callback::ReadCallback;

#[derive(Debug)]
pub struct Request {
    pub request_type: RequestType,
    pub memory_address: MemoryAddress,
    pub stat:RequestStat,
    // pub cmd: MemoryCommand
}
