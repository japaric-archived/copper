#![feature(lang_items)]

// We won't use the usual `main` function. We are going to use a different "entry point".
#![no_main]

// We won't use the standard library because it requires OS abstractions like threads and files and
// those are not available in this platform.
#![no_std]

// Conceptually, this is our program "entry point". It's the first thing the microcontroller will
// execute when it (re)boots. (As far as the linker is concerned the entry point must be named
// `start` (by default; it can have a different name). That's why this function is `pub`lic, named
// `start` and is marked as `#[no_mangle]`.)
//
// Returning from this function is undefined because there is nothing to return to! To statically
// forbid returning from this function, we mark it as divergent, hence the `fn() -> !` signature.
#[no_mangle]
pub fn start() -> ! {
    // Our first program initializes some variables on the stack and does nothing more. Yay!
    let x = 42;
    let y = x;

    // We can't return from this function so we just spin endlessly here.
    loop {}
}

// Ignore this part for now :-). It will covered in a later section.
mod vector_table {
    #[link_section = ".reset"]
    static RESET: fn() -> ! = ::start;
}

// Finally, we need to define the panic_fmt and eh_personality "lang item"s, which are just
// functions. This specifies what the program should do when a `panic!` occurs. Our program won't
// panic, so we can leave the function bodies empty for now.
mod lang_items {
    #[lang = "panic_fmt"]
    extern fn panic_fmt() {}

    #[lang = "eh_personality"]
    pub extern fn rust_eh_personality() {}
}
