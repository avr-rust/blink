# blink

An small Hello World Rust application for the AVR.

The program itself toggles a LED on PORTB periodically.

Designed for the ATmega328p.

[How to set up a cross compiler](https://github.com/avr-rust/rust)

# Usage

```bash
rustup run avr-toolchain xargo build --target avr-atmega328p --release

# there is now an ELF file at target/atmega328p/release/blink.elf
```

You may need to invoke the build like this instead, as the current
version of `avr-rust` is based on the dev channel:

```
XARGO_RUST_SRC=/path/to/avr-rust rustup run avr-toolchain xargo build --target avr-atmega328p --release --verbose
```
