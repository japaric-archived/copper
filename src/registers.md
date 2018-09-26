Thanks for your interest! This book has been deprecated in favor of [the
embedded Rust book][book]. Also check the rest of the [embedded WG's][wg]
[bookshelf].

[book]: https://rust-embedded.github.io/bookshelf/book/index.html
[wg]: https://github.com/rust-embedded/wg
[bookshelf]: https://rust-embedded.github.io/bookshelf/

<!-- # Zero-cost type-safe register manipulation -->

<!-- Goals: -->

<!-- - Go from hexadecimal gibberish to structs. -->
<!-- - Prevent writing to read-only registers and viceversa. -->
<!-- - Prevent reading to,writing to or modifying reserved bits. -->
<!-- - Nicer debugging experience: `print GPIOA` -> you get all the registers in that register block and -->
<!--   their values. -->
