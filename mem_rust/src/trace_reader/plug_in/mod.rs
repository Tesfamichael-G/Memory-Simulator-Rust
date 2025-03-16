use std::ffi::CString;
use std::os::raw::c_char;
use libloading::{Library, Symbol};
use simulation_interface::address_mapping::Mapper;
use simulation_interface::trace::{CPUTrace, MemoryTrace, TraceReader};

pub mod mem;
pub mod cpu;
pub mod test_plugin_reader;

// Define common type aliases
pub(crate) type TraceReaderPtr<T> = *mut Box<dyn TraceReader<Trace = T>>;
pub(crate) type CreateReaderFn<T> = unsafe extern "C" fn(*const c_char, mapper: *const Mapper) -> TraceReaderPtr<T>;
pub(crate) type DestroyReaderFn<T> = unsafe extern "C" fn(TraceReaderPtr<T>);

#[derive(Debug)]
pub struct TraceReaderPlugin<T: 'static> {
    pub reader: TraceReaderPtr<T>, //*mut Box<dyn TraceReader<Trace = MemoryTrace>>,
    pub destroy_fn: Symbol<'static, DestroyReaderFn<T>>,
    _lib: Library,
}

// Generic function to load symbols
pub(crate) unsafe fn load_symbols<'a,T>(
    lib: &'a Library,
    create_symbol: &'a [u8],
    destroy_symbol: &'a [u8],
) -> Result<(Symbol<'a , CreateReaderFn<T>>, Symbol<'static, DestroyReaderFn<T>>), String> {
    let create_reader: Symbol<CreateReaderFn<T>> = lib
        .get(create_symbol)
        .map_err(|e| format!("Failed to load symbol: {}", e))?;

    let destroy_reader: Symbol<DestroyReaderFn<T>> = lib
        .get(destroy_symbol)
        .map_err(|e| format!("Failed to load symbol: {}", e))?;

    // Extend the lifetime of destroy_reader to 'static
    let destroy_reader = std::mem::transmute::<_, Symbol<'static, DestroyReaderFn<T>>>(destroy_reader);

    Ok((create_reader, destroy_reader))
}

pub unsafe fn get_trace_reader<T>(
    plugin_path: &str,
    trace_path: &str,
    mapper: &Mapper,
    create_symbol: &[u8],
    destroy_symbol: &[u8],
) -> Result<TraceReaderPlugin<T>, String> {
    print!("get lib");
    let lib = Library::new(plugin_path).map_err(|e| format!("Failed to load plugin: {}", e))?;

    print!("got lib");
    let (create_reader, destroy_reader) =
        load_symbols::<T>(&lib, create_symbol, destroy_symbol)?;
    print!("load symbols {:?}", create_reader);
    let file_path = CString::new(trace_path).map_err(|e| format!("Failed to convert file path to CString: {}", e))?;
    print!("path ok? {:?}", file_path);
    let reader = create_reader(file_path.as_ptr(), mapper);
    print!("reader created");
    if reader.is_null() {
        return Err("Failed to create trace reader".into());
    }
    print!("Ready: {:?}", reader);
    Ok(TraceReaderPlugin {
        reader,
        destroy_fn: destroy_reader,
        _lib: lib,
    })
}






























// pub struct MemoryTraceReaderPlugin {
//     pub reader: TraceReaderPtr<MemoryTrace>, //*mut Box<dyn TraceReader<Trace = MemoryTrace>>,
//     pub destroy_fn: Symbol<'static, DestroyReaderFn<MemoryTrace>>,
//     _lib: Library,
// }

// pub struct CPUTraceReaderPlugin {
//     pub reader: TraceReaderPtr<CPUTrace>,
//     pub destroy_fn: Symbol<'static, DestroyReaderFn<CPUTrace>>,
//     _lib: Library,
// }

