#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World van Convoi");

    let kernel_version = 0.00.01;
    println!("Kernel versie: {}", kernel_version);

    let x = 20;
    let y = 22;
    println!("Small calculation: {} + {} = {}", x, y, x + y);

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}