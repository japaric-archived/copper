Thanks for your interest! This book has been deprecated in favor of [the
embedded Rust book][book]. Also check the rest of the [embedded WG's][wg]
[bookshelf].

[book]: https://rust-embedded.github.io/bookshelf/book/index.html
[wg]: https://github.com/rust-embedded/wg
[bookshelf]: https://rust-embedded.github.io/bookshelf/

<!-- # (Mis)Optimization -->

<!-- Surprise: Enabling optimizations (`--release`) optimizes away our whole program! -->

<!-- Solution: -->

<!-- - Split program into a library to avoid the aggressive pre-linking removal of symbols. -->
<!-- - Use volatile loads/stores to prevent the compiler from optimizing away/coalescing memory accesses. -->
<!-- - Add linker script assertions to avoid future misoptimizations. -->
