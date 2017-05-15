Thanks for your interest! This book is currently outdated so I'm taking it down
until I get time to update it; that way it won't cause more confusion. In the
meantime, you can read [this blog post] that covers the *easy*, high level way
of writing Rust applications for any ARM Cortex-M microcontroller.

[this blog post]: http://blog.japaric.io/quickstart/

<!-- # Target specification -->

<!-- In this section, we explain why we compiled our crate with `--target -->
<!-- thumbv7m-none-eabi`. -->

<!-- Building a Rust program for a Cortex-M microcontroller involves cross compiling. -->
<!-- Cross compilation is the process of building a binary that will run on a -->
<!-- **target** device/system that's different from the **host** device/system where -->
<!-- the binary was produced. In our case, the target is the Cortex-M microcontroller -->
<!-- and the host is your laptop or your PC. -->

<!-- Out of the box, `rustc`, and therefore Cargo, can cross compile for a bunch of -->
<!-- different targets using the `--target` flag. -->

<!-- Here's the list of supported cross compilation targets as of November of 2016 -->
<!-- (this list will continue growing over time): -->

<!-- ``` -->
<!-- $ rustc -V -->
<!-- rustc 1.14.0-nightly (cae6ab1c4 2016-11-05) -->

<!-- $ rustc --print target-list -->
<!-- aarch64-linux-android           mips64el-unknown-linux-gnuabi64 -->
<!-- aarch64-unknown-fuchsia         mipsel-unknown-linux-gnu -->
<!-- aarch64-unknown-linux-gnu       mipsel-unknown-linux-musl -->
<!-- arm-linux-androideabi           mipsel-unknown-linux-uclibc -->
<!-- arm-unknown-linux-gnueabi       powerpc-unknown-linux-gnu -->
<!-- arm-unknown-linux-gnueabihf     powerpc64-unknown-linux-gnu -->
<!-- arm-unknown-linux-musleabi      powerpc64le-unknown-linux-gnu -->
<!-- arm-unknown-linux-musleabihf    s390x-unknown-linux-gnu -->
<!-- armv7-linux-androideabi         thumbv6m-none-eabi -->
<!-- armv7-unknown-linux-gnueabihf   thumbv7em-none-eabi -->
<!-- armv7-unknown-linux-musleabihf  thumbv7em-none-eabihf -->
<!-- asmjs-unknown-emscripten        thumbv7m-none-eabi -->
<!-- i586-pc-windows-msvc            wasm32-unknown-emscripten -->
<!-- i586-unknown-linux-gnu          x86_64-apple-darwin -->
<!-- i686-apple-darwin               x86_64-pc-windows-gnu -->
<!-- i686-linux-android              x86_64-pc-windows-msvc -->
<!-- i686-pc-windows-gnu             x86_64-rumprun-netbsd -->
<!-- i686-pc-windows-msvc            x86_64-sun-solaris -->
<!-- i686-unknown-dragonfly          x86_64-unknown-bitrig -->
<!-- i686-unknown-freebsd            x86_64-unknown-dragonfly -->
<!-- i686-unknown-haiku              x86_64-unknown-freebsd -->
<!-- i686-unknown-linux-gnu          x86_64-unknown-fuchsia -->
<!-- i686-unknown-linux-musl         x86_64-unknown-haiku -->
<!-- le32-unknown-nacl               x86_64-unknown-linux-gnu -->
<!-- mips-unknown-linux-gnu          x86_64-unknown-linux-musl -->
<!-- mips-unknown-linux-musl         x86_64-unknown-netbsd -->
<!-- mips-unknown-linux-uclibc       x86_64-unknown-openbsd -->
<!-- mips64-unknown-linux-gnuabi64 -->
<!-- ``` -->

<!-- Luckily for us, since nightly-2016-10-05, `rustc` supports Cortex-M -->
<!-- microcontrollers as cross compilation targets. Furthermore, there is not 1 but -->
<!-- actually 4 different targets. Which one to pick depends on which microcontroller -->
<!-- you are targeting: -->

<!-- - `thumbv6m-none-eabi`. For Cortex-M0, Cortex-M0+ and Cortex-M1 micros. -->

<!-- - `thumbv7m-none-eabi`. For the Cortex-M3 micro. -->

<!-- - `thumbv7em-none-eabi`. For the Cortex-M4 and Cortex-M7 micros that *don't* -->
<!--   have a FPU. -->

<!-- - `thumbv7em-none-eabihf`. For the Cortex-M4 and Cortex-M7 micros that *have* a -->
<!--   FPU. -->

<!-- In the case of the LM3S6965, we want to use the `thumbv7m-none-eabi` target -->
<!-- because that microcontroller has a Cortex-M3 processor in it. -->
