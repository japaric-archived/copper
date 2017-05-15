Thanks for your interest! This book is currently outdated so I'm taking it down
until I get time to update it; that way it won't cause more confusion. In the
meantime, you can read [this blog post] that covers the *easy*, high level way
of writing Rust applications for any ARM Cortex-M microcontroller.

[this blog post]: http://blog.japaric.io/quickstart/

<!-- # Exceptions: Crashing your micro -->

<!-- In the previous chapter we wrote a very simple program, built a binary from it, -->
<!-- verified that said binary was well-formed (i.e. that it had a vector table where -->
<!-- expected), and even executed it under an emulator and also on real hardware. -->
<!-- Everything went smoothly! However, when developing things are bound to go awry -->
<!-- at some point. In the worst case scenario, one might, by mistake, ask the -->
<!-- microcontroller to perform an operation that it can't handle, like: -->

<!-- - Reading/writing to a invalid memory address e.g. beyond the limits of the RAM -->
<!--   region. -->

<!-- - Writing to flash memory using simple assignments (`*ptr = value`). Flash -->
<!--   memory has an elaborate write mechanism. -->

<!-- - Executing an instruction that the microcontroller doesn't support. Like trying -->
<!--   to execute a floating point instruction (e.g. `vadd.f32`) on a device without -->
<!--   a FPU. -->

<!-- Under these situations, the processor will raise a *hardware exception*. When an -->
<!-- exception is raised, the normal flow of our program is *interrupted* (stopped) -->
<!-- and the processor jumps to an exception *handler* (recall: handler is just -->
<!-- another name for function). -->

<!-- There are several types of exceptions but most of them signify a catastrophic -->
<!-- failure and, in real applications, are usually handled by resetting the system -->
<!-- (if bare metal) or by resetting/killing the faulty process (if running on top of -->
<!-- an OS). These catastrophic exceptions should happen rarely though (if ever) -->
<!-- during the execution of a program; hence the name "exception". -->

<!-- As we are just starting out and we are going to be running our programs under -->
<!-- the debugger, we'll handle all the exceptions the same way: we'll halt the -->
<!-- processor and yield control back to the debugger using the `bkpt` instruction. -->

<!-- > **NOTE** Executing the `bkpt` instruction when there is no debugger attached -->
<!-- > to the processor will make the processor raise an exception! -->

<!-- ## Installing the exception handlers -->

<!-- As the official ARM [documentation] states, the exception vectors (recall: a -->
<!-- vector is a pointer to a handler, i.e. a function pointer) must be stored in the -->
<!-- vector table just right after the reset vector. We'll extend our linker script, -->
<!-- `layout.ld`, to account for this: -->

<!-- [documentation]: http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0553a/BABIFJFG.html -->

<!-- ``` diff -->
<!--      /* Vector table */ -->
<!--      LONG(ORIGIN(RAM) + LENGTH(RAM)) -->
<!--      LONG(_reset + 1); -->
<!-- +    KEEP(*(.rodata._EXCEPTIONS)); -->

<!--      /* Reset handler */ -->
<!--      _reset = .; -->
<!-- ``` -->

<!-- The change is quite literal: "Place the exceptions (the `_EXCEPTIONS` symbol) -->
<!-- right after the address of the reset handler (`LONG(_reset + 1)`)". -->

<!-- Note that this time we use `.rodata._EXCEPTIONS` rather than -->
<!-- `.text._EXCEPTIONS`. The reason is that the `_EXCEPTIONS` symbol is "data" and -->
<!-- not a function. -->

<!-- Now, we need create this `_EXCEPTIONS` symbol in our program: -->

<!-- ``` rust -->
<!-- // Add this to `src/main.rs` -->

<!-- #![feature(asm)] -->

<!-- mod exception { -->
<!--     pub extern "C" fn handler() { -->
<!--         unsafe { -->
<!--             asm!("bkpt"); -->
<!--         } -->

<!--         loop {} -->
<!--     } -->

<!--     #[export_name = "_EXCEPTIONS"] -->
<!--     pub static EXCEPTIONS: [Option<extern "C" fn()>; 14] = [Some(handler), // NMI -->
<!--                                                             Some(handler), // Hard fault -->
<!--                                                             Some(handler), // Memmanage fault -->
<!--                                                             Some(handler), // Bus fault -->
<!--                                                             Some(handler), // Usage fault -->
<!--                                                             None, // Reserved -->
<!--                                                             None, // Reserved -->
<!--                                                             None, // Reserved -->
<!--                                                             None, // Reserved -->
<!--                                                             Some(handler), // SVCall -->
<!--                                                             None, // Reserved for Debug -->
<!--                                                             None, // Reserved -->
<!--                                                             Some(handler), // PendSV -->
<!--                                                             Some(handler)]; // Systick -->
<!-- } -->
<!-- ``` -->

