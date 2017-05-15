Thanks for your interest! This book is currently outdated so I'm taking it down
until I get time to update it; that way it won't cause more confusion. In the
meantime, you can read [this blog post] that covers the *easy*, high level way
of writing Rust applications for any ARM Cortex-M microcontroller.

[this blog post]: http://blog.japaric.io/quickstart/

<!-- # Boot process -->

<!-- In this section, we'll learn what the microcontroller does right after it's -->
<!-- powered. -->

<!-- ## Memory -->

<!-- But before that, we have to talk a little about the different types of memory -->
<!-- available in a microcontroller. Cortex-M microcontrollers have at least two -->
<!-- different types of memory available to them: Flash memory and Random Access -->
<!-- Memory (RAM). -->

<!-- Flash memory is non-volatile and it's used to store the `text` section of our -->
<!-- program, i.e. functions and constants. Because this memory is non-volatile, our -->
<!-- program will persist in memory even after the microcontroller is powered off. -->

<!-- RAM, on the other hand, is volatile and it's used to store the [call stack], the -->
<!-- [heap] and static variables. Because RAM is volatile, its contents are lost when -->
<!-- the microcontroller is powered off. Also, when the microcontroller have just -->
<!-- been powered on, its RAM is filled with random values. -->

<!-- [call stack]: https://en.wikipedia.org/wiki/Call_stack -->
<!-- [heap]: https://en.wikipedia.org/wiki/Memory_management#HEAP -->

<!-- These two different memories can be accessed by the processor through the same -->
<!-- 32 bit address space. For instance, the LM3S6965 has the following memory -->
<!-- specifications: -->

<!-- - 256 KiB of flash memory. The flash memory region starts at address `0x0` and -->
<!--   ends at address `0x40000`. -->
<!-- - 64 KiB of RAM. The RAM region starts at address `0x2000_0000` and ends at address `0x2001_0000`. -->

<!-- ### Vector table -->

<!-- On Cortex-M microcontrollers, the beginning of the flash memory holds a data -->
<!-- structure known as the "vector table". The values stored in the vector table are -->
<!-- used in different hardware processes like the boot process. -->

<!-- The vector table is, effectively, an array of pointers and each of its elements -->
<!-- is used for a different purpose. The vector table is fully documented [here]. -->
<!-- But, right now, we are only interested in its first two elements: -->

<!-- [here]: http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0552a/BABIFJFG.html -->

<!-- 1. At `0x0`: the initial value of the stack pointer. -->

<!-- The stack pointer is a register used to track the top of the call stack. The -->
<!-- initial value determines where in memory the call stack will be initialized. As -->
<!-- per the [AAPCS][AAPCS] (ARM Architecture Procedure Call Standard), the call -->
<!-- stack grows downwards (towards smaller addresses). That's why the initial value -->
<!-- of the stack pointer is usually set to the largest valid address of the RAM -->
<!-- region. -->

<!-- [AAPCS]: http://infocenter.arm.com/help/topic/com.arm.doc.ihi0042f/IHI0042F_aapcs.pdf#page=16&zoom=auto,52,151 -->

<!-- 2. At `0x4`: the reset vector -->

<!-- A "vector" is a pointer to a "handler" and a "handler" is just another name for -->
<!-- a function. Therefore, the reset vector is a (function) pointer to the reset -->
<!-- handler. The reset handler gets called, through the reset vector, whenever a -->
<!-- reset occurs and during the boot process. -->

<!-- ## Putting everything together -->

<!-- Here's what happens during the boot process or whenever the microcontroller is -->
<!-- reset: -->

<!-- - `SP = *(0x0 as *const u32)`. The stack pointer, which is a (CPU) register, -->
<!--   is initialized to the value stored at address `0x0`. -->

<!-- - `mem::transmute::<_, fn()>(0x4)()`. The reset handler gets called through the -->
<!--   reset vector. -->

<!-- ## The takeaway -->

<!-- What you should remember from all this is that the programs you write for -->
<!-- Cortex-M microcontrollers must comply with a **specific memory layout**. In -->
<!-- particular, the memory section at address `0x0`, the vector table, must be -->
<!-- properly initialized or your microcontroller won't boot! In the next section, -->
<!-- we'll go over the boot process again using the LM3S6965 as an example. -->
