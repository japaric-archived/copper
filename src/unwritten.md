Thanks for your interest! This book is currently outdated so I'm taking it down
until I get time to update it; that way it won't cause more confusion. In the
meantime, you can read [this blog post] that covers the *easy*, high level way
of writing Rust applications for any ARM Cortex-M microcontroller.

[this blog post]: http://blog.japaric.io/quickstart/

<!-- # Unwritten topics -->

<!-- - More debugging patterns: breakpoints, watchpoints, `bkpt`, etc. -->
<!-- - The `.data` and `.bss` sections: `static` variables, linker script (again) and initialization -->
<!--   routine. -->
<!-- - Guaranteed `panic!`-free programs with this one weird trick! -->
<!-- - The Rust allocator interface. -->
<!-- - Debug prints with ITM (Instrumentation Trace Macrocell) -->
<!-- - "Apps" -->
<!--   - Heartbeat LED -->
<!--   - Read an analog sensor. e.g. just potentiometer, a photoresistor or a CNY70. -->
<!--   - Read a digital sensor via SPI or I2C. e.g. motion sensor. -->
<!--   - Data acquisition. e.g. sensor + serial comm -->
<!--   - Move a servomotor -->
<!--   - Open loop control system. e.g. serial comm + servo or potentiometer + servo -->
<!--   - Closed loop control system. e.g. servo + magnetometer -->
<!-- - Interfacing external devices -->
<!--   - Buttons and debouncing. -->
<!--   - Servomotors. -->
<!-- - Async and concurrency -->
<!--   - Scheduling strategies: Super loop, cooperative, preemptive. -->
<!--   - Interrupts: `asm!("cpsid i")`, `asm!("cpsie i")`, data races and atomic operations. -->
<!--   - DMA -->
<!-- - FPU -->
<!-- - `nm -C --size-sort`: binary size profiling -->
<!-- - `nm -C`: global vs local text symbol -->
<!-- - librarify: src/lib.rs + src/bin/*.rs. Uses `extern` (lib) and `no_mangle` (bin) symbol. Basically -->
<!--   the library says: you (the binary) must give me a symbol with this *unmangled* name. Downside: -->
<!--   types signatures are lost. How to recover them? traits? Related: weak symbols for e.g. exceptions. -->
<!-- - ARM calling convention: -->
<!--   - cf. http://infocenter.arm.com/help/topic/com.arm.doc.ihi0042f/IHI0042F_aapcs.pdf -->
<!--   - A diagram of how the stack, the heap and static variables (.data, .bss) are allocated in RAM. -->
<!--   - An explanation of how the call stack works: stack pointer, stack frames, what happens when a -->
<!--     function/interrupt is called, etc. -->
<!--   - How arguments are passed to functions: via stack, registers -->
<!-- - mention that a section can't exceed its region capacity. This is automatically enforced by the -->
<!--   linker based on what the linker script says. -->
<!-- - bluetooth: HC-06. `rfcomm bind rfcomm0 $ADDRESS` -->
<!-- - document other flashing methods: ELF -> `objcopy` -> binary, then directly flash the binary using -->
<!--   OpenOCD (without `gdb`) or st-link. -->
<!-- - revisit the default exception handler: where did we come from? (inspect stack) which exception is -->
<!--   this? (inspect NVIC(?) register). -->
<!--   - cf. http://www.freertos.org/Debugging-Hard-Faults-On-Cortex-M-Microcontrollers.html -->
<!-- - embedding Rust in C frameworks like FreeRTOS or the Photon. -->
<!--   - cf. https://github.com/japaric/photon -->
