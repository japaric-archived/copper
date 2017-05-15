Thanks for your interest! This book is currently outdated so I'm taking it down
until I get time to update it; that way it won't cause more confusion. In the
meantime, you can read [this blog post] that covers the *easy*, high level way
of writing Rust applications for any ARM Cortex-M microcontroller.

[this blog post]: http://blog.japaric.io/quickstart/

<!-- # Why the ARM Cortex-M? -->

<!-- > Why not AVR or PIC or ...? -->

<!-- ARM Cortex-M processors are very popular and are used across many industries. In a way they provide -->
<!-- unprecedented performance in a variety of cores from the low power Cortex-M0 to the floating-point -->
<!-- enabled Cortex-M4F. They are used in everything from your [Fitbit][0], to your [Quadcopter][1], and -->
<!-- beyond to medical equipment, robotic arms, and automobiles. Also, ARM Cortex-M processors are -->
<!-- developed by many companies, all using the standard core specified by ARM. This gives embedded -->
<!-- systems developers a wide range of choice of manufacturer, and feature set. Also, going forward the -->
<!-- relatively low cost of these 32-bit processors may make 8-bit processors less competitive in both an -->
<!-- industrial and hobbyist perspective. Even Arduino is moving away from the AVR platform with the -->
<!-- [Arduino Zero][2]. -->

<!-- Also not many microcontroller families are currently supported by Rust, which uses LLVM as its -->
<!-- backend. Popular microcontrollers like AVR, and PIC are not (officially) supported by LLVM -->
<!-- and as the Rust compiler relies on LLVM for code generation it doesn't support them either. However, -->
<!-- an [unofficial AVR backend][3] for LLVM is currently undergoing the process of being merged back -->
<!-- into upstream LLVM, so this situation will likely improve in the near future. -->

<!-- __TODO__: might be interesting to address the new Intel Quark platforms. -->

<!-- [0]: https://www.arm.com/markets/embedded/fitbit-one.php -->
<!-- [1]: http://diydrones.com/profiles/blogs/vr-multipilot32f4-arm-cortex-m4-the-most-powerful-auto-pilot-in -->
<!-- [2]: https://www.arduino.cc/en/Main/ArduinoBoardZero -->
<!-- [3]: https://github.com/avr-llvm/llvm -->

<!-- Do note that some of the concepts covered here like the tooling and how to use common peripherals -->
<!-- are transferable to other microcontroller families. -->
