use crate::print;
use core::sync::atomic::{AtomicPtr, Ordering};

static EFI_SYSTEM_TABLE: AtomicPtr<EfiSystemTable> = AtomicPtr::new(core::ptr::null_mut());

pub unsafe fn register_efi_system_table(system_table: *mut EfiSystemTable) {
    EFI_SYSTEM_TABLE.compare_and_swap(core::ptr::null_mut(), system_table, Ordering::SeqCst);
}

// Gets the memory map that we need before we exit boot services
pub fn get_memory_map() -> usize {
    let st = EFI_SYSTEM_TABLE.load(Ordering::SeqCst);
    assert!(!st.is_null(), "System table is null");

    // Array of Memory Descriptors and 64 is an arbitary number I picked, less and it throws buffer to small error
    let mut memory_map = [EfiMemoryDescriptor::default(); 64];

    // Delcare variables that will store the output of the get_memory_map function;
    let mut size: usize = core::mem::size_of_val(&memory_map);
    let mut key: usize = 0;
    let mut mdesc_size: usize = 0;
    let mut mdesc_version: u32 = 0;

    unsafe {
        let status = ((*(*st).boot_services).get_memory_map)(
            &mut size,
            memory_map.as_mut_ptr(),
            &mut key,
            &mut mdesc_size,
            &mut mdesc_version,
        );

        assert!(
            status == EfiStatus::Success,
            "Failed to get memory_map: {:?}",
            status
        );

        for entry in memory_map.iter(){
            let typ: EfiMemoryType = entry.typ.into();
            print!("{:?}, {}, {}, {}\n", typ, entry.phyiscal_start, entry.virtual_start, entry.number_of_pages);
            //print!("{:?}\n", &entry);
        }

    }

    //print!("\n{:?}\n", memory_map);

    // print!(
    //     "\nSize: {}bytes, MapKey is: {}, Descriptor Size: {}bytes, Descriptor Version: {}",
    //     size, key, mdesc_size, mdesc_version
    // );

    key
}

pub fn exit_boot(handle: EfiHandle, key: usize) {
    let st = EFI_SYSTEM_TABLE.load(Ordering::SeqCst);

    assert!(!st.is_null(), "System table is null");

    print!("\nKey: {:?}, Handle: {:x?}", key, handle);
    unsafe {
        let status = ((*(*st).boot_services).exit_boot_services)(handle, key);

        print!("\nExit Boot Services Status: {:?}", status);
    }
}

// Prints out a USC2 string in array buffers
pub fn output_string(string: &str) -> EfiStatus {
    // Assign the pointer to the system table to st and check it is valid
    let st = EFI_SYSTEM_TABLE.load(Ordering::SeqCst);
    assert!(!st.is_null(), "System table is null");

    let mut str_buf = [0u16; 32];
    let mut i = 0;

    for chr in string.encode_utf16() {
        if i == str_buf.len() {
            unsafe { (*(*st).console_out).output_string(str_buf.as_ptr()) };
            str_buf = [0u16; 32];
            i = 0;
        }
        str_buf[i] = chr;
        i += 1;
    }
    unsafe { (*(*st).console_out).output_string(str_buf.as_ptr()) }
}

// Function to clear the uefi screen
pub fn clear_screen() {
    let st = EFI_SYSTEM_TABLE.load(Ordering::SeqCst);

    assert!(!st.is_null(), "System table is null");

    unsafe {
        let out = (*st).console_out;
        ((*out).reset)(out, true);
    };
}

#[repr(C)]
#[derive(Debug)]
pub struct EfiHandle(usize);

struct EfiInputKey {
    _scan_code: u16,
    _unicode_char: u16,
}

#[repr(C)]
struct EfiTableHeader {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    _reserved: u32,
}

#[repr(C)]
struct EfiSimpleTextInputProtocol {
    reset: unsafe fn(this: &EfiSimpleTextInputProtocol, extended_verification: bool) -> EfiStatus,

    read_keystroke:
        unsafe fn(this: &EfiSimpleTextInputProtocol, key: &mut EfiInputKey) -> EfiStatus,

    wait_for_key: usize,
}

#[repr(C)]
struct EfiSimpleTextOutputProtocol {
    reset: unsafe fn(
        this: *const EfiSimpleTextOutputProtocol,
        extended_verification: bool,
    ) -> EfiStatus,
    output_string:
        unsafe fn(this: *const EfiSimpleTextOutputProtocol, string: *const u16) -> EfiStatus,
    test_string:
        unsafe fn(this: *const EfiSimpleTextOutputProtocol, string: *const u16) -> EfiStatus,
    _query_mode: usize,
    _set_attribute: usize,
    _clear_screen: usize,
    _set_cursor_position: usize,
    _enable_cursor: usize,
    _mode: usize,
}

impl EfiSimpleTextOutputProtocol {
    fn output_string(&self, str_ptr: *const u16) -> EfiStatus {
        unsafe { (self.output_string)(self, str_ptr) }
    }
}

#[repr(C)]
pub struct EfiSimpleTextErrorProtocol {}


#[derive(Clone, Copy, Default, Debug)]
#[repr(C)]
pub struct EfiMemoryDescriptor {
    typ: u32,
    phyiscal_start: u64,
    virtual_start: u64,
    number_of_pages: u64,
    attribute: u64,
}

