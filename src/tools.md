Thanks for your interest! This book is currently outdated so I'm taking it down
until I get time to update it; that way it won't cause more confusion. In the
meantime, you can read [this blog post] that covers the *easy*, high level way
of writing Rust applications for any ARM Cortex-M microcontroller.

[this blog post]: http://blog.japaric.io/quickstart/

<!-- # Setting up a development environment -->

<!-- Working with embedded systems requires extra tooling as cross compilation is at -->
<!-- the heart of the development process. This chapter will introduce the tools -->
<!-- we'll use, why we need them and how to install them on the 3 major OSes. -->

<!-- Without further ado, these are the tools we'll use: -->

<!-- - [Rust & Cargo: nightly edition][rust] -->
<!-- - [A cross C linker: arm-none-eabi-gcc][gcc] -->
<!-- - [Binary inspection tools][binutils] -->
<!--   - [arm-none-eabi-objdump][objdump] -->
<!--   - [arm-none-eabi-size][size] -->
<!-- - [A debugger][*db] -->
<!-- - [An emulator: QEMU][qemu] -->
<!-- - [Xargo][xargo] -->
<!-- - [OpenOCD][openocd] -->

<!-- The rest of this page will justify each of these tools. For installation -->
<!-- instructions check the subsections of this chapter: [Linux], [Mac] and [Windows] -->

<!-- [Linux]: linux.html -->
<!-- [Mac]: macos.html -->
<!-- [Windows]: windows.html -->

<!-- ## Rust & Cargo: nightly edition -->
<!-- [rust]: tools.html#Rust%20%26%20Cargo%3A%20nightly%20edition -->

<!-- Rust & Cargo don't need  much explanation. To build Rust programs we'll need the -->
<!-- Rust compiler, `rustc`, and the Rust package manager, Cargo. -->

<!-- What does need justification is the use of the nightly channel. We need to use -->
<!-- the nightly channel because we'll make use of the following unstable features: -->

<!-- - `asm`: We'll use some inline assembly via the `asm!` syntax extension. But -->
<!--   that syntax extension hasn't been stabilized. -->

<!-- - `lang_items`: The need for this feature gate is sometimes artificial. The -->
<!--   compiler demands some lang times, like `panic_fmt`, to be defined even if the -->
<!--   functionality they provide (unwinding/panicking) is never used in our program. -->

<!-- - To cross compile programs we need a cross compiled `core` crate. The Rust -->
<!--   project doesn't distribute a binary release of that crate for Cortex-M -->
<!--   microcontrollers so we have to cross compile that crate ourselves. As the -->
<!--   `core` crate relies on lots of unstable features, we have to use the nightly -->
<!--   channel to cross compile it. -->

<!-- ## A cross C linker: `arm-none-eabi-gcc` -->
<!-- [gcc]: tools.html#A%20cross%20C%20linker%3A%20arm-none-eabi-gcc -->

<!-- (`arm-none-eabi-gcc` is not a linker per se but `rustc` uses it as a proxy for -->
<!-- `arm-none-eabi-ld`.) -->

<!-- > Wait, aren't we going to write Rust? Why do we need a *C compiler*? -->

<!-- `rustc` uses `gcc` to link intermediate object files so we pretty much have no -->
<!-- choice. -->

<!-- ## Binary inspection tools -->
<!-- [binutils]: tools.html#Binary%20inspection%20tools -->

<!-- When working this close to the hardware and with devices that have constrained -->
<!-- resources and mandatory memory layouts, it's pretty important to inspect the -->
<!-- produced binaries to keep track of their sizes and to check that the produced -->
<!-- binaries follow a specific memory layout. -->

<!-- We'll mainly use the following two tools: -->

<!-- ### `arm-none-eabi-size` -->
<!-- [size]: tools.html#arm-none-eabi-size -->

<!-- To keep an eye on the binary size of our applications. -->

<!-- ### `arm-none-eabi-objdump` -->
<!-- [objdump]: tools.html#arm-none-eabi-objdump -->

<!-- To confirm the memory layout of our program matches the memory layout -->
<!-- constraints of the target device. -->

<!-- ## A debugger -->
<!-- [*db]: tools.html#A%20debugger -->

<!-- A debugger is vital when working with microcontrollers as other debugging -->
<!-- methods like logging or *cough* `println` may not be available. This is -->
<!-- specially true when you are just starting out and haven't yet written drivers -->
<!-- for peripherals that allow microcontroller <-> PC communication. -->

<!-- We'll mainly use `arm-none-eabi-gdb` in this document as `lldb` doesn't provide -->
<!-- all the functionality we need. -->

<!-- ## An emulator: QEMU -->
<!-- [qemu]: tools.html#An%20emulator%3A%20QEMU -->

<!-- Before trying out our first program on real hardware, we'll run it under an -->
<!-- emulator to verify that the program works as expected. -->

<!-- ## Xargo -->
<!-- [xargo]: tools.html#Xargo -->

<!-- In general, cross compiling requires cross compiled "standard" crates like the -->
<!-- `core` or the `std` crate. Binary releases of these crates for Cortex-M -->
<!-- microcontrollers are not provided by the Rust project so we'll have to cross -->
<!-- compile these ourselves. Just cross compiling is not enough though, as one must -->
<!-- place the produced binaries in a specific directory layout called a "sysroot". -->
<!-- Because this process is cumbersome and error prone, I have [created Xargo]. -->
<!-- Xargo is a *transparent* Cargo wrapper that automatically builds and manages -->
<!-- sysroots without user intervention. -->

<!-- [created Xargo]: https://github.com/japaric/xargo -->

<!-- Usage looks like this: -->

<!-- ``` -->
<!-- $ cargo build --target cortex-m3 && echo OK -->
<!--    Compiling app v0.1.0 -->
<!-- error: can't find crate for `core` [E0463] -->

<!-- error: aborting due to previous error -->
<!-- error: Could not compile `app`. -->

<!-- To learn more, run the command again with --verbose. -->
<!-- ``` -->

<!-- ``` -->
<!-- $ xargo build --target thumbv6m-none-eabi && echo OK -->
<!--    Compiling core v0.0.0 (file://$sysroot/lib/rustlib/src/rust/src/libcore) -->
<!--    Compiling alloc v0.0.0 (file://$sysroot/lib/rustlib/src/rust/src/liballoc) -->
<!--     Finished release [optimized] target(s) in 19.56 secs -->
<!--    Compiling rustc_unicode v0.0.0 (file://$sysroot/lib/rustlib/src/rust/src/librustc_unicode) -->
<!--    Compiling collections v0.0.0 (file://$sysroot/lib/rustlib/src/rust/src/libcollections) -->
<!--     Finished release [optimized] target(s) in 5.67 secs -->
<!--     Finished release [optimized] target(s) in 0.0 secs -->
<!--    Compiling rand v0.0.0 (file://$sysroot/lib/rustlib/src/rust/src/librand) -->
<!--     Finished release [optimized] target(s) in 1.78 secs -->
<!--     Finished release [optimized] target(s) in 0.0 secs -->
<!--    Compiling app v0.1.0 (file://$PWD) -->
<!-- OK -->
<!-- ``` -->

<!-- ## OpenOCD -->
<!-- [openocd]: tools.html#OpenOCD -->

<!-- [OpenOCD] is a tool that lets your computer communicate with devices that -->
<!-- support the JTAG/SWD communication protocol. We'll use OpenOCD to *flash* -->
<!-- (write) our programs into the microcontroller and to debug them (with the help -->
<!-- of `gdb`). -->

<!-- [OpenOCD]: http://openocd.org/ -->
