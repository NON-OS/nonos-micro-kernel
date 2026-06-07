#![no_std]
#![no_main]

use nonos_libc::mk_exit;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    mk_exit(0)
}
