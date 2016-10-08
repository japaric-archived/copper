# Build & inspect

Let's start by creating a new Cargo project:

```
$ cargo new --bin app && cd $_
```

And then add these files to the project:

- [layout.ld](./first/layout.ld) - a "linker script"
- [cortex-m3.json](./first/cortex-m3.json) - a "target specification file"
- [src/main.rs](./first/main.rs)

I'll explain what these two first files are for in a later section. It's not important to know what
they do *right now*.

Your project directory should look like this:

```
$ tree .
```

```
.
├── Cargo.toml
├── cortex-m3.json
├── layout.ld
└── src
    └── main.rs
```

For convenience, `src/main.rs` is replicated here:

``` rust
#![feature(lang_items)]

// We won't use the usual `main` function. We are going to use a different "entry point".
#![no_main]

// We won't use the standard library because it requires OS abstractions like threads and files and
// those are not available in this platform.
#![no_std]

// Conceptually, this is our program "entry point". It's the first thing the microcontroller will
// execute when it (re)boots. (As far as the linker is concerned the entry point must be named
// `start` (by default; it can have a different name). That's why this function is `pub`lic, named
// `start` and is marked as `#[no_mangle]`.)
//
// Returning from this function is undefined because there is nothing to return to! To statically
// forbid returning from this function, we mark it as divergent, hence the `fn() -> !` signature.
#[no_mangle]
pub fn start() -> ! {
    // Our first program initializes some variables on the stack and does nothing more. Yay!
    let x = 42;
    let y = x;

    // We can't return from this function so we just spin endlessly here.
    loop {}
}

// Ignore this part for now :-). It will covered in a later section.
mod vector_table {
    #[link_section = ".reset"]
    static RESET: fn() -> ! = ::start;
}

// Finally, we need to define the panic_fmt and eh_personality "lang item"s, which are just
// functions. This specifies what the program should do when a `panic!` occurs. Our program won't
// panic, so we can leave the function bodies empty for now.
mod lang_items {
    #[lang = "panic_fmt"]
    extern fn panic_fmt() {}

    #[lang = "eh_personality"]
    pub extern fn rust_eh_personality() {}
}
```

If you have written a native Rust program before, this shouldn't look that different. It's just as
if our "`main`" function, which in this case is named `start`, has a different signature.

Before building the project, add these to your Cargo.toml:

``` diff
+[profile.dev]
+panic = "abort"
+
+[profile.release]
+panic = "abort"
```

These disable the unwinding mechanism on panics and change it to just "abort" on panics. Again, our
program doesn't have to deal with panics but changing the panic mechanism to abort let us avoid
having to define another lang item.

Now you can build the Cargo project:

```
$ xargo build --target cortex-m3
```

```
   Compiling app v0.1.0
```

> **NOTE** Do **not** build the project in release mode!

## Trust, but verify

The project appeared to build fine, but it's always a good idea to inspect the executable. We are
going to look at three things:

### Memory layout

First, you have to appreciate the fact that, during its execution, everything in a program resides
in memory and has an associated memory address. Now, everything that's *statically allocated*, like
functions and static variables, gets assigned an address at compile time. Whereas, everything else,
like local/stack variables and "heap" allocated variables, gets assigned an address at runtime.

We can observe the addresses of statically allocated stuff using `objdump`. Let's check our
executable:

```
# with
$ arm-none-eabi-objdump --demangle --disassemble-all target/cortex-m3/debug/app
# or with the equivalent
$ arm-none-eabi-objdump -CD target/cortex-m3/debug/app
```

```
target/cortex-m3/debug/app:     file format elf32-littlearm


Disassembly of section .vector_table:

00000000 <app::vector_table::RESET::ha7f0c0bde3147d64-0x4>:
   0:	20010000 	andcs	r0, r1, r0

00000004 <app::vector_table::RESET::ha7f0c0bde3147d64>:
   4:	00000009 	andeq	r0, r0, r9

Disassembly of section .text:

