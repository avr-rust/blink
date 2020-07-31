#![feature(llvm_asm)]

#![no_std]
#![no_main]

use ruduino::cores::atmega328 as avr_core;
use ruduino::Register;

use avr_core::{DDRB, PORTB};

#[no_mangle]
pub extern fn main() {
    // Set all PORTB pins up as outputs
    DDRB::set_mask_raw(0xFFu8);

    loop {
        // Set all pins on PORTB to high.
        PORTB::set_mask_raw(0xFF);

        small_delay();

        // Set all pins on PORTB to low.
        PORTB::unset_mask_raw(0xFF);

        small_delay();
    }
}

/// A small busy loop.
fn small_delay() {
    for _ in 0..400000 {
        unsafe { llvm_asm!("" :::: "volatile")}
    }
}
