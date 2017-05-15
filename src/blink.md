Thanks for your interest! This book is currently outdated so I'm taking it down
until I get time to update it; that way it won't cause more confusion. In the
meantime, you can read [this blog post] that covers the *easy*, high level way
of writing Rust applications for any ARM Cortex-M microcontroller.

[this blog post]: http://blog.japaric.io/quickstart/

<!-- # Blinking an LED -->

<!-- (with too many hexadecimals) -->

<!-- > **HEADS UP** Most of the links in this page are PDFs! -->

<!-- So far we have only used the processor inside our microcontroller. The processor -->
<!-- can only do math and logic, and, on its own, it can't interact with the external -->
<!-- world: it can't drive a LED or a motor, read a sensor or communicate with other -->
<!-- devices. -->

<!-- To make our programs more useful (and fun!) we must learn to use *peripherals*. -->
<!-- A peripheral is an extra piece of electronics that's built, alongside the -->
<!-- processor, in the same microcontroller package. Peripherals give the processor -->
<!-- the extra functionality it needs to interact with the external world. -->

<!-- > Awesome! What can I do with these peripherals? -->

<!-- All sort of things! There are several different types of peripherals, each one -->
<!-- provides a different functionality. Microcontrollers manufacturers call them by -->
<!-- different names even if they provide the same functionality though. Here are -->
<!-- some of the most common ones (using STM32 naming convention): -->

<!-- - `GPIO`. General Purpose Input/Output. Can be used to turn on/off external -->
<!--   devices (e.g. a motor, a lamp, etc.) or to read the state of a "switch" -->
<!--   (e.g. a two-state (ON/OFF) switch, a keyboard, etc.). -->

<!-- - `ADC`. Analog-to-Digital Converter. Can be used to "read" analog sensors (e.g. -->
<!--   a thermometer, a light (intensity) sensor, etc.) or signals (e.g. voltage -->
<!--   level of a battery, electric current, etc.). -->

<!-- - `TIM`. Timers. Can be used to perform periodic tasks (e.g. every 100 ms), -->
<!--   measure lengths of time (e.g. for how long was this button pressed?) or -->
<!--   generate periodic signals with variable [duty cycle][dc] -->
<!--   (AKA [Pulse Width Modulation][pwm] (PWM)). PWM is mainly used to control how -->
<!--   much power is supplied to an electric machine like a motor which, in turn, -->
<!--   lets you indirectly control other parameters like speed and torque. -->

<!-- [dc]: https://en.wikipedia.org/wiki/Duty_cycle -->
<!-- [pwm]: https://en.wikipedia.org/wiki/Pulse-width_modulation -->

<!-- We'll explore these and several other peripherals in more detail in a [later -->
<!-- chapter]. -->

<!-- [later chapter]: peripherals.html -->

<!-- > So, how do I use these peripherals? -->

<!-- Thanks to some magic called [memory mapped I/O][MMIO], to the processor, -->
<!-- peripherals appear as memory regions (!). This means that, for example, by -->
<!-- writing to some special memory address you can use the digital I/O peripheral to -->
<!-- turn on/off a LED. Another example: By reading from some special memory address -->
<!-- you can use the Analog to Digital Converter peripheral to "read" an analog -->
<!-- sensor like a thermometer and get the current environment temperature as a -->
<!-- digital/discrete value. -->

<!-- [MMIO]: https://en.wikipedia.org/wiki/Memory-mapped_I/O -->

<!-- A chunk of memory associated to a single peripheral is known as a "register -->
<!-- block".  As other types of memory, these regions are usually accessed in word -->
<!-- sized chunks (32-bit on ARM). Each of these word sized chunks is referred to as -->
<!-- a (hardware) register; though registers can also be half-word or byte sized. -->
<!-- Each of these registers has a human-friendly name and an address associated to -->
<!-- it. -->

<!-- A concrete example: The STM32F303VCT6 microcontroller has a peripheral known as -->
<!-- Reset and Clock Control (RCC). The register block associated with this -->
<!-- peripheral starts at address `0x4002_10000`. This register block is comprised of -->
<!-- several registers as seen on its [register map]. One of registers associated -->
<!-- with this peripheral is the ["AHB peripheral clock enable register"][AHBENR] -->
<!-- (AKA `APB2ENR`) which lives at address `0x4002_1014`. This particular register -->
<!-- can be used to power on/off other peripherals. -->

