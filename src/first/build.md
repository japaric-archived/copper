Thanks for your interest! This book is currently outdated so I'm taking it down
until I get time to update it; that way it won't cause more confusion. In the
meantime, you can read [this blog post] that covers the *easy*, high level way
of writing Rust applications for any ARM Cortex-M microcontroller.

[this blog post]: http://blog.japaric.io/quickstart/

<!-- # Build & inspect -->

<!-- Let's start by creating a new Cargo project: -->

<!-- ``` -->
<!-- $ cargo new --bin app && cd $_ -->
<!-- ``` -->

<!-- And then add these files to the project: -->

<!-- - [layout.ld](./first/layout.ld) - a "linker script" -->

<!-- - [.cargo/config](./first/config) - a project local Cargo configuration -->
<!--   file -->

<!-- - [src/main.rs](./first/main.rs) -->

<!-- I'll explain what the linker script and the Cargo configuration file are for in -->
<!-- a later section. It's not important to know what it does *right now*. -->

<!-- Your project directory should look like this: -->

<!-- ``` -->
<!-- $ tree . -->
<!-- . -->
<!-- ├── .cargo -->
<!-- │   └── config -->
<!-- ├── Cargo.toml -->
<!-- ├── layout.ld -->
<!-- └── src -->
<!--     └── main.rs -->
<!-- ``` -->

<!-- For convenience, `src/main.rs` is replicated here: -->

<!-- ``` rust -->
<!-- #![feature(lang_items)] -->

<!-- // We won't use the usual `main` function. We are going to use a different -->
<!-- // "entry point". -->
<!-- #![no_main] -->

<!-- // We won't use the standard library because it requires OS abstractions like -->
<!-- // threads and files and those are not available on this platform. -->
<!-- #![no_std] -->

<!-- // Conceptually, this is our program "entry point". It's the first thing the -->
<!-- // microcontroller will execute when it (re)boots. This entry point must be a -->
<!-- // `pub`lic function named `_reset` to be recognized as such because that's what -->
<!-- // our linker script (`layout.ld`) states. Later, we'll say more about these -->
<!-- // requirements. -->
<!-- // -->
<!-- // Also, returning from this function is undefined because there is nothing to -->
<!-- // return to! To statically forbid returning from this function, we mark it as -->
<!-- // "divergent", hence the `fn() -> !` signature. -->
<!-- #[export_name = "_reset"] -->
<!-- pub extern "C" fn main() -> ! { -->
<!--     // Our first program initializes some variables on the stack and does -->
<!--     // nothing more. Yay! -->
<!--     let y; -->
<!--     let x = 42; -->
<!--     y = x; -->

<!--     // We can't return from this function so we just spin endlessly here. -->
<!--     loop {} -->
<!-- } -->

<!-- // Finally, we need to define the panic_fmt "lang item", which is just a -->
<!-- // function. This specifies what the program should do when a `panic!` occurs. -->
<!-- // Our program won't panic, so we can leave the function body empty for now. -->
<!-- mod lang_items { -->
<!--     #[lang = "panic_fmt"] -->
<!--     extern "C" fn panic_fmt() {} -->
<!-- } -->
<!-- ``` -->

<!-- If you have written a native Rust program before, this shouldn't look that -->
<!-- different except that `main` has a different signature. -->

<!-- Now you can build the Cargo project: -->

<!-- ``` -->
<!-- $ xargo build --target thumbv7m-none-eabi -->
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
<!-- ``` -->

<!-- > **NOTE** Do **not** build the project in release mode! -->

<!-- > **NOTE** You may not see Xargo compile `core` and the other "standard" crates -->
<!-- > if the sysroot was already compiled. -->

<!-- ## Trust, but verify -->

<!-- The project appeared to build fine, but it's always a good idea to inspect the -->
<!-- executable. We are going to look at three things: -->

<!-- ### Memory layout -->

<!-- First, you have to appreciate the fact that, during its execution, everything in -->
<!-- a program resides in (physical) memory and has an associated memory address. -->
<!-- Now, everything that's *statically allocated*, like functions and static -->
<!-- variables, gets assigned an address at compile time. Whereas, everything else, -->
<!-- like local/stack variables and "heap" allocated variables, gets assigned an -->
<!-- address at runtime. -->

