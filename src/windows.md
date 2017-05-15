Thanks for your interest! This book is currently outdated so I'm taking it down
until I get time to update it; that way it won't cause more confusion. In the
meantime, you can read [this blog post] that covers the *easy*, high level way
of writing Rust applications for any ARM Cortex-M microcontroller.

[this blog post]: http://blog.japaric.io/quickstart/

<!-- # Windows -->

<!-- There's no default package manager in Windows so you'll have to scavenge the -->
<!-- tools from different locations: -->

<!-- ## Cross toolchain and Co. -->

<!-- The C cross compiler, the binary inspection tools and the debugger can all be -->
<!-- installed from a [single place]. Simply grab the latest `.exe` installer, run it -->
<!-- and follow the instructions. Before finishing the installation process, make -->
<!-- sure to check the "Add path to environment variable" option. -->

<!-- [single place]: https://launchpad.net/gcc-arm-embedded/+download -->

<!-- Check that the tools are really in your `PATH` -->

<!-- ``` -->
<!-- $ arm-none-eabi-gcc -v -->
<!-- (..) -->
<!-- gcc version 5.4.1 20160609 (..) -->

<!-- $ arm-none-eabi-size -v -->
<!-- GNU size <GNU Tools for ARM Embedded Processors> 2.26.0.20160616 -->
<!-- (..) -->

<!-- $ arm-none-eabi-gdb -v -->
<!-- GNU gdb <GNU Tools for ARM Embedded Processors> 7.10.1.20160616-cvs -->
<!-- (..) -->
<!-- ``` -->

<!-- ## OpenOCD -->

<!-- There are no official binary releases of OpenOCD for Windows, but [this page] -->
<!-- has unofficial releases. There is not installer so grab the latest zipfile and -->
<!-- unpack somewhere in your main drive. Then update your `PATH` variable by -->
<!-- appending `;Z:\directory\where\you\extracted\the\zipfile\bin` to it. This should -->
<!-- make the `openocd` command available anywhere: -->

<!-- [this page]: http://gnutoolchains.com/arm-eabi/openocd/ -->

<!-- ``` -->
<!-- $ echo %path% -->
<!-- (..);C:\OpenOCD-0.9.0-Win32\bin -->

<!-- $ openocd -v -->
<!-- Open On-Chip Debugger 0.9.0 <2015-08-15-12:41> -->
<!-- (..) -->
<!-- ``` -->

<!-- After installing OpenOCD, be sure to test it using [these instructions]. The -->
<!-- instructions are for Linux but they pretty much hold for Windows: just ignore -->
<!-- the `sudo` part of the commands used there. -->

<!-- [these instructions]: linux.html#First%20OpenOCD%20connection -->

<!-- There are some caveats with using OpenOCD on Windows: -->

<!-- - OpenOCD script search path will be empty. Commands like `openocd -f -->
<!--   board/stm32vldiscovery.cfg` will fail to find the `.cfg` file. This can be -->
<!--   remedied by passing an extra argument to the command: `openocd -s -->
<!--   Z:\path\to\openocd\share\openocd\scripts -f board/stm32vldiscovery.cfg`. -->

<!-- > **TODO** There must be a better solution to the script search path problem -->

<!-- - Some (hardware) programmers/debuggers (like the ST-LINK/V1) will try to use -->
<!--   the "wrong" driver by default and the `openocd` command will always fail with -->
<!--   `LIBUSB_ERROR_NOT_SUPPORTED`. You'll have to switch their driver to the WinUSB -->
<!--   driver using Zadig (see the following section for more information). -->

<!-- ## (Optional) Zadig -->

<!-- If you are getting the `LIBUSB_ERROR_NOT_SUPPORTED` error when trying to connect -->
<!-- to your programmer/debugger using OpenOCD, use Zadig to override the USB driver -->
<!-- that Windows picked: -->

<!-- - First, install Zadig from [its website]. -->

<!-- - Make sure your device is connected to the host machine! -->

<!-- - Then launch Zadig and choose the menu item: Options > List all devices. -->

<!-- - The drop-down list should now be populated; from it, pick your device (e.g. -->
<!--   STM32 STlink). -->

<!-- - Pick WinUSB as the target driver, click the "Replace Driver" button and follow -->
<!--   the instructions that pop up. -->

<!-- [its website]: http://zadig.akeo.ie/ -->

<!-- Try the `openocd` command again; it should succeed this time. -->

<!-- ## QEMU -->

<!-- You can find unofficial binaries [here]. Install a recent version and add -->
<!-- `;Z:\Program Files\qemu` to your `PATH`. -->

<!-- [here]: https://qemu.weilnetz.de/ -->

<!-- Test it with: -->

<!-- ``` -->
<!-- $ qemu-system-arm -version -->
<!-- ``` -->

<!-- The command won't print to the console but instead redirect its output to the -->
<!-- file `Z:\Program Files\qemu\stdout`. It should contain something like this: -->

<!-- ``` -->
<!-- $ type Z:\Program Files\qemu\stdout.txt -->
<!-- ``` -->

<!-- ## Rust and Cargo -->

<!-- Head to the [rustup] website and follow the instructions. -->

<!-- [rustup]: https://www.rustup.rs/ -->

<!-- ## Xargo -->

<!-- Simply call this command: -->

<!-- ``` -->
<!-- $ cargo install xargo -->
<!-- ``` -->

<!-- Note that Xargo 0.2.0+ depends on the `rust-src` component, so install that as -->
<!-- well: -->

<!-- ``` -->
<!-- $ rustup component add rust-src -->
<!-- ``` -->