#[repr(C)]
pub struct EfiBootServices {
    header: EfiTableHeader,
    // Task Priority Services
    raise_tpl: usize,
    restore_tpl: usize,
    // Memory Services
    allocate_pages: usize,
    free_pages: usize,
    get_memory_map: unsafe fn(
        memory_map_size: &mut usize,
        memory_map: *mut EfiMemoryDescriptor,
        map_key: &mut usize,
        descriptor_size: &mut usize,
        descriptor_version: &mut u32,
    ) -> EfiStatus,
    allocate_pool: usize,
    free_pool: usize,
    // Event & timer services
    create_event: usize,
    set_timer: usize,
    wait_for_event: usize,
    signal_event: usize,
    close_event: usize,
    check_event: usize,
    // Protocol Handler Services
    install_protocol_interface: usize,
    reinstall_protocol_interface: usize,
    uninstall_protocol_interface: usize,
    handle_protocol: usize,
    _reserved: usize,
    register_protocol_notify: usize,
    locate_handle: usize,
    locate_deivce_path: usize,
    install_configuration_table: usize,
    // Image Services
    image_load: usize,
    image_start: usize,
    exit: usize,
    image_unload: usize,
    test: usize,
    exit_boot_services: unsafe fn(image_handle: EfiHandle, map_key: usize) -> EfiStatus,
}

#[repr(C)]
pub struct EfiSystemTable {
    header: EfiTableHeader,
    firmware_vendor: *const u16,
    firmware_revision: u32,
    console_in_handle: EfiHandle,
    console_in: *const EfiSimpleTextInputProtocol,
    console_out_handle: EfiHandle,
    console_out: *const EfiSimpleTextOutputProtocol,
    console_err_handle: EfiHandle,
    console_err: *const EfiSimpleTextErrorProtocol,
    runtime_services: usize,
    boot_services: *const EfiBootServices,
}


// MEMORY TPYE STUFF

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum EfiMemoryType {
    ReservedMemoryType,  
    LoaderCode,  
    LoaderData,  
    BootServicesCode,  
    BootServicesData,  
    RuntimeServicesCode,  
    RuntimeServicesData,  
    ConventionalMemory,  
    UnusableMemory,  
    ACPIReclaimMemory,  
    ACPIMemoryNVS,  
    MemoryMappedIO,  
    MemoryMappedIOPortSpace,  
    PalCode,  
    PersistentMemory,  
    MaxMemoryType,
    Invalid
}

impl From<u32> for EfiMemoryType{
    fn from(val: u32) -> Self {
        match val {
        0 => EfiMemoryType::ReservedMemoryType,  
        1 => EfiMemoryType::LoaderCode,  
        2 => EfiMemoryType::LoaderData,  
        3 => EfiMemoryType::BootServicesCode,  
        4 => EfiMemoryType::BootServicesData,  
        5 => EfiMemoryType::RuntimeServicesCode,  
        6 => EfiMemoryType::RuntimeServicesData,  
        7 => EfiMemoryType::ConventionalMemory,  
        8 => EfiMemoryType::UnusableMemory,  
        9 => EfiMemoryType::ACPIReclaimMemory,  
        10 => EfiMemoryType::ACPIMemoryNVS,  
        11 => EfiMemoryType::MemoryMappedIO,  
        12 => EfiMemoryType::MemoryMappedIOPortSpace,  
        13 => EfiMemoryType::PalCode,  
        14 => EfiMemoryType::PersistentMemory,  
        15 => EfiMemoryType::MaxMemoryType,
        _ => EfiMemoryType::Invalid
    }
}

}

// STATUS CODE STUFF---------------
const ERROR_BIT: usize = 1 << (core::mem::size_of::<usize>() * 8 - 1);

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(usize)]
pub enum EfiStatus {
    Success = 0,
    LoadError = ERROR_BIT | 1,
    InvalidParameter = ERROR_BIT | 2,
    Unsupported = ERROR_BIT | 3,
    BadBufferSize = ERROR_BIT | 4,
    BufferTooSmall = ERROR_BIT | 5,
    NotReady = ERROR_BIT | 6,
    DeviceError = ERROR_BIT | 7,
    WriteProtected = ERROR_BIT | 8,
    OutOfResources = ERROR_BIT | 9,
    VolumeCorrupted = ERROR_BIT | 10,
    VolumeFull = ERROR_BIT | 11,
    NoMedia = ERROR_BIT | 12,
    MediaChanged = ERROR_BIT | 13,
    NotFound = ERROR_BIT | 14,
    AccessDenied = ERROR_BIT | 15,
    NoResponse = ERROR_BIT | 16,
    NoMapping = ERROR_BIT | 17,
    Timeout = ERROR_BIT | 18,
    NotStarted = ERROR_BIT | 19,
    AlreadyStarted = ERROR_BIT | 20,
    Aborted = ERROR_BIT | 21,
    IcmpError = ERROR_BIT | 22,
    TftpError = ERROR_BIT | 23,
    ProtocolError = ERROR_BIT | 24,
    IncompatibleVersion = ERROR_BIT | 25,
    SecurityViolation = ERROR_BIT | 26,
    CrcError = ERROR_BIT | 27,
    EndOfMedia = ERROR_BIT | 28,
    EndOfFile = ERROR_BIT | 31,
    InvalidLanguage = ERROR_BIT | 32,
    CompromisedData = ERROR_BIT | 33,
    IpAddressConflict = ERROR_BIT | 34,
    HttpError = ERROR_BIT | 35,
    WarnUnknownGlyph = 1,
    WarnDeleteFailure = 2,
    WarnWriteFailure = 3,
    WarnBufferTooSmall = 4,
    WarnStaleData = 5,
    WarnFileSystem = 6,
    WarnResetRequired = 7,
}
