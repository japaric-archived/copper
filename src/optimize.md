Thanks for your interest! This book is currently outdated so I'm taking it down
until I get time to update it; that way it won't cause more confusion. In the
meantime, you can read [this blog post] that covers the *easy*, high level way
of writing Rust applications for any ARM Cortex-M microcontroller.

[this blog post]: http://blog.japaric.io/quickstart/

<!-- # (Mis)Optimization -->

<!-- Surprise: Enabling optimizations (`--release`) optimizes away our whole program! -->

<!-- Solution: -->

<!-- - Split program into a library to avoid the aggressive pre-linking removal of symbols. -->
<!-- - Use volatile loads/stores to prevent the compiler from optimizing away/coalescing memory accesses. -->
<!-- - Add linker script assertions to avoid future misoptimizations. -->
