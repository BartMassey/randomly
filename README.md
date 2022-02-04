![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

# randomly: randomly choose an alternative
Bart Massey 2022 (version 0.1.0)

Macro to select a random block. This is useful in games,
where taking random actions is common.

## Examples

```rust
use randomly::randomly;

let n = randomly! {
    { println!("hello"); 0 }
    { println!("goodbye"); 1 }
};
```

Full crate [rustdoc](https://bartmassey.github.io/randomly/randomly/index.html)
is available.

This crate is made available under the "MIT
license". Please see the file `LICENSE.txt` in this distribution
for license terms.

Thanks to the `cargo-readme` crate for generation of this `README`.