<!-- [register map]: http://www.st.com/content/ccc/resource/technical/document/reference_manual/4a/19/6e/18/9d/92/43/32/DM00043574.pdf/files/DM00043574.pdf/jcr:content/translations/en.DM00043574.pdf#page=166&zoom=auto,67,754 -->
<!-- [AHBENR]: http://www.st.com/content/ccc/resource/technical/document/reference_manual/4a/19/6e/18/9d/92/43/32/DM00043574.pdf/files/DM00043574.pdf/jcr:content/translations/en.DM00043574.pdf#page=148&zoom=auto,67,447 -->

<!-- To get familiar with the use of peripherals, we'll write the microcontroller -->
<!-- version of the "hello world" program: Blinking an LED. -->

<!-- ## The device-agnostic plan -->

<!-- This is an overview of what our program will do: -->

<!-- 1. Power on the digital output peripheral. -->

<!-- To save energy, most of the peripherals in a microcontroller boot in a powered -->
<!-- off state. We have to explicitly "power on" the peripherals we want to use. -->

<!-- 2. Put the *pin* that's connected to the LED in *output mode*. -->

<!-- A pin a metal contact that a microcontroller exposes and that can be -->
<!-- electrically connected to another device. A pin can either be (configured) to be -->
<!-- in input mode or in output mode, but it must be in output mode to be able to -->
<!-- drive (i.e. supply current to) an external device. Most pins start in input mode -->
<!-- right after the micro boots to avoid spuriously driving external devices. -->

<!-- 3. Set the pin *high* or *low* to turn on the LED. -->

<!-- *Low* means outputting zero volts (0V) on the pin whereas *high* means -->
<!-- outputting a non-zero voltage, usually the power supply voltage (3.3V on most -->
<!-- Cortex-M micros), on the pin. Depending on how the LED is wired to the pin, -->
<!-- setting the pin low/high should turn it off/on or the other way around. -->

<!-- After we've confirmed that we can turn the LED on/off, we'll modify the program -->
<!-- to toggle the state of the LED pin every few seconds. -->

<!-- ## The device-specific details -->

<!-- Now we must fill in the device-specific details to realize our plan. All the -->
<!-- needed information will come from the microcontroller reference manual -->
<!-- ([here's mine][rm]) and the dev board user manual ([here's mine][um]). -->

<!-- [rm]: http://www.st.com/resource/en/reference_manual/DM00043574.pdf -->
<!-- [um]: http://www.st.com/resource/en/user_manual/dm00063382.pdf -->

<!-- ### Which LED, which pin? -->

<!-- First, we must pick a LED on the dev board to work with. Your dev board very -->
<!-- likely has at least one "user LED" that's connected to one of the -->
<!-- microcontroller's pin (check its user manual). Don't confuse an "user LED" with -->
<!-- the "power LED". The latter is an indicator of whether the board is powered on -->
<!-- or off and can't be controlled by the microcontroller. -->

<!-- > **TODO** What do I do if my dev board doesn't have an "user LED"? -->

<!-- The STM32F3DISCOVERY has [eight user LEDs][leds]. For this example, I'll be -->
<!-- using the red one that's connected to the pin *PE9*. Because micros have many -->
<!-- I/O pins, these pins are usually grouped in *ports*. A port is a collection of -->
<!-- 8, 16, or some other number of pins. Ports are usually identified with letters: -->
<!-- A, B, etc. and the pins in it are usually identified with numbers: 0, 1, etc. -->
<!-- Therefore, you can think of the the pin PE9 as the 10th (because numbering -->
<!-- starts at 0) pin in the port E. -->

<!-- [leds]: http://www.st.com/content/ccc/resource/technical/document/user_manual/8a/56/97/63/8d/56/41/73/DM00063382.pdf/files/DM00063382.pdf/jcr:content/translations/en.DM00063382.pdf#page=18&zoom=auto,67,521 -->

<!-- ### How to power on a peripheral? -->

<!-- Micros have a dedicated peripheral that's in charge of "clocking" other -->
<!-- peripherals. Clocking in this context means powering on/off a peripheral. A -->
<!-- peripheral that doesn't receive a clock signal is basically powered off -- it -->
<!-- can't be used and it doesn't (actively) consume energy. -->

<!-- On STM32 micros this peripheral is called [RCC]. The family of `*ENR` registers -->
<!-- in this peripheral control the clocking of other peripherals. In my case, I'm -->
<!-- interested in the [AHBENR] register which contains a `IOPEEN` bit that controls -->
<!-- the clocking of the E port. -->

<!-- [RCC]: http://www.st.com/content/ccc/resource/technical/document/reference_manual/a2/2d/02/4b/78/57/41/a3/CD00246267.pdf/files/CD00246267.pdf/jcr:content/translations/en.CD00246267.pdf#page=69&zoom=auto,67,755 -->

<!-- ### How to put the pin in output mode? -->

