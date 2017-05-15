Thanks for your interest! This book is currently outdated so I'm taking it down
until I get time to update it; that way it won't cause more confusion. In the
meantime, you can read [this blog post] that covers the *easy*, high level way
of writing Rust applications for any ARM Cortex-M microcontroller.

[this blog post]: http://blog.japaric.io/quickstart/

<!-- # Run the program under QEMU -->

<!-- Now that we have an executable in our hands, it's time to test it under an -->
<!-- emulator! You may be wondering "how are we going to do that?" since the program -->
<!-- doesn't do any I/O. Well, instead of expecting the program to output something -->
<!-- to the terminal (which won't happen), we are going to "hook" a debugger to the -->
<!-- emulator, execute the program "statement by statement" and verify that the -->
<!-- emulated memory changes as the program executes. Sounds fun? You bet it is. -->

<!-- Let's start! The first thing we have to do is load our binary in the emulator -->
<!-- with this command: -->

<!-- ``` -->
<!-- $ qemu-system-arm \ -->
<!--     -cpu cortex-m3 \ -->
<!--     -machine lm3s6965evb \ -->
<!--     -gdb tcp::3333 \ -->
<!--     -S \ -->
<!--     -nographic -monitor null \ -->
<!--     -serial null \ -->
<!--     -kernel target/thumbv7m-none-eabi/debug/app -->
<!-- ``` -->

<!-- So many arguments! Let's explain why all those are there for: -->

<!-- - `qemu-system-arm` this is a QEMU variant that can emulate an ARM processor in -->
<!--   system mode emulation. -->

<!-- - `-machine lm3s6965evb` this is the dev board we are going to emulate: the -->
<!--   [LM3S6965EVB]. -->

<!-- - `-cpu cortex-m3` this is the CPU to emulate, it must match the CPU of the -->
<!--   emulated `machine`. -->

<!-- - `-gdb tcp::3333` tells the emulator to expect a gdb connection on port `3333`, -->
<!--   we need this to control the execution of the emulated program under gdb. -->

<!-- - `-S` "do not immediately start the CPU". This tells the emulator to load the -->
<!--   program but don't immediately execute it, otherwise by the time you attach -->
<!--   `gdb` your program may have already terminated! -->

<!-- - `-nographic`, `-monitor null` we don't need anything graphic related -->

<!-- - `-serial null` we are not going to use the serial console this time -->

<!-- - `-kernel target/thumbv7m-none-eabi/debug/app` use our binary directly as the -->
<!--   "kernel" which is the first thing the emulator executes. -->

<!-- [LM3S6965EVB]: http://www.ti.com/lit/ug/spmu029a/spmu029a.pdf -->

<!-- This command will block; just leave it running for now. -->

<!-- Next we hook a debugger to the emulator we just started. In another terminal, -->
<!-- type: -->

<!-- ``` -->
<!-- $ arm-none-eabi-gdb -q target/thumbv7m-none-eabi/debug/app -->
<!-- ``` -->

<!-- > **NOTE** You can use `lldb` instead of `gdb` but you won't be able to use the -->
<!-- > same commands I have used here, because `lldb` uses different commands to -->
<!-- > expose the same functionality as `gdb`. [This page] will help you map `gdb` -->
<!-- > commands to `lldb`'s and vice versa. -->

<!-- [This page]: http://lldb.llvm.org/lldb-gdb.html -->

<!-- Under this `gdb` session, enter the following command to connect to the -->
<!-- emulator: -->

<!-- ``` -->
<!-- (gdb) target remote :3333 -->
<!-- ``` -->

<!-- You should see an output like this: -->

<!-- ``` -->
<!-- Remote debugging using :3333 -->
<!-- app::main () at $PWD/src/main.rs:6 -->
<!-- 6       pub extern "C" fn main() -> ! { -->
<!-- ``` -->

<!-- The emulator is *halted* and currently at the program entry point: `main`. You -->
<!-- can now execute the program statement by statement using the `step` command: -->

<!-- ``` -->
<!-- (gdb) step -->
<!-- 8           let x = 42 -->
<!-- (gdb) step -->
<!-- 9           y = x -->
<!-- ``` -->

<!-- At this point the statement `let x = 42` has been executed but the statement -->
<!-- `y = x` has not, so `x` is initialized but `y` is not. Let's inspect both -->
<!-- variables by `print`ing their addresses and values. -->

<!-- ``` -->
<!-- (gdb) print x -->
<!-- $1 = 42 -->
<!-- (gdb) print &x -->
<!-- $2 = (i32 *) 0x2000fff8 -->
<!-- (gdb) print y -->
<!-- $3 = 0 -->
<!-- (gdb) print &y -->
<!-- $4 = (i32 *) 0x2000fffc -->
<!-- ``` -->

<!-- A few things to note: -->

<!-- - Both `x` and `y` live in the "stack". That's why they have contiguous -->
<!--   addresses. -->

<!-- - `y`, which was declared before `x`, has a larger address than `x`. The reason -->
<!--   is that the stack grows downwards (toward smaller addresses). If you keep -->
<!--   creating stack variables, you'll see their addresses get smaller and smaller. -->

<!-- - `y` which is currently uninitialized holds the value `0` -- this is a QEMU -->
<!--   simplification. On real hardware you will observe that uninitialized variables -->
<!--   hold random values. Of course, (safe) Rust won't actually let you *use* -->
<!--   uninitialized variables but you can peek at them using `gdb`. -->

<!-- Back to the debugger. If you step again, you should see that `y` is now -->
<!-- initialized: -->

<!-- ``` -->
<!-- (gdb) step -->
<!-- 11          loop {} -->
<!-- (gdb) print y -->
<!-- $5 = 42 -->
<!-- ``` -->

<!-- The emulator is about to execute an endless loop. If you call `step` again, -->
<!-- `gdb` will get stuck in the loop and hang. Instead, call `stepi` to advance *one -->
<!-- instruction* rather than one statement. -->

<!-- ``` -->
<!-- (gdb) stepi -->
<!-- 0x00000014      10          loop {} -->
<!-- (gdb) stepi -->
<!-- 0x00000014      10          loop {} -->
<!-- ``` -->

<!-- Congrats, you are now stuck in an endless loop! -->

<!-- There is not much left to do in this emulation. But, before you terminate the -->
<!-- `gdb` session and exit the emulator ... -->

<!-- ## Homework -->

<!-- `gdb` has an ["examine"] command that let's you inspect the contents of memory -->
<!-- at a certain address. Try the following command: -->

<!-- ["examine"]: http://www.delorie.com/gnu/docs/gdb/gdb_56.html -->

<!-- ``` -->
<!-- (gdb) x/4x main -->
<!-- ``` -->

<!-- Compare the output of that command with the output of the command: -->
<!-- `arm-none-eabi-objdump -Cd target/thumbv7m-none-eabi/debug/app`. Are the outputs -->
<!-- related somehow? Elaborate. -->
