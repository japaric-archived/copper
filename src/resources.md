Thanks for your interest! This book is currently outdated so I'm taking it down
until I get time to update it; that way it won't cause more confusion. In the
meantime, you can read [this blog post] that covers the *easy*, high level way
of writing Rust applications for any ARM Cortex-M microcontroller.

[this blog post]: http://blog.japaric.io/quickstart/

<!-- # Resources -->

<!-- ## Cortex-M -->

<!-- The ultimate source of information about this processor family is the [ARM info center][0]. -->

<!-- [0]: http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.set.cortexm/index.html -->

<!-- Some documents of interest: -->

<!-- - [Cortex-M0 Devices - Generic User Guide][0]. About the Cortex-M0 processor, its instruction set -->
<!--   and its peripherals. -->
<!-- - [Cortex-M0+ Devices - Generic User Guide][1]. Same as above but for the Cortex-M0+. -->
<!-- - [Cortex-M3 Devices - Generic User Guide][2]. Same as above but for the Cortex-M3. -->
<!-- - [Cortex-M4 Devices - Generic User Guide][3]. Same as above but for the Cortex-M4. -->
<!-- - [Cortex-M7 Devices - Generic User Guide][4]. Same as above but for the Cortex-M7. -->
<!-- - [Procedure Call Standard for the ARM Architecture][5]. About the machine registers, how -->
<!--   subroutines are called, how their parameters are passed and how their results are returned. -->
<!-- - [Lazy Stacking and Context Switching][6]. How the processor automatically decides when (not) to -->
<!--   save floating-point registers on the stack during interrupts. Important if implementing an OS, as -->
<!--   one should re-implement this functionality in their context switching logic to keep context -->
<!--   switching times small. -->

<!-- [0]: http://infocenter.arm.com/help/topic/com.arm.doc.dui0497a/DUI0497A_cortex_m0_r0p0_generic_ug.pdf -->
<!-- [1]: http://infocenter.arm.com/help/topic/com.arm.doc.dui0662b/DUI0662B_cortex_m0p_r0p1_dgug.pdf -->
<!-- [2]: http://infocenter.arm.com/help/topic/com.arm.doc.dui0552a/DUI0552A_cortex_m3_dgug.pdf -->
<!-- [3]: http://infocenter.arm.com/help/topic/com.arm.doc.dui0553a/DUI0553A_cortex_m4_dgug.pdf -->
<!-- [4]: http://infocenter.arm.com/help/topic/com.arm.doc.dui0646a/DUI0646A_cortex_m7_dgug.pdf -->
<!-- [5]: http://infocenter.arm.com/help/topic/com.arm.doc.ihi0042f/IHI0042F_aapcs.pdf -->
<!-- [6]: http://infocenter.arm.com/help/topic/com.arm.doc.dai0298a/DAI0298A_cortex_m4f_lazy_stacking_and_context_switching.pdf -->

<!-- ## LM3S6965EVB -->

<!-- This is the evaluation board we'll emulate using QEMU. It contains an LM3S6965 microcontroller. The -->
<!-- most important bits of documentation related to this board are: -->

<!-- - The [Microcontroller data sheet][6]. It contains the register maps and the descriptions of all the -->
<!--   device peripherals. -->
<!-- - The [Evaluation Board User manual][7]. Description of the physical board. -->

<!-- [6]: http://www.ti.com/lit/ds/symlink/lm3s6965.pdf -->
<!-- [7]: http://www.ti.com/lit/ug/spmu029a/spmu029a.pdf -->

<!-- ## Similar/related projects -->

<!-- ### `zinc` -->

<!-- > The bare metal stack for rust -->

<!-- [GitHub](https://github.com/hackndev/zinc) -->

<!-- The Zinc project was the pioneer in the embedded Rust space. I have borrowed a lot of ideas from -->
<!-- there (like building executables using only Cargo) to write this book. -->

<!-- ### Hanno Braun's blog -->

<!-- > "I'm teaching myself embedded programming, and I've decided to write about every step of the way" -->

<!-- [Website](http://embedded.hannobraun.de/) -->

<!-- [GitHub repository](https://github.com/hannobraun/embedded) -->

<!-- Hanno is also exploring this space but using an Arduino Due. Definitely worth checking out! -->

<!-- ### RustyGecko -->

<!-- > Developing Rust on Bare-Metal - An experimental embedded software platform -->

<!-- [Abstract and full text PDF](https://brage.bibsys.no/xmlui/handle/11250/2352353) -->

<!-- [GitHub repositories](https://github.com/RustyGecko) -->

<!-- A master thesis about using Rust to develop software for the Cortex-M3 based EFM32GG -->
<!-- microcontroller. I found the section about "Handling interrupts with Closures" and the discussion -->
<!-- about "Avoiding Mutable Aliases to Hardware" particularly interesting! The authors have also put -->
<!-- their code up on GitHub. -->