00000008 <start>:
   8:	b082      	sub	sp, #8
   a:	202a      	movs	r0, #42	; 0x2a
   c:	9001      	str	r0, [sp, #4]
   e:	9000      	str	r0, [sp, #0]
  10:	e7ff      	b.n	12 <start+0xa>
  12:	e7fe      	b.n	12 <start+0xa>

(...)
```

Let me highlight the important bits of the output:

```

00000000
   0:	20010000

00000004
   4:	00000009

00000008 <start>:
   8:	          	sub	sp, #8
   a:	          	movs	r0, #42	; 0x2a
   c:	          	str	r0, [sp, #4]
   e:	          	str	r0, [sp, #0]
  10:	          	b.n	12 <start+0xa>
  12:	          	b.n	12 <start+0xa>
```

Now let's read the output section by section:

```
00000000
   0:	20010000
```

"The memory at address `0x0` holds the (`u32`) value `0x2001_0000`". When the program starts its
execution, that part of the memory will be already initialized to that value.

```
00000004
   4:	00000009
```

Likewise, "the memory at `0x4` holds the  (`u32`) value `0x9`".

```
00000008 <start>:
   8:	          	sub	sp, #8
   a:	          	movs	r0, #42	; 0x2a
   c:	          	str	r0, [sp, #4]
   e:	          	str	r0, [sp, #0]
  10:	          	b.n	12 <start+0xa>
  12:	          	b.n	12 <start+0xa>
```

Finally, "the `start` function resides at address `0x8`". The lines, like `sub sp, #8`, below
`<start>` are the instructions that make up the function. When the processor executes this function,
it will execute each one of these instructions.

But, why are these addresses/values important? During boot up, the microcontroller reads the memory
section at `0x0..0x8` and uses those  values to perform the boot process. In other words, the values
in that memory section have an special meaning to the boot process. In a [later section], I'll
explain what those values mean and how they are used in the boot process. For now, it's okay to just
confirm that the memory at `0x0..0x8` is initialized.

[later section]: details/boot.html

### Program size

Microcontrollers are memory constrained devices. It's important to keep an eye on the size of our
programs to make sure they don't exceed the memory capacity of our device. We'll use the `size`
command to do that:

```
$ arm-none-eabi-size target/cortex-m3/debug/app
```
```
   text	   data	    bss	    dec	    hex	filename
     20       0       0      20      14 target/cortex-m3/debug/app
```

The output mentions three different memory *sections*:

- The `text` section contains the "program code", i.e. all the instructions that the processor will
  execute when the program is executed. Functions go in this section. This section also stores
  constants like strings. `static CONSTANT: &'static str = "Hello, world!"`is an example of a
  constant.
- `data` holds static variables that have an initial value, for example `static mut X: i32 = 42`.
- On the contrary, `bss` holds "uninitialized" static variables. Because of C heritage, here
  "uninitialized" actually means *zeroed*. For example, `static mut ZEROS: [u8; 4] = [0; 4]` would
  go in this section.

`dec` is just the sum of these three sections and `hex` is the hexadecimal representation of `dec`.

Our program size is just 20 bytes big! Microcontrollers usually have memory capacities in the order
of a few KiB up to hundreds of KiB. In particular, the LM3S6965 microcontroller, which we are going
to emulate, has 256KiB of (Flash) memory. So, no problem here; this program will fit in the device
memory.

> **NOTE** KiB = [Kibibyte] = 1024 bytes

[Kibibyte]: https://en.wikipedia.org/wiki/Kibibyte

### Entry point

To work properly, tools like `gdb` and `qemu` need to be informed about the program entry point,
which is where the program starts its execution.

We can check our program entry point using the `readelf` command:

```
$ arm-none-eabi-readelf -h target/cortex-m3/debug/app
```

```
ELF Header:
  Magic:   7f 45 4c 46 01 01 01 00 00 00 00 00 00 00 00 00
  Class:                             ELF32
  Data:                              2's complement, little endian
  Version:                           1 (current)
  OS/ABI:                            UNIX - System V
  ABI Version:                       0
  Type:                              EXEC (Executable file)
  Machine:                           ARM
  Version:                           0x1
  Entry point address:               0x8
  Start of program headers:          52 (bytes into file)
  Start of section headers:          67748 (bytes into file)
  Flags:                             0x5000202, has entry point, Version5 EABI, soft-float ABI
  Size of this header:               52 (bytes)
  Size of program headers:           32 (bytes)
  Number of program headers:         2
  Size of section headers:           40 (bytes)
  Number of section headers:         16
  Section header string table index: 13
```

There you can see that `0x8` is the entry point address. From the previous `objdump` output, you
know that `0x8` is the address of the `start` function. This means that the entry point have been
correctly declared.
