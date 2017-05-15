Thanks for your interest! This book is currently outdated so I'm taking it down
until I get time to update it; that way it won't cause more confusion. In the
meantime, you can read [this blog post] that covers the *easy*, high level way
of writing Rust applications for any ARM Cortex-M microcontroller.

[this blog post]: http://blog.japaric.io/quickstart/

<!-- # Linker script -->

<!-- As we saw in the previous section, our program must comply with a specific -->
<!-- memory layout for the microcontroller to work properly (otherwise it won't -->
<!-- boot!). -->

<!-- The linker is what ultimately determines the memory layout of our program. To -->
<!-- some extent, we can control the linker, and therefore the memory layout of our -->
<!-- program, using a file called linker script. -->

<!-- For our first program, I gave you the linker script `layout.ld`. In this -->
<!-- section, I'll explain its contents. -->

<!-- ## Terminology -->

<!-- First, let's define some terms you'll often hear when dealing with linker -->
<!-- scripts. -->

<!-- ### Linking -->

<!-- The process of producing a binary from a Rust source code involves two steps (or -->
<!-- several more if you peek inside `rustc`'s pipeline): compiling and linking. -->
<!-- Compiling is the action of producing an intermediate file known as "object file" -->
<!-- from the source code. In Rust, the minimal compilation unit is the crate which -->
<!-- can be a single `.rs` or a collection of them. Each crate gets compiled to an -->
<!-- object file (actually, dependencies get compiled to `.rlib`s which are a object -->
<!-- files plus some "metadata"). To produce the final binary/executable, the -->
<!-- compiled crates get "merged" together; this process is known as linking. -->

<!-- ### Symbol -->

<!-- A symbol is either a function or a static variable. Each symbol has a name, a -->
<!-- start address and occupies some space memory. For example, our program entry -->
<!-- point is a symbol with name `_reset` and address `0x8` and occupies 14 bytes (7 -->
<!-- 16-bit instructions) in memory. -->

<!-- ### Section -->

<!-- A section is a collection of symbols stored in contiguous memory. Other way to -->
<!-- think about this is that symbols are *organized* in sections. -->

<!-- ### Region -->

<!-- A (memory) region is a span of memory that's described using a start address and -->
<!-- a length (in bytes). For example, the LM3S6965 has two memory regions: Its flash -->
<!-- memory region which starts at address `0x0` and has a size of 256 KiB, and its -->
<!-- RAM region which starts at address `0x2000_0000` and has a size of 64 KiB. -->

<!-- ### Object files -->

<!-- The linker takes as input one or more object files and outputs a single object -->
<!-- file. In this book, we'll be exclusively working with object files in the ELF -->
<!-- format. ELF also happens to be the format Linux executables use, but there exist -->
<!-- other formats like Mach-O (macOS) and COFF (Windows). -->

<!-- ## `layout.ld` -->

<!-- Instead of explaining the linker script syntax (which is documented -->
<!-- [elsewhere]), I'm going to focus on explaining the contents of the `layout.ld` -->
<!-- file and how this file relates to the boot process we covered in the previous -->
<!-- section. -->

<!-- [elsewhere]: https://sourceware.org/binutils/docs/ld/Scripts.html -->

<!-- For convenience, here's the full linker script. -->

<!-- ``` -->
<!-- ENTRY(_reset); -->

<!-- MEMORY -->
<!-- { -->
<!--   FLASH : ORIGIN = 0x00000000, LENGTH = 256K -->
<!--   RAM   : ORIGIN = 0x20000000, LENGTH = 64K -->
<!-- } -->

<!-- SECTIONS -->
<!-- { -->
<!--   .text : -->
<!--   { -->
<!--     /* Vector table */ -->
<!--     LONG(ORIGIN(RAM) + LENGTH(RAM)) -->
<!--     LONG(_reset + 1); -->

<!--     /* Reset handler */ -->
<!--     _reset = .; -->
<!--     *(.text._reset) -->

<!--     *(.text*) -->
<!--   } > FLASH -->

<!--   /DISCARD/ : -->
<!--   { -->
<!--     *(.ARM.exidx*) -->
<!--     *(.note.gnu.build-id*) -->
<!--   } -->
<!-- } -->
<!-- ``` -->

<!-- Let's go over it block by block -->

<!-- ### ENTRY -->

<!-- `ENTRY(_reset)`. This indicates that the `_reset` symbol is our program entry -->
<!-- point. Getting the entry point right is important because the linker will remove -->
<!-- any symbol that is not referenced by the entry point because it's "dead code". -->
<!-- If you get it wrong, then the compiler may remove all the symbols and produce an -->
<!-- empty executable. -->

<!-- Because `_reset` is the name of entry point, our Rust program must somehow -->
<!-- expose a symbol with that exact name. That's why we used the `export_name` -->
<!-- attribute in our program: -->

<!-- ``` -->
<!-- #[export_name = "_reset"] -->
<!-- pub extern "C" fn main() -> ! { .. } -->
<!-- ``` -->

<!-- Note that the function has type `extern "C" fn`. This forces the compiler to -->
<!-- lower this function to a subroutine that adheres to the C ABI. We use the C ABI -->
<!-- rather than Rust's ABI here because Rust's ABI is not stable and because the -->
<!-- Cortex-M processor expects the subroutine to use the C ABI. -->

<!-- Also, mind you that getting the name right is not sufficient for this to work. -->
<!-- The symbol must also be a "global" symbol. This "global" requirement is -->
<!-- fulfilled by making `main` public. -->

<!-- `nm` is a useful tool to debug problems about symbol names and visibility. Let's -->
<!-- give it a try to verify that our program generates a global symbol named -->
<!-- `_reset`. First, you'll have to generate an object file from your crate. -->
<!-- ``` -->
<!-- $ xargo rustc --target thumbv7m-none-eabi -- --emit=obj -->
<!-- ``` -->

<!-- The reason we want an object file is that we want to look at the symbols `rustc` -->
<!-- generates from our program *before* the linker gets a chance to drop them. -->

<!-- The object file will be named `app.o` and will sit in the -->
<!-- `target/thumbv7m-none-eabi/debug` directory right next to the `app` executable. -->

<!-- Next, we call `nm` to inspect the symbols and their visibility: -->

<!-- ``` -->
<!-- $ arm-none-eabi-nm --demangle target/thumbv7m-none-eabi/debug/app.o -->

<!-- # or its shorter form -->
<!-- $ arm-none-eabi-nm -C target/thumbv7m-none-eabi/debug/app.o -->
<!-- (..) -->
<!--          U __aeabi_unwind_cpp_pr0 -->
<!-- 00000000 T _reset -->
<!-- 00000000 T rust_begin_unwind -->
<!-- 00000000 N __rustc_debug_gdb_scripts_section__ -->
<!-- ``` -->

<!-- In the output, you can see the `_reset` symbol and an uppercase `T` right next -->
<!-- to it. This `T` indicates that `_reset` is a global symbol that resides in the -->
<!-- `text` section (it's part of the program code). (See the [nm manual] for more -->
<!-- details) -->

<!-- [nm manual]: https://sourceware.org/binutils/docs/binutils/nm.html -->

<!-- As homework, try changing the `main` function a little: remove the `export_name` -->
<!-- attribute and/or the `pub` modifier. How does the `nm` output changes? Also -->
<!-- check how the `app` executable changes using `objdump`. -->

<!-- ### MEMORY -->

<!-- ``` -->
<!-- MEMORY -->
<!-- { -->
<!--   FLASH : ORIGIN = 0x00000000, LENGTH = 256K -->
<!--   RAM  : ORIGIN = 0x20000000, LENGTH = 64K -->
<!-- } -->
<!-- ``` -->

<!-- > **NOTE** In linker scripts, `K` means `1024` (not `1000`!) , `M` means -->
<!-- > `1048576` (i.e. `1024 * 1024`) and so on. -->

<!-- The `MEMORY` block declares two memory regions: one named FLASH and the other -->
<!-- named RAM. These regions represent the flash memory and RAM regions of the -->
<!-- LM3S6965. The values here match the memory specification I gave you in the -->
<!-- [previous section]. -->

<!-- [previous section]: details/ld.html#Region -->

<!-- ### SECTIONS -->

<!-- ``` -->
<!-- SECTIONS -->
<!-- { -->
<!--   /* .. */ -->
<!-- } -->
<!-- ``` -->

<!-- The `SECTIONS` block declares sections and assigns each section to a memory -->
<!-- region. It's important to assign each and every section to a memory region, -->
<!-- otherwise you'll get weird "overlap" linker errors. -->

<!-- #### .text -->

<!-- The .text section goes into the Flash region which starts at address `0x0`. And, -->
<!-- if you remember, the vector table goes in address `0x0`. So, we must place the -->
<!-- vector table at the beginning of the .text section. -->

<!-- ``` -->
<!--   .text : -->
<!--   { -->
<!--     /* Vector table */ -->
<!--     LONG(ORIGIN(RAM) + LENGTH(RAM)) -->
<!--     LONG(_reset + 1); -->
<!-- ``` -->

<!-- From the vector table, we only use its first two elements at this time: -->

<!-- - The initial value of the stack pointer. As we mentioned before this is usually -->
<!--   set to the largest valid RAM address and that would be `0x2001_0000` for the -->
<!--   LSM303DLHC. Instead of hard coding that value, we can compute it from the -->
<!--   `RAM` region we declared before and that's what the `LONG(ORIGIN(RAM) + -->
<!--   LENGTH(RAM))` line does. -->

<!-- - The reset vector. This is the address of the reset handler, `_reset`. We add -->
<!--   1 because the processor is in Thumb mode. ([remember?]) -->

<!-- [remember?]: first/build.html#Entry%20point -->

<!-- Right after the vector table we place the reset handler, the actual function -->
<!-- with its instructions. -->

<!-- ``` -->
<!--     /* Reset handler */ -->
<!--     _reset = .; -->
<!--     *(.text._reset) -->
<!-- ``` -->

<!-- We do that with the `*(.text._reset)` part. Which basically means place *all* -->
<!-- (`*()`) the symbols, i.e. from all the input object files, named `_reset` here -->
<!-- (but there can only be one symbol with that exact name). -->

<!-- Then we have this `_reset = .` line before the `*(.text._reset)` bit. That -->
<!-- assigns the address of the `_reset` symbol to the `_reset` variable. And we used -->
<!-- that `_reset` variable in the vector table part (the `LONG(_reset + 1)` line). -->

<!-- We also place the other text symbols (functions and constants) from all the -->
<!-- input object files in this section: -->

<!-- ``` -->
<!--     *(.text*) -->
<!-- ``` -->

<!-- Finally, we indicate to the linker that this `.text` section goes in the `FLASH` -->
<!-- memory region. -->

<!-- ``` -->
<!--   } > FLASH -->
<!-- ``` -->

<!-- #### /DISCARD/ -->

<!-- ``` -->
<!--   /DISCARD/ : -->
<!--   { -->
<!--     *(.ARM.exidx*) -->
<!--     *(.note.gnu.build-id*) -->
<!--   } -->
<!-- ``` -->

<!-- `/DISCARD/` is not a real section. Everything in this "special section" gets -->
<!-- discarded by the linker and doesn't make it to the output object. So, why are we -->
<!-- discarding these specific symbols? -->

<!-- - `.ARM.exidx*` These symbols are related to unwinding. Which we aren't using! -->
<!--   If we leave these symbols in, they'll in turn demand other symbols related to -->
<!--   unwinding and cause "undefined reference" linker errors. -->

<!-- - `note.gnu.build-id*`. These symbols are introduced by `gcc`, which is the -->
<!--   linker we are using. If not removed, these symbols will try to place -->
<!--   themselves at address `0x0` causing "overlap" linker errors. -->

<!-- ## Inspect again -->

<!-- Let's look again at the `objdump` output from section [3.1]. We should now be -->
<!-- able to understand it better: -->

<!-- [3.1]: ./first/build.html -->

<!-- ``` -->
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
<!--    e:   9001            str     r0, [sp, #4] -->
<!--   10:   9002            str     r0, [sp, #8] -->
<!--   12:   e7ff            b.n     14 <_reset+0xc> -->
<!--   14:   e7fe            b.n     14 <_reset+0xc> -->
<!-- ``` -->

<!-- Let's go over it part by part but in reverse! -->

<!-- ``` -->
<!-- 00000008 <_reset>: -->
<!--    8:	b082      	sub	sp, #8 -->
<!--    a:	202a      	movs	r0, #42	; 0x2a -->
<!--    c:	9001      	str	r0, [sp, #4] -->
<!--    e:	9000      	str	r0, [sp, #0] -->
<!--   10:	e7ff      	b.n	12 <start+0xa> -->
<!--   12:	e7fe      	b.n	12 <start+0xa> -->
<!-- ``` -->

<!-- `_reset` is the reset handler and our program entry point. This is first thing -->
<!-- the microcontroller will execute when it boots. Note the address `0x8`, this -->
<!-- function is in the Flash memory region. -->

<!-- Then we have: -->

<!-- ``` -->
<!-- 00000000 <_reset-0x8>: -->
<!--    0:   20010000        .word   0x20010000 -->
<!--    4:   00000009        .word   0x00000009 -->
<!-- ``` -->

<!-- These two "words" are the vector table which resides at address `0x0`. Even -->
<!-- though the vector table is in the `.text` section, the vector table is not -->
<!-- composed of instructions but of data. -->

<!-- ``` -->
<!-- 00000000 <_reset-0x8>: -->
<!--    0:   20010000        .word   0x20010000 -->
<!-- ``` -->

<!-- The first element of the vector table is an "anonymous" symbol (though the -->
<!-- linker automatically generates a name for it, based on the names of nearby -->
<!-- symbols) with value `0x2001_0000`. `0x2001_0000` is the highest valid RAM -->
<!-- address of the LM3S6965. Which we are going to use as the initial value of the -->
<!-- stack pointer. This address is where the call stack will be created. -->

<!-- ``` -->
<!--    4:   00000009        .word   0x00000009 -->
<!-- ``` -->

<!-- The second element of the vector table is the reset "vector", which is nothing -->
<!-- more than a pointer to the reset handler, `_reset`. Even though the reset -->
<!-- handler lives in address `0x8`, this symbol has value `0x9` to indicate that the -->
<!-- reset handler must be called in thumb mode. -->

<!-- ## Closing comment -->

<!-- You don't need to commit all this to memory right now -- I know it's a lot to -->
<!-- take in. But, yes, linker scripts are important and we'll be dealing with them -->
<!-- again in the future. At that time, feel free to refer back to this section as a -->
<!-- reference. For now, the take home message is: -->

<!-- - When programming against hardware, your program must follow a device-specific -->
<!--   memory layout. -->

<!-- - You can use a linker script to adjust your program memory layout to meet those -->
<!--   requirements. -->
