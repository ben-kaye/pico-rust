#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod boot {
    use core::arch::global_asm;

    global_asm!(
        ".section .text._start"
    );
}

mod picore;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    picore::event_loop();
    loop {}
}

#[panic_handler]
fn panic (_info: &PanicInfo) -> ! {
    loop {}
}