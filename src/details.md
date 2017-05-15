Thanks for your interest! This book is currently outdated so I'm taking it down
until I get time to update it; that way it won't cause more confusion. In the
meantime, you can read [this blog post] that covers the *easy*, high level way
of writing Rust applications for any ARM Cortex-M microcontroller.

[this blog post]: http://blog.japaric.io/quickstart/

<!-- # Nitty-gritty details -->

<!-- > My program works! I have no idea why ... -->

<!-- Great! We wrote a program that worked in the emulator. But I omitted some -->
<!-- important details to simplify things, so we could focus our attention on the -->
<!-- tooling. In particular, I didn't tell you that our Cargo project generates -->
<!-- binaries that only work on the LM3S6965! It's time to take a step back and -->
<!-- understand all the pieces involved. In particular: -->

<!-- - What's the role of that extra file, `layout.ld`, that we added to the Cargo -->
<!--   project? -->
<!-- - Why we used `thumbv7m-none-eabi` as the cross compilation target? -->
<!-- - Why we had to add a `.cargo/config` to our Cargo project? -->

<!-- Once you understand that, you'll be able to write programs for **any** Cortex-M -->
<!-- microcontroller. -->
