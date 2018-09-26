#![feature(lang_items)]

// We won't use the usual `main` function. We are going to use a different
// "entry point".
#![no_main]

// We won't use the standard library because it requires OS abstractions like
// threads and files and those are not available on this platform.
#![no_std]

// Conceptually, this is our program "entry point". It's the first thing the
// microcontroller will execute when it (re)boots. This entry point must be a
// `pub`lic function named `_reset` to be recognized as such because that's what
// our linker script (`layout.ld`) states. Later, we'll say more about these
// requirements.
//
// Also, returning from this function is undefined because there is nothing to
// return to! To statically forbid returning from this function, we mark it as
// "divergent", hence the `fn() -> !` signature.
#[export_name = "_reset"]
pub extern "C" fn main() -> ! {
    // Our first program initializes some variables on the stack and does
    // nothing more. Yay!
    let y;
    let x = 42;
    y = x;

    // We can't return from this function so we just spin endlessly here.
    loop {}
}

// Finally, we need to define the panic_fmt "lang item", which is just a
// function. This specifies what the program should do when a `panic!` occurs.
// Our program won't panic, so we can leave the function body empty for now.
mod lang_items {
    #[lang = "panic_fmt"]
    extern "C" fn panic_fmt() {}
}