<!-- In my case, I need to put the pin `PE9` in output mode. Some register in the -->
<!-- [GPIOE] peripheral should let me do that. After looking through the -->
<!-- documentation, I found that the `MODER` register does that. In particular, the -->
<!-- [MODER] register contains the bitfield `MODER9` which control the "mode" (input -->
<!-- or output) of the pin `PE9`. I'll use the following setting: -->

<!-- [GPIOE]: http://www.st.com/content/ccc/resource/technical/document/reference_manual/4a/19/6e/18/9d/92/43/32/DM00043574.pdf/files/DM00043574.pdf/jcr:content/translations/en.DM00043574.pdf#page=228&zoom=auto,67,755 -->
<!-- [MODER]: http://www.st.com/content/ccc/resource/technical/document/reference_manual/4a/19/6e/18/9d/92/43/32/DM00043574.pdf/files/DM00043574.pdf/jcr:content/translations/en.DM00043574.pdf#page=237&zoom=auto,67,669 -->

<!-- - `MODER9 = 0b01` Puts the pin in general purpose push-pull output mode. -->

<!-- ### Driving the pin high and low -->

<!-- Again the register that I want must be in the `GPIOE` peripheral. In this case, -->
<!-- it's the `BSRR` register. It can individually *set* or *reset* a pin. Here, -->
<!-- *reset* means putting the pin low and *set* means driving the pin high. -->

<!-- ## Putting it all together -->

<!-- Here's a detailed specification of the program: -->

<!-- 1. Turn on the GPIOC peripheral: Set the `IOPEEN` bit in the `RCC->AHBENR` -->
<!--    register to `1`. -->

<!-- 2. Put the PE9 pin in output mode: Set the `MODER9` bitfield in the -->
<!--    `GPIOE->MODER` register to `0b01`. -->

<!-- 3. Set the PE9 pin high: Set the `BS9` bit in the `GPIOE->BSRR` register to `1`. -->

<!-- 4. Set the PE9 pin low: Set the `BR9` bit in the `GPIOE->BSRR` register to `1`. -->

<!-- ## The code -->

<!-- And here's the code. I'm omitting the `exception` and `lang_items` modules which -->
<!-- haven't changed since [our previous program]. -->

<!-- [our previous program]: ./exceptions.html#Installing%20the%20exception%20handlers -->

<!-- ``` rust -->
<!-- #[export_name = "_reset"] -->
<!-- pub extern "C" fn main() -> ! { -->
<!--     power_on_gpioe(); -->
<!--     put_pe9_in_output_mode(); -->
<!--     set_pe9_high(); -->
<!--     set_pe9_low(); -->

<!--     loop {} -->
<!-- } -->

<!-- fn power_on_gpioe() { -->
<!--     /// Start address of the RCC register block -->
<!--     const RCC: u32 = 0x4002_1000; -->

<!--     /// Offset address of the AHBENR register -->
<!--     const RCC_AHBENR: u32 = 0x14; -->

<!--     /// IOPCEN bit mask -->
<!--     const RCC_AHBENR_IOPEEN: u32 = 1 << 21; -->

<!--     unsafe { -->
<!--         // Pointer to the AHBENR register -->
<!--         let ahbenr = (RCC + RCC_AHBENR) as *mut u32; -->

<!--         // IOPECN = 1 -->
<!--         *ahbenr |= RCC_AHBENR_IOPEEN; -->
<!--     } -->
<!-- } -->

<!-- /// Start address of the GPIOC register block -->
<!-- const GPIOE: u32 = 0x4800_1000; -->

<!-- /// Offset address of the BSRR register -->
<!-- const GPIOE_BSRR: u32 = 0x18; -->

<!-- fn put_pe9_in_output_mode() { -->
<!--     /// Offset address of the CRH register -->
<!--     const GPIOE_MODER: u32 = 0x0; -->

<!--     unsafe { -->
<!--         // Pointer to the MODER register -->
<!--         let moder = (GPIOE + GPIOE_MODER) as *mut u32; -->

<!--         // MODER9 = 0b01 -->
<!--         *moder = (*moder & !(0b11 << 18)) | (0b01 << 18) -->
<!--     } -->
<!-- } -->

<!-- fn set_pe9_high() { -->
<!--     unsafe { -->
<!--         // Pointer to the BSRR register -->
<!--         let bsrr = (GPIOE + GPIOE_BSRR) as *mut u32; -->

<!--         // BS9 = 1 -->
<!--         *bsrr = 1 << 9; -->
<!--     } -->
<!-- } -->

<!-- fn set_pe9_low() { -->
<!--     unsafe { -->
<!--         // Pointer to the BSRR register -->
<!--         let bsrr = (GPIOE + GPIOE_BSRR) as *mut u32; -->

<!--         // BR9 = 1 -->
<!--         *bsrr = 1 << (16 + 9); -->
<!--     } -->
<!-- } -->
<!-- ``` -->

