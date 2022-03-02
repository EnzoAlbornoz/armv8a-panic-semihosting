// Declare Crate Attributes
#![no_std]
// Import Dependencies
use armv8a_semihosting::{
    debug::{self, EXIT_FAILURE},
    hio,
};
use core::arch::asm;
use core::fmt::Write;
use core::panic::PanicInfo;
// Declare Helpers
#[inline(always)]
fn int_disable() {
    unsafe {
        asm!("msr daifset, #0b0011");
    }
}
// Define as Panic Handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    int_disable();

    #[cfg(feature = "info")]
    if let Ok(mut hstdout) = hio::hstdout() {
        writeln!(hstdout, "{}", info).ok();
    }

    // Exit the QEMU process
    debug::exit(EXIT_FAILURE);
    // Make unrecoverable
    loop {}
}
