#![feature(lang_items, unwind_attributes)]

#![no_std]
#![no_main]

#[no_mangle]
pub extern fn main() {
}

// These do not need to be in a module, but we group them here for clarity.
pub mod std {
    #[lang = "eh_personality"]
    #[no_mangle]
    pub unsafe extern "C" fn rust_eh_personality(state: (), exception_object: *mut (), context: *mut ()) -> () {
    }

    #[lang = "panic_fmt"]
    #[unwind]
    pub extern fn rust_begin_panic(msg: (), file: &'static str, line: u32) -> ! {
        loop { }
    }
}

