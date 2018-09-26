Thanks for your interest! This book has been deprecated in favor of [the
embedded Rust book][book]. Also check the rest of the [embedded WG's][wg]
[bookshelf].

[book]: https://rust-embedded.github.io/bookshelf/book/index.html
[wg]: https://github.com/rust-embedded/wg
[bookshelf]: https://rust-embedded.github.io/bookshelf/

<!-- # Run it on real hardware -->

<!-- > **TODO** Document gotchas. Some dev boards may need to plug/unplug jumpers to -->
<!-- > enable the built-in > programmer/debugger. -->

<!-- It's time to test our program on real hardware! We'll use OpenOCD  to *flash* -->
<!-- our program into the microcontroller and then hook `gdb` and OpenOCD to debug -->
<!-- our program just like we did before. -->

<!-- > Wait, what's flashing? -->

<!-- *Flashing* means we are going to transfer our program from the host machine (PC, -->
<!-- laptop, etc.) to the target device (the microcontroller). Once flashed, the -->
<!-- micro will execute the flashed program every time it boots or resets. -->

<!-- > **ATTENTION!** The flashing instructions here will overwrite the program -->
<!-- > that's currently stored in your microcontroller. Make sure that it's either -->
<!-- > something you are not gonna miss or that's something you can easily get a copy of. -->

<!-- ## Compile for a different target device -->

<!-- Unless you happen to have a real LM3S6965EVB board right next to you, the binary -->
<!-- we produced in the previous section won't work for your device. To produce a -->
<!-- valid binary for your device, you'll have to change the cross compilation target -->
<!-- from the LM3S6965 to *your* device and then rebuild the program. -->

<!-- For the rest of this section, I'll be using the [STM32F3DISCOVERY] as the target -->
<!-- device. -->

<!-- [STM32F3DISCOVERY]: http://www.st.com/en/evaluation-tools/stm32f3discovery.html -->

<!-- ### Update the linker script -->

<!-- One of the things that you'll always have to do when changing the cross -->
<!-- compilation target is to update the device-specific parts of the linker script. -->

<!-- The STM32F3DISCOVERY contains a [STM32F303VCT6] micro with 256 KiB of flash and -->
<!-- 40+8 (\*) KiB of RAM. A peculiarity of STM32 devices is that their flash memory starts -->
<!-- at address `0x0800_0000`. -->

<!-- (\*) RAM is split in two memory regions located at different addresses. -->

<!-- [STM32F303VCT6]: http://www.st.com/resource/en/datasheet/stm32f303vc.pdf -->

<!-- *My* linker script adjustments look like this (yours will probably look -->
<!-- different): -->

<!-- ``` diff -->
<!--  MEMORY -->
<!--  { -->
<!-- -  FLASH : ORIGIN = 0x00000000, LENGTH = 256K -->
<!-- -  RAM : ORIGIN = 0x20000000, LENGTH = 64K -->
<!-- +  CCRAM : ORIGIN = 0x10000000, LENGTH = 8K -->
<!-- +  FLASH : ORIGIN = 0x08000000, LENGTH = 256K -->
<!-- +  RAM : ORIGIN = 0x20000000, LENGTH = 40K -->
<!--  } -->
<!-- ``` -->

<!-- ### Change the target specification -->

<!-- Depending on the micro in your dev board, you may also have to change the -->
<!-- `rustc` target from `thumbv7m-none-eabi` to something else. In my case, the -->
<!-- STM32F303VCT6 contains a Cortex-M4F processor with FPU so I need to switch to the -->
<!-- `thumbv7em-none-eabihf` target. -->

<!-- If you change the target, you'll also need to update the `.cargo/config` file to -->
<!-- port the configuration to the new target: -->

<!-- ``` diff -->
<!-- -[target.thumbv7m-none-eabi] -->
<!-- +[target.thumbv7em-none-eabihf] -->
<!--  rustflags = [ -->
<!--      "-C", "link-arg=-Tlayout.ld", -->
<!--      "-C", "link-arg=-nostartfiles", -->
<!--  ] -->
<!-- ``` -->

<!-- ## Build & inspect -->

<!-- Now that the changes have been committed, we can rebuild the program: -->

<!-- ``` -->
<!-- $ xargo build --target $TARGET -->
<!-- ``` -->

<!-- > **NOTE** From now on, I'm going to use `$TARGET`, instead of e.g. -->
<!-- >  `thumbv7em-none-eabihf`, as a placeholder for the cross compilation target to -->
<!-- >  make sure you don't use the wrong target triple. -->

<!-- <\!-- FIXME use build.rs to force a rebuild when the linker script has been -\-> -->
<!-- <\!-- modified -\-> -->

<!-- > **HEADS UP** Make sure that Cargo actually rebuilds the binary! Cargo doesn't -->
<!-- > trigger a rebuild when the linker script changes. So, if the only thing you -->
<!-- > changed was the linker script and not the `rustc` target, then you'll have to -->
<!-- > `cargo clean` first and then call `build`. -->

<!-- As usual, it's a good idea to inspect the binary with `objdump` to verify that -->
<!-- the vector table is where expected. -->

<!-- ``` -->
<!-- $ arm-none-eabi-objdump -Cd ./target/$TARGET/debug/app -->

<!-- ./target/$TARGET/debug/app:     file format elf32-littlearm -->


<!-- Disassembly of section .text: -->

<!-- 08000000 <_reset-0x8>: -->
<!--  8000000:       2000a000        .word   0x2000a000 -->
<!--  8000004:       08000009        .word   0x08000009 -->

<!-- 08000008 <_reset>: -->
<!--  8000008:       b083            sub     sp, #12 -->
<!--  800000a:       e7ff            b.n     800000c <_reset+0x4> -->
<!--  800000c:       202a            movs    r0, #42 ; 0x2a -->
<!--  800000e:       9001            str     r0, [sp, #4] -->
<!--  8000010:       9002            str     r0, [sp, #8] -->
<!--  8000012:       e7ff            b.n     8000014 <_reset+0xc> -->
<!--  8000014:       e7fe            b.n     8000014 <_reset+0xc> -->
<!-- ``` -->

<!-- Looks good! The vector table is at `0x0800_0000` as expected for my device. -->

<!-- ## Establishing an OpenOCD connection -->

<!-- Before we flash the program we have to "open" an OpenOCD connection between the -->
<!-- device and the host machine. You should be already familiar with these steps -->
<!-- from the [development environment][de] chapter: -->

<!-- [de]: linux.html#First%20OpenOCD%20connection -->

<!-- ``` -->
<!-- # Physically connect the dev board to the host machine (probably your laptop), then -->
<!-- $ [sudo] openocd -f board/$BOARD -->
<!-- (...) -->
<!-- Info : Unable to match requested speed 1000 kHz, using 950 kHz -->
<!-- Info : Unable to match requested speed 1000 kHz, using 950 kHz -->
<!-- Info : clock speed 950 kHz -->
<!-- Info : STLINK v2 JTAG v27 API v2 SWIM v15 VID 0x0483 PID 0x374B -->
<!-- Info : using stlink api v2 -->
<!-- Info : Target voltage: 2.919073 -->
<!-- Info : stm32f3x.cpu: hardware has 6 breakpoints, 4 watchpoints -->
<!-- ``` -->

<!-- ## Flash and debug -->

<!-- We'll use the `gdb` shell to both flash and debug the program. So, fire up -->
<!-- `gdb`: -->

<!-- > **NOTE** You can't use `lldb` this time `:-(`. AFAICT, there's no `lldb` -->
<!-- > equivalent to the `monitor` and `load` commands provided by `gdb`. And those -->
<!-- > commands are required in this section. -->

<!-- ``` -->
<!-- $ arm-none-eabi-gdb target/$TARGET/debug/app -->
<!-- ``` -->

<!-- Next, we need to connect `gdb` and OpenOCD. The command is the same as the one -->
<!-- we used with QEMU: -->

<!-- ``` -->
<!-- (gdb) target remote :3333 -->
<!-- Remote debugging using :3333 -->
<!-- 0x00000000 in ?? () -->
<!-- ``` -->

<!-- You should also see extra output on the OpenOCD terminal but maybe not the exact -->
<!-- same output shown here: -->

<!-- ``` diff -->
<!--  Info : stm32f1x.cpu: hardware has 6 breakpoints, 4 watchpoints -->
<!-- +Info : accepting 'gdb' connection on tcp/3333 -->
<!-- +Info : device id = 0x10036422 -->
<!-- +Info : flash size = 256kbytes -->
<!-- ``` -->

<!-- Now that we are connected to the device via `gdb`. Let's flash the program using -->
<!-- the `load` command: -->

<!-- ``` -->
<!-- (gdb) load -->
<!-- Loading section .text, size 0x16 lma 0x8000000 -->
<!-- Start address 0x8000008, load size 20 -->
<!-- Transfer rate: 246 bytes/sec, 10 bytes/write. -->
<!-- ``` -->

<!-- You should also see new output on the OpenOCD terminal: -->

<!-- ``` diff -->
<!--  Info : accepting 'gdb' connection on tcp/3333 -->
<!--  Info : device id = 0x10036422 -->
<!--  Info : flash size = 256kbytes -->
<!-- +Info : Unable to match requested speed 1000 kHz, using 950 kHz -->
<!-- +Info : Unable to match requested speed 1000 kHz, using 950 kHz -->
<!-- +adapter speed: 950 kHz -->
<!-- +target state: halted -->
<!-- +target halted due to debug-request, current mode: Thread -->
<!-- +xPSR: 0x01000000 pc: 0x08000194 msp: 0x2000a000 -->
<!-- +Info : Unable to match requested speed 8000 kHz, using 4000 kHz -->
<!-- +Info : Unable to match requested speed 8000 kHz, using 4000 kHz -->
<!-- +adapter speed: 4000 kHz -->
<!-- +target state: halted -->
<!-- +target halted due to breakpoint, current mode: Thread -->
<!-- +xPSR: 0x61000000 pc: 0x2000003a msp: 0x2000a000 -->
<!-- +Info : Unable to match requested speed 1000 kHz, using 950 kHz -->
<!-- +Info : Unable to match requested speed 1000 kHz, using 950 kHz -->
<!-- +adapter speed: 950 kHz -->
<!-- +target state: halted -->
<!-- +target halted due to debug-request, current mode: Thread -->
<!-- +xPSR: 0x01000000 pc: 0x08000194 msp: 0x2000a000 -->
<!-- ``` -->

<!-- The program is now flashed and the device is halted at the program's entry -->
<!-- point, i.e. the `main` function. Let's repeat the debug session we used for the -->
<!-- previous QEMU run: -->

<!-- ``` -->
<!-- (gdb) step -->
<!-- 8           let x = 42 -->

<!-- (gdb) step -->
<!-- 9           y = x -->

<!-- (gdb) print x -->
<!-- $1 = 42 -->

<!-- (gdb) print &x -->
<!-- $2 = (i32 *) 0x20001ffc -->

<!-- (gdb) print y -->
<!-- $3 = -2052926870 -->

<!-- (gdb) print/x y -->
<!-- $4 = 0x85a2d26a -->

<!-- (gdb) print &y -->
<!-- $5 = (i32 *) 0x20001ff8 -->

<!-- (gdb) step -->
<!-- 11          loop {} -->

<!-- (gdb) print y -->
<!-- $5 = 42 -->
<!-- ``` -->

<!-- Yay! This time the uninitialized value of `y` looks more random; I got -->
<!-- `-2052926870` on this run. -->

<!-- While you executed the above commands, you should have seen more output on the -->
<!-- OpenOCD terminal. Each time you stepped over the program, OpenOCD printed the -->
<!-- *program counter* which is the address of the instruction the processor will -->
<!-- execute next. -->

<!-- ``` diff -->
<!--  xPSR: 0x01000000 pc: 0x08000008 msp: 0x20002000 -->
<!-- +Info : halted: PC: 0x0800000a -->
<!-- +Info : halted: PC: 0x0800000c -->
<!-- +Info : halted: PC: 0x0800000e -->
<!-- +Info : halted: PC: 0x08000010 -->
<!-- ``` -->

<!-- Here's one more trick for you to try: -->

<!-- ``` -->
<!-- (gdb) monitor reset halt -->
<!-- ``` -->

<!-- This will generate the following OpenOCD output: -->

<!-- ``` -->
<!--  Info : halted: PC: 0x08000010 -->
<!-- +target state: halted -->
<!-- +target halted due to debug-request, current mode: Thread -->
<!-- +xPSR: 0x01000000 pc: 0x08000008 msp: 0x20002000 -->
<!-- ``` -->

<!-- This will reset your microcontroller (!) and halt your program at the reset -->
<!-- handler, i.e. the `main` function. -->

<!-- For fun, let's inspect the `y` variable **before** it's initialized: -->

<!-- ``` -->
<!-- (gdb) step -->
<!-- 8           let x = 42 -->

<!-- (gdb) step -->
<!-- 9           y = x -->

<!-- (gdb) print x -->
<!-- $6 = 42 -->

<!-- (gdb) print &x -->
<!-- $7 = (i32 *) 0x20001ffc -->

<!-- (gdb) print y -->
<!-- $8 = 42 -->

<!-- (gdb) print &y -->
<!-- $9 = (i32 *) 0x20001ff8 -->
<!-- ``` -->

<!-- Surprise! `y` appears to have been already initialized! Except, that's not the -->
<!-- case. What actually happened is that resetting the microcontroller doesn't touch -->
<!-- RAM (i.e. it doesn't power it off). Therefore, the RAM still holds the data from -->
<!-- the previous run and that's why `y` hold the value `42` it was assigned during -->
<!-- the *previous* run of the program. -->

<!-- That's all for this section! You can now close both OpenOCD and `gdb`. Let's -->
<!-- move onto more complex programs! -->
