use crate::mem_request::MemoryCommand;

pub struct RequestStatus {
    pub is_issuable: bool,
    pub is_hot: bool,
    pub command: Option<MemoryCommand>,
}
