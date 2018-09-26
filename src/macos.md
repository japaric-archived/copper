Thanks for your interest! This book has been deprecated in favor of [the
embedded Rust book][book]. Also check the rest of the [embedded WG's][wg]
[bookshelf].

[book]: https://rust-embedded.github.io/bookshelf/book/index.html
[wg]: https://github.com/rust-embedded/wg
[bookshelf]: https://rust-embedded.github.io/bookshelf/

<!-- # macOS -->

<!-- You can install most of the required tools using `brew`: -->

<!-- ``` -->
<!-- # NOTE if you get "Error: Unknown command: cask", then run this command: `brew tap Caskroom/cask` -->
<!-- # and try again -->
<!-- $ brew cask install gcc-arm-embedded -->
<!-- $ brew install openocd qemu -->
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

<!-- Note that Xargo 0.2.0+ depends on the `rust-src` component so install that as -->
<!-- well: -->

<!-- ``` -->
<!-- $ rustup component add rust-src -->
<!-- ``` -->

<!-- ## Testing OpenOCD -->

<!-- Follow [these instructions] to test connecting to your programmer/debugger using -->
<!-- OpenOCD. -->

<!-- [these instructions]: linux.html#First%20OpenOCD%20connection -->