<!-- Let's go over each section added to our program: -->

<!-- ``` rust -->
<!-- mod exception { -->
<!--     pub extern "C" fn handler() { -->
<!--         unsafe { -->
<!--             asm!("bkpt"); -->
<!--         } -->

<!--         loop {} -->
<!--     } -->

<!--     .. -->
<!-- } -->
<!-- ``` -->

<!-- `exception::handler` is the handler (function) that we'll use to handle all the -->
<!-- exceptions. This handler uses the `bkpt` instruction to trigger a manual -->
<!-- breakpoint and then loops endlessly. We avoid returning from this function -->
<!-- because whatever triggered the exception may have corrupted the stack so it may -->
<!-- not be safe to continue executing the program. -->

<!-- ``` rust -->
<!-- mod exception { -->
<!--     .. -->

<!--     #[export_name = "_EXCEPTIONS"] -->
<!--     pub static EXCEPTIONS: [Option<extern "C" fn()>; 14] = [Some(handler), // NMI -->
<!--                                                             Some(handler), // Hard fault -->
<!--                                                             Some(handler), // Memmanage fault -->
<!--                                                             Some(handler), // Bus fault -->
<!--                                                             Some(handler), // Usage fault -->
<!--                                                             None, // Reserved -->
<!--                                                             None, // Reserved -->
<!--                                                             None, // Reserved -->
<!--                                                             None, // Reserved -->
<!--                                                             Some(handler), // SVCall -->
<!--                                                             None, // Reserved for Debug -->
<!--                                                             None, // Reserved -->
<!--                                                             Some(handler), // PendSV -->
<!--                                                             Some(handler)]; // Systick -->
<!-- } -->
<!-- ``` -->

<!-- On the second part, we create the *global* `_EXCEPTIONS` symbol backed by a -->
<!-- static `EXCEPTIONS` variable. The [documentation] states that there are 14 -->
<!-- exception "slots" right after the reset handler so we'll place a 14-element -->
<!-- array of function pointers in the `EXCEPTIONS` variable. Some of the exception -->
<!-- slots, the ones marked as "Reserved" in the docs, will never be used at runtime; -->
<!-- we'll leave those uninitialized (i.e. zeroed). In Rust, function pointers `fn()` -->
<!-- are non-nullable. To get nullable function pointers we must wrap the `fn()`s in -->
<!-- an `Option`, then the `None` variant becomes the equivalent of the null pointer. -->

<!-- ## Build and inspect -->

<!-- It's always a good idea to inspect the output binary to check that everything -->
<!-- looks as expected: -->

<!-- ``` -->
<!-- $ xargo build --target $TARGET -->

<!-- $ arm-none-eabi-objdump -CD target/$TARGET/debug/app -->

<!-- ./target/$TARGET/debug/app:     file format elf32-littlearm -->


<!-- Disassembly of section .text: -->

<!-- 08000000 <_EXCEPTIONS-0x8>: -->
<!--  8000000:       2000a000        .word   0x2000a000 -->
<!--  8000004:       08000041        .word   0x08000041 -->

<!-- 08000008 <_EXCEPTIONS>: -->
<!--  8000008:       08000053 08000053 08000053 08000053     S...S...S...S... -->
<!--  8000018:       08000053 00000000 00000000 00000000     S............... -->
<!--  8000028:       00000000 08000053 00000000 00000000     ....S........... -->
<!--  8000038:       08000053 08000053                       S...S... -->

<!-- 08000040 <_reset>: -->
<!--  8000040:       b084            sub     sp, #16 -->
<!--  8000042:       e7ff            b.n     8000044 <_reset+0x4> -->
<!--  8000044:       2000            movs    r0, #0 -->
<!--  8000046:       6800            ldr     r0, [r0, #0] -->
<!--  8000048:       9002            str     r0, [sp, #8] -->
<!--  800004a:       6800            ldr     r0, [r0, #0] -->
<!--  800004c:       9001            str     r0, [sp, #4] -->
<!--  800004e:       e7ff            b.n     8000050 <_reset+0x10> -->
<!--  8000050:       e7fe            b.n     8000050 <_reset+0x10> -->

<!-- 08000052 <app::exception::handler::he7207b3009cf1097>: -->
<!--  8000052:       b083            sub     sp, #12 -->
<!--  8000054:       e7ff            b.n     8000056 <app::exception::handler::he7207b3009cf1097+0x4> -->
<!--  8000056:       be00            bkpt    0x0000 -->
<!--  8000058:       e7ff            b.n     800005a <app::exception::handler::he7207b3009cf1097+0x8> -->
<!--  800005a:       e7fe            b.n     800005a <app::exception::handler::he7207b3009cf1097+0x8> -->
<!-- ``` -->

