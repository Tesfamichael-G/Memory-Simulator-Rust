#[derive(Debug, Clone, Copy)]
pub enum MemoryCommand {

    Pre_ab,
    Pre_b,

    Activate,

    Read,
    Write,
    ReadAP,
    WriteAP,

    Refresh,
}
