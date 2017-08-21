# blink

An small Hello World Rust application for the AVR.

The program itself toggles a LED on PORTB periodically.

Designed for the ATmega328p.

[How to set up a cross compiler](https://github.com/avr-rust/rust)

# Usage

```bash
xargo build --target avr-atmega328p

# there is now an ELF file at target/atmega328p/debug/blink.elf
```

