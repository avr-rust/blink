# blink

A small Hello World Rust application for the AVR.

The program itself toggles a LED on PORTB periodically.

Designed for the ATmega328p.

[The AVR-Rust Book](https://book.avr-rust.com/)

## Prerequisites

  * A recent version of the nightly Rust compiler. Anything including or
    greater than `rustc 1.63.0-nightly (fee3a459d 2022-06-05)` can be used.
  * The rust-src rustup component - `$ rustup component add rust-src`
  * AVR-GCC on the system for linking
  * AVR-Libc on the system for support libraries

## Usage


Now to build, run:

```bash
rustup override set nightly

# Ensure time delays are consistent with a 16MHz microcontroller.
export AVR_CPU_FREQUENCY_HZ=16000000

# Compile the crate to an ELF executable.
cargo build -Z build-std=core --target avr-atmega328p.json --release

# Note: there is work in progress to make that easier
#       peeking in .cargo/config.tom  will show that
#            cargo build --release
#       is the short version
```
There should now be an ELF file at `target/avr-atmega328p/release/blink.elf`. It
can be flashed directly to an AVR microcontroller or ran inside a simulator.


## Resources

  * The [AVR-Rust book](https://book.avr-rust.com)