<!-- The two most relevant things to note: The `exception::handler` is located at -->
<!-- address `0x0800_0052`. And, `_EXCEPTIONS`, the part of the vector table where -->
<!-- the exception handlers reside, is mainly filled with the thumb address of -->
<!-- `exception::handler` i.e. `0x0800_0053`; which is what we wanted. You will also -->
<!-- see a few `0x0`s inside `_EXCEPTIONS`. These are the reserved exceptions slots -->
<!-- and they are basically zeroed/uninitialized chunks of memory. -->

<!-- ## Let's crash! -->

<!-- > **NOTE** This program won't work on QEMU. AFAICT, raising an exception on a -->
<!-- > emulated device is rather hard. For some reason (probably to simplify the -->
<!-- > implementation), actions that normally raise an exception on real hardware are -->
<!-- > allowed in QEMU. *shrugs* -->

<!-- Let's write a program that raises an exception to test the exception handler. -->
<!-- Our program will attempt to read memory beyond the RAM region. Here is it: -->

<!-- ``` rust -->
<!-- #[export_name = "_reset"] -->
<!-- pub extern "C" fn main() -> ! { -->
<!--     unsafe { -->
<!--         let sram_boundary = *(0x0000_0000 as *const u32); -->
<!--         let _crash = *(sram_boundary as *const u32); -->
<!--     } -->

<!--     loop {} -->
<!-- } -->
<!-- ``` -->

<!-- Let' run the program under the debugger on real hardware. -->

<!-- ``` -->
<!-- # On another terminal -->
<!-- $ [sudo] openocd (..) -->
<!-- ``` -->

<!-- ``` -->
<!-- $ arm-none-eabi-gdb target/cortex-m3/debug/app -->
<!-- (gdb) target remote :3333 -->

<!-- (gdb) load -->

<!-- (gdb) step -->
<!-- 9               let sram_boundary = *(0x0000_0000 as *const u32) -->

<!-- (gdb) step -->
<!-- 10              let _crash = *(ram_boundary as *const u32) -->

<!-- (gdb) print/x ram_boundary -->
<!-- $1 = 0x2000a000 -->

<!-- (gdb) step -->
<!-- app::exception::handler () at src/main.rs:19 -->
<!-- 19                  asm!("bkpt") -->
<!-- ``` -->

<!-- Remember that the address `0x0000_0000` is the first element of the vector table -->
<!-- and contains the highest RAM address. Dereferencing that address -->
<!-- (`sram_boundary`) will load memory beyond the RAM region. -->

<!-- Upon trying to read invalid memory, the processor raises *an* exception (we -->
<!-- don't know *which* one) and then proceeds to execute the `exception::handler`. -->
<!-- While executing the handler, the processor encounters the `bkpt` instruction, -->
<!-- halts and yields control back to the debugger. -->

<!-- We'll use this very simple exception handler to uncover programming mistakes -->
<!-- while debugging. We'll revisit the exception handler in the future to make it -->
<!-- tell us which exception was raised and which instruction (and line of code) -->
<!-- generated the exception. -->

<!-- ## Homework -->

<!-- What do you think would have happened if we executed the last `main` function -->
<!-- that reads memory beyond the RAM boundary **if** we didn't *install* the -->
<!-- exception handlers, or IOW if we didn't add the `_EXCEPTIONS` symbol or the -->
<!-- `EXCEPTIONS` variable to our program? -->

<!-- > **HINT** Look at the disassembly (`objdump`) of such binary. Take note of the -->
<!-- > addresses. What does the vector table look like? -->

<!-- > **WARNING** Don't actually run a program that raises a hardware exception but -->
<!-- > doesn't handle it! -->