<!-- We can observe the addresses of statically allocated stuff using `objdump`. -->
<!-- Let's check our executable: -->

<!-- ``` -->
<!-- # with -->
<!-- $ arm-none-eabi-objdump --demangle --disassemble target/thumbv7m-none-eabi/debug/app -->

<!-- # or its shorter form -->
<!-- $ arm-none-eabi-objdump -Cd target/thumbv7m-none-eabi/debug/app -->

<!-- target/thumbv7m-none-eabi/debug/app:     file format elf32-littlearm -->


<!-- Disassembly of section .text: -->

<!-- 00000000 <_reset-0x8>: -->
<!--    0:   20010000        .word   0x20010000 -->
<!--    4:   00000009        .word   0x00000009 -->

<!-- 00000008 <_reset>: -->
<!--    8:   b083            sub     sp, #12 -->
<!--    a:   e7ff            b.n     c <_reset+0x4> -->
<!--    c:   202a            movs    r0, #42 ; 0x2a -->
<!--    e:   9002            str     r0, [sp, #8] -->
<!--   10:   9001            str     r0, [sp, #4] -->
<!--   12:   e7ff            b.n     14 <_reset+0xc> -->
<!--   14:   e7fe            b.n     14 <_reset+0xc> -->
<!-- ``` -->

<!-- > **NOTE** If you get an output that significantly different from what's shown -->
<!-- > above. For example, if the `<_reset>` part is missing, then likely your linker -->
<!-- > has some sort of bug. We have had quite a few reports about that. The solution -->
<!-- > is to try a different linker. -->

<!-- Let me highlight the important bits of the output: -->

<!-- ``` -->
<!-- 00000000 -->
<!--    0:	20010000 -->

<!-- 00000004 -->
<!--    4:	00000009 -->

<!-- 00000008 <_reset>: -->
<!--    8:                   sub     sp, #12 -->
<!--    a:                   b.n     c <_reset+0x4> -->
<!--    c:                   movs    r0, #42 ; 0x2a -->
<!--    e:                   str     r0, [sp, #8] -->
<!--   10:                   str     r0, [sp, #4] -->
<!--   12:                   b.n     14 <_reset+0xc> -->
<!--   14:                   b.n     14 <_reset+0xc> -->
<!-- ``` -->

<!-- Now let's read the output section by section: -->

<!-- ``` -->
<!-- 00000000 -->
<!--    0:	20010000 -->
<!-- ``` -->

<!-- "The memory at address `0x0` holds the (`u32`) value `0x2001_0000`". When the -->
<!-- program starts its execution, that part of the memory will be already -->
<!-- initialized to that value. -->

<!-- ``` -->
<!-- 00000004 -->
<!--    4:	00000009 -->
<!-- ``` -->

<!-- Likewise, "the memory at `0x4` holds the  (`u32`) value `0x9`". -->

<!-- ``` -->
<!-- 00000008 <start>: -->
<!--    8:	          	sub	sp, #8 -->
<!--    a:	          	movs	r0, #42	; 0x2a -->
<!--    c:	          	str	r0, [sp, #4] -->
<!--    e:	          	str	r0, [sp, #0] -->
<!--   10:	          	b.n	12 <start+0xa> -->
<!--   12:	          	b.n	12 <start+0xa> -->
<!-- ``` -->

<!-- Finally, "the `start` function resides at address `0x8`". The lines, like -->
<!-- `sub sp, #8`, below `<start>` are the instructions that make up the function. -->
<!-- When the processor executes this function, it will execute each one of these -->
<!-- instructions. -->

<!-- But, why are these addresses/values important? During boot, the microcontroller -->
<!-- reads the memory section at `0x0..0x8` and uses those  values to complete the -->
<!-- boot process. In other words, the values in that memory section have an special -->
<!-- meaning to the boot process. In a [later section], we'll explain what those -->
<!-- values mean and how they are used in the boot process. For now, it's okay to -->
<!-- just confirm that the memory at `0x0..0x8` is initialized. -->

