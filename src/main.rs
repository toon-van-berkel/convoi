#![no_std]
#![no_main]

#[macro_use]
mod vga_buffer;

use core::panic::PanicInfo;

// #[unsafe(no_mangle)]
// pub extern "C" fn _start() -> ! {
//     let message = b"CONVOI KERNEL STARTED";

//     let vga_buffer = 0xb8000 as *mut u8;

//     for (i, &byte) in message.iter().enumerate() {
//         unsafe {
//             *vga_buffer.offset(i as isize * 2) = byte;
//             *vga_buffer.offset(i as isize * 2 + 1) = 0x0f;
//         }
//     }

//     loop {}
// }

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World van Convoi");

    let kernel_version = "0.0.1";
    println!("Kernel versie: {}", kernel_version);

    let x = 2;
    let y = 3;
    println!("Small calculation: {} + {} = {}", x, y, x + y);

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

