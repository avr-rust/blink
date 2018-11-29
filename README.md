# blink

An small Hello World Rust application for the AVR.

The program itself toggles a LED on PORTB periodically.

Designed for the ATmega328p.

[How to set up a cross compiler](https://github.com/avr-rust/rust)

# Usage

There are a few environment variables that need to be set first:

```bash
# Needed until https://github.com/japaric/xargo/pull/205 goes through,
# to tell it where to find avr-atmega328p.json:
export RUST_TARGET_PATH=`pwd`

# Likely needed if you've just compiled avr-rust from source:
export XARGO_RUST_SRC=/path_to_avr_rust/src
# (e.g. :if you built from sources and typed `git clone https://github.com/avr-rust/rust.git` in `~/avr-rust`, this would be `~/avr-rust/rust/src)
```

Now to build, run:

```bash
rustup run avr-toolchain xargo build --target avr-atmega328p --release

```
There should now be an ELF file at `target/atmega328p/release/blink.elf`.
