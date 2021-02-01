#![feature(llvm_asm)]

#![no_std]
#![no_main]

use ruduino::Register;
use ruduino::cores::current::{DDRB, PORTB};

#[no_mangle]
pub extern fn main() {
    // Set all PORTB pins up as outputs
    DDRB::set_mask_raw(0xFFu8);

    loop {
        // Set all pins on PORTB to high.
        PORTB::set_mask_raw(0xFF);

        ruduino::delay::delay_ms(1000);

        // Set all pins on PORTB to low.
        PORTB::unset_mask_raw(0xFF);

        ruduino::delay::delay_ms(1000);
    }
}
