Thanks for your interest! This book has been deprecated in favor of [the
embedded Rust book][book]. Also check the rest of the [embedded WG's][wg]
[bookshelf].

[book]: https://rust-embedded.github.io/bookshelf/book/index.html
[wg]: https://github.com/rust-embedded/wg
[bookshelf]: https://rust-embedded.github.io/bookshelf/

<!-- # Linux -->

<!-- ## Shortcut: Just use this docker image -->

<!-- It's based on Ubuntu 16.04 and comes with all the required dependencies: -->

<!-- ``` -->
<!-- $ docker run --privileged -it japaric/copper:2016-05-10 -->
<!-- # Or use a newer tag. See https://hub.docker.com/r/japaric/copper/tags -->
<!-- ``` -->

<!-- Alternatively, instead of using this docker image, you can ... -->

<!-- ## Install the tools on your system -->

<!-- On most Linux distributions, most of the required tools can be installed via the -->
<!-- package manager. The actual command that you need to call will depend on your -->
<!-- Linux distribution. But, here's the one for Ubuntu: -->

<!-- ``` -->
<!-- $ sudo apt-get install gcc-arm-none-eabi gdb-arm-none-eabi openocd qemu-system-arm -->
<!-- ``` -->

<!-- To install Rust and Cargo, I recommend using [rustup]: -->

<!-- [rustup]: https://www.rustup.rs/ -->

<!-- ``` -->
<!-- $ curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain=nightly -->
<!-- ``` -->

<!-- Or if you already have rustup installed, switch to the nightly channel with: -->

<!-- ``` -->
<!-- $ rustup default nightly -->
<!-- ``` -->

<!-- To install Xargo simply use: -->

<!-- ``` -->
<!-- $ cargo install xargo -->
<!-- ``` -->

<!-- Note that Xargo 0.2.0+ depends on the `rust-src` component, so install that as -->
<!-- well: -->

<!-- ``` -->
<!-- $ rustup component add rust-src -->
<!-- ``` -->

<!-- ## First OpenOCD connection -->

<!-- [first connection]: linux.html#First%20connection -->

<!-- > **TODO** document STM32VLDISCOVERY quirk -->

<!-- Even if using the Docker image, it's a good idea to test that OpenOCD works by -->
<!-- establishing a connection between your host system (PC, laptop, etc.) and your -->
<!-- dev board. First, you'll have to physically connect your dev board and your host -->
<!-- system via an USB cable. Then, you'll have to use a command that looks like -->
<!-- this: -->

<!-- ``` -->
<!-- $ sudo openocd -f board/$BOARD -->
<!-- ``` -->

<!-- if you are using a dev board that has a built-in debugger. Or one like this: -->

<!-- ``` -->
<!-- $ sudo openocd -f interface/$INTERFACE -f target/$TARGET -->
<!-- ``` -->

<!-- if you are using an external programmer/debugger. Then, you should get an output -->
<!-- like this: -->

<!-- ``` -->
<!-- Open On-Chip Debugger 0.9.0 (2015-09-02-10:42) -->
<!-- Licensed under GNU GPL v2 -->
<!-- For bug reports, read -->
<!-- http://openocd.org/doc/doxygen/bugs.html -->
<!-- Info : The selected transport took over low-level target control. The results might differ -->
<!-- compared to plain JTAG/SWD -->
<!-- adapter speed: 1000 kHz -->
<!-- adapter_nsrst_delay: 100 -->
<!-- none separate -->
<!-- srst_only separate srst_nogate srst_open_drain connect_deassert_srst -->
<!-- Info : Unable to match requested speed 1000 kHz, using 950 kHz -->
<!-- Info : Unable to match requested speed 1000 kHz, using 950 kHz -->
<!-- Info : clock speed 950 kHz -->
<!-- Info : STLINK v1 JTAG v11 API v2 SWIM v0 VID 0x0483 PID 0x3744 -->
<!-- Info : using stlink api v2 -->
<!-- Info : stm32f1x.cpu: hardware has 6 breakpoints, 4 watchpoints -->
<!-- ``` -->

<!-- > **TODO** add troubleshooting instructions for when the `openocd` command -->
<!-- > fails. -->

<!-- The program will block with that output, but it's okay to exit it with `Ctrl-C` -->
<!-- at this time. -->

<!-- As for the actual values of `$BOARD`/`$INTERFACE`/`$TARGET` that you must use, -->
<!-- the possible values are in `/usr/share/openocd/scripts` (might be a different -->
<!-- directory in your Linux distribution): -->

<!-- ``` -->
<!-- $ tree /usr/share/openocd/scripts -->
<!-- /usr/share/openocd/scripts -->
<!-- ├── (...) -->
<!-- ├── board -->
<!-- │   ├── actux3.cfg -->
<!-- │   ├── adapteva_parallella1.cfg -->
<!-- │   └── (...) -->
<!-- ├── interface -->
<!-- │   ├── altera-usb-blaster2.cfg -->
<!-- │   ├── altera-usb-blaster.cfg -->
<!-- │   └── (...) -->
<!-- ├── target -->
<!-- │   ├── aduc702x.cfg -->
<!-- │   ├── aducm360.cfg -->
<!-- │   └── (...) -->
<!-- └── (...) -->
<!-- ``` -->

<!-- Try something that resembles the name of your hardware. For example, for the -->
<!-- STM32VLDISCOVERY I use: -->

<!-- ``` -->
<!-- $ sudo openocd -f board/stm32vldiscovery.cfg -->
<!-- ``` -->

<!-- And for the STM32F3DISCOVERY, I use: -->

<!-- ``` -->
<!-- $ sudo openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg -->
<!-- ``` -->

<!-- ### (Optional) OpenOCD without `sudo` -->

<!-- > **NOTE** For those using the Docker image. You have to run these commands on -->
<!-- > the host system, *not* from within the container. -->

<!-- The reason we have to use `sudo` in the `openocd` invocations is that we don't -->
<!-- have sufficient permissions to use the USB device. This can be fixed using -->
<!-- `udev` rules. -->

<!-- First let's identify the USB device OpenOCD is using from the output of `sudo -->
<!-- openocd`: -->

<!-- ``` -->
<!-- $ sudo openocd -->
<!-- ``` -->

<!-- ``` -->
<!-- $ lsusb -->
<!-- (...) -->
<!-- Bus 003 Device 116: ID 0483:3744 STMicroelectronics STLINK Pseudo disk -->
<!-- (...) -->
<!-- ``` -->

<!-- Device number 116 on the bus 3, let's check its permissions: -->

<!-- ``` -->
<!-- $ ls -l /dev/bus/usb/003/116 -->
<!-- crw-rw-r-- 1 root root 189, 371 May  9 15:39 /dev/bus/usb/003/116 -->
<!-- ``` -->

<!-- Only `root` can read/write from/to it. We'll write an udev rule to change the -->
<!-- permissions of this particular USB device. udev rules are stored in -->
<!-- `/etc/udev/rules.d` as files, let's add a new one: -->

<!-- ``` -->
<!-- $ cat /etc/udev/rules.d/99-openocd.rules -->
<!-- # STLINKv1 -->
<!-- ATTRS{idVendor}=="0483", ATTRS{idProduct}=="3744", GROUP="users" -->
<!-- ``` -->

<!-- This udev rule changes the group of the USB device to `users`. -->

<!-- > **NOTE** You *can* use a group different than `users`. **But**, if you are -->
<!-- > using a Docker container, it's very likely that the id of a different group -->
<!-- > won't match between the host system and the container -- in that case you -->
<!-- > still won't have enough permissions to use the USB device!. -->

<!-- > **NOTE** For more details about the udev rules see [man 7 udev] -->

<!-- [man 7 udev]: http://linux.die.net/man/7/udev -->

<!-- You'll have to change 0483 and 3744 for the vendor and product id of **your** -->
<!-- device respectively. You can get those values from `lsusb`: -->

<!-- ``` -->
<!-- $ lsusb | grep STLINK -->
<!-- Bus 003 Device 116: ID 0483:3744 STMicroelectronics STLINK Pseudo disk -->
<!--                        ^^^^ ^^^^ -->
<!-- ``` -->

<!-- This new rule won't come into effect until you reload all the rules with: -->

<!-- ``` -->
<!-- $ sudo udevadm control --reload-rules -->
<!-- ``` -->

<!-- Now, unplug and re-plug the device and you should see the updated permissions: -->

<!-- ``` -->
<!-- $ lsusb | grep STLINK -->
<!-- Bus 003 Device 118: ID 0483:3744 STMicroelectronics STLINK Pseudo disk -->

<!-- $ ls -l /dev/bus/usb/003/118 -->
<!-- crw-rw-r-- 1 root users 189, 373 May  9 16:00 /dev/bus/usb/003/118 -->
<!-- ``` -->

<!-- You should now be able to use your `openocd` command without `sudo` **if** your -->
<!-- user was already part of the `users` group. If your user wasn't in that group, -->
<!-- you can add yourself to this group with this command: -->

<!-- ``` -->
<!-- $ sudo usermod -a -G users $(whoami) -->
<!-- ``` -->

<!-- You'll have to re-log for this last command to take effect. -->