<!-- [later section]: details/boot.html -->

<!-- ### Program size -->

<!-- Microcontrollers are memory constrained devices. It's important to keep an eye -->
<!-- on the size of our programs to make sure they don't exceed the memory capacity -->
<!-- of our device. We'll use the `size` command to do that: -->

<!-- ``` -->
<!-- $ arm-none-eabi-size target/thumbv7m-none-eabi/debug/app -->
<!--    text    data     bss     dec     hex filename -->
<!--      22       0       0      22      16 target/thumbv7m-none-eabi/debug/app -->
<!-- ``` -->

<!-- The output mentions three different memory *sections*: -->

<!-- - The `text` section contains the "program code", i.e. all the instructions that -->
<!--   the processor will execute when the program is executed. Functions go in this -->
<!--   section. This section also stores   constants like strings. `static CONSTANT: -->
<!--   &'static str = "Hello, world!"`is an example of a constant. -->
<!-- - `data` holds static variables that have an initial value, for example `static -->
<!--   mut X: i32 = 42`. -->
<!-- - On the contrary, `bss` holds "uninitialized" static variables. Because of C -->
<!--   heritage, here "uninitialized" actually means *zeroed*. For example, `static -->
<!--   mut ZEROS: [u8; 4] = [0; 4]` would go in this section. -->

<!-- `dec` is just the sum of these three sections and `hex` is the hexadecimal -->
<!-- representation of `dec`. -->

<!-- Our program size is just 22 bytes big! Microcontrollers usually have memory -->
<!-- capacities in the order of a few KiB up to hundreds of KiB. In particular, the -->
<!-- LM3S6965 microcontroller, which we are going to emulate, has 256KiB of (Flash) -->
<!-- memory. So, no problem here; this program will fit in the device memory. -->

<!-- > **NOTE** KiB = [Kibibyte] = 1024 bytes -->

<!-- [Kibibyte]: https://en.wikipedia.org/wiki/Kibibyte -->

<!-- ### Entry point -->

<!-- To work properly, tools like `gdb` and `qemu` need to be informed about the -->
<!-- program entry point, which is where the program starts its execution. -->

<!-- We can check our program entry point using the `readelf` command: -->

<!-- ``` -->
<!-- $ arm-none-eabi-readelf -h target/thumbv7m-none-eabi/debug/app -->
<!-- ELF Header: -->
<!--   Magic:   7f 45 4c 46 01 01 01 00 00 00 00 00 00 00 00 00 -->
<!--   Class:                             ELF32 -->
<!--   Data:                              2's complement, little endian -->
<!--   Version:                           1 (current) -->
<!--   OS/ABI:                            UNIX - System V -->
<!--   ABI Version:                       0 -->
<!--   Type:                              EXEC (Executable file) -->
<!--   Machine:                           ARM -->
<!--   Version:                           0x1 -->
<!--   Entry point address:               0x9 <-- -->
<!--   Start of program headers:          52 (bytes into file) -->
<!--   Start of section headers:          67236 (bytes into file) -->
<!--   Flags:                             0x5000200, Version5 EABI, soft-float ABI -->
<!--   Size of this header:               52 (bytes) -->
<!--   Size of program headers:           32 (bytes) -->
<!--   Number of program headers:         2 -->
<!--   Size of section headers:           40 (bytes) -->
<!--   Number of section headers:         15 -->
<!--   Section header string table index: 12 -->
<!-- ``` -->

<!-- There you can see that `0x9` is the entry point address. From the previous -->
<!-- `objdump` output, you know that `0x8` is the address of the `main` function -->
<!-- which we want to be the entry point. At first, this looks wrong because the -->
<!-- address is off by 1, but this is actually correct. -->

<!-- Cortex M microcontrollers always operate in "thumb" mode (i.e. they use "thumb" -->
<!-- instructions). To indicate this thumb state all function address have their -->
<!-- first bit (position 0) set to 1. Because `0x8 | 0b1` is `0x9`, `0x9` is the -->
<!-- address of `main` when called in thumb mode. -->

<!-- This means that `main` have been correctly declared as the entry point. -->