<!-- Quite unsightly, right? So many magic values. In a [later chapter], we'll -->
<!-- refactor this code to get rid of the magic values, the pointer arithmetic and -->
<!-- the raw pointers. But this code will make do for now! -->

<!-- [later chapter]: ./registers.html -->

<!-- ## Test it -->

<!-- Time to test our code! Don't feel discouraged if your program crashes or doesn't -->
<!-- work on the first try! I certainly get most of my embedded programs wrong when -->
<!-- I'm just starting to write drivers and have to deal with all these magic values -->
<!-- and/or have to jump back and forth between the microcontroller reference manual -->
<!-- and my library/program. -->

<!-- OK, here's how I'd debug this program: -->

<!-- 1. Starting from the program entry point, `_step`, repeatedly `step` over the -->
<!--    program until you hit the the "set the pin high" statement, in my case this -->
<!--    is the `*bsrr = 1 << 8`. If you didn't hit an exception, congratulations! -->
<!--    Head to step 3, otherwise go to step 2. -->

<!-- 2. If you hit an exception, you should now know which statement triggered it. -->
<!--    Reset your microcontroller with `monitor reset halt`, then step all the way -->
<!--    until your reach the faulty statement but don't execute it!. At this point, -->
<!--    inspect the address of the register that will be modified by the faulty -->
<!--    statement. Is the address right/valid? If not, fix it then go to step 1. -->

<!-- 3. You should now be about to execute the instruction that sets the LED pin -->
<!--    high. Step from here all the way to the endless `loop`. This should toggle -->
<!--    the state of the LED at least once. If it doesn't, then quite a few things -->
<!--    could have gone wrong ... See below: -->

<!-- - Wrong register address as seen in step 2. -->

<!-- - `GPIO` has not been powered on or configured properly. You'll have to -->
<!--   "examine" (`(gdb) x $ADDRESS`) all the related registers. If you didn't power -->
<!--   on the `GPIO` peripheral, you'll see that trying to write to that peripheral -->
<!--   registers has no effect. -->

<!-- - You are driving the wrong pin, i.e. one that's not connected to an LED. -->
<!--   Confirm this against your dev board user manual. -->

<!-- ## Adding a loop -->

<!-- Now that we know that we can toggle the state of the LED. Making the LED blink -->
<!-- is relatively easy. We need to add a `delay` function and then move the LED -->
<!-- toggling code inside a loop: -->

<!-- ``` rust -->
<!-- #[export_name = "_reset"] -->
<!-- pub extern "C" fn main() -> ! { -->
<!--     power_on_gpioe(); -->
<!--     put_pe9_in_output_mode(); -->

<!--     let ticks = 100_000; -->
<!--     loop { -->
<!--         set_pe9_high(); -->
<!--         delay(ticks); -->
<!--         set_pe9_low(); -->
<!--         delay(ticks); -->
<!--     } -->
<!-- } -->

<!-- fn delay(n: u32) { -->
<!--     for _ in 0..n {} -->
<!-- } -->
<!-- ``` -->

<!-- I have no way of telling you what value of `n` will give you e.g. a delay of 1 -->
<!-- second because that depends on the built-in internal clock of your -->
<!-- microcontroller (mine is 8 MHz) and the actual instructions that `delay` -->
<!-- compiles to in debug mode. However, using a value between `10_000` and `100_000` -->
<!-- for `ticks` should make the LED blink at a visible rate. -->

<!-- ## Test it again -->

<!-- To test, simply flash the program and let it run from the debugger: -->

<!-- ``` -->
<!-- (gdb) continue -->
<!-- ``` -->

<!-- You should now see the LED blink at some rate. To make the LED blink faster make -->
<!-- the value of `ticks` smaller. To do this, first manually break the program by -->
<!-- pressing `Crtl-C` at `gdb`'s prompt, then use the following commands: -->

<!-- ``` -->
<!-- # break somewhere inside the loop -->
<!-- (gdb) break main.rs:13 -->

<!-- (gdb) continue -->
<!-- Breakpoint 1, app::main () at (..)/src/main.rs:13 -->
<!-- 13              set_pe9_high() -->

<!-- # make ticks smaller -->
<!-- (gdb) set ticks = 10000 -->

<!-- # clear breakpoint -->
<!-- (gdb) clear main.rs:13 -->

<!-- (gdb) continue -->
<!-- ``` -->

<!-- The LED should now blink at a faster rate. You can repeat the experiment but -->
<!-- setting `ticks` to a larger value. -->
