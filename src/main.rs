#![no_main]
#![no_std]

mod core_reqs;
mod efi;
mod print;

use core::panic::PanicInfo;
use crate::efi::{EfiHandle, EfiSystemTable};
//use crate::core_reqs::{memset, memcpy, memcmp};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print!("{}", _info);
    loop{}
}

#[no_mangle]
fn efi_main(image_handle: EfiHandle, system_table: *mut EfiSystemTable){

    unsafe { efi::register_efi_system_table(system_table); }

    efi::clear_screen();


    let key: usize = efi::get_memory_map();
    efi::exit_boot(image_handle, key);

    //efi::clear_screen();


    //loop {}
}