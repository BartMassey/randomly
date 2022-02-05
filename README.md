![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

# randomly: randomly choose an alternative
Bart Massey 2022 (version 0.2.1)

Macro to select a random block. This is useful in games,
where taking random actions is common.

## Examples

```rust
use randomly::*;

let n = randomly! {
    { println!("hello"); 0 }
    { println!("goodbye"); 1 }
};
println!("chose {}", n);
```

To use `randomly`, add the following to your `Cargo.toml`
```
[dependencies.randomly]
version = "0.2.1"
git = "https://github.com/BartMassey/randomly"
```
(Do *not* use version 0.1.0 of `randomly`, as it generates
code exponentially sized in the number of actions, and could
thus increase your compile times drastically.)


Full crate
[rustdoc](https://bartmassey.github.io/randomly/randomly/index.html)
is available.

This crate is made available under the "MIT
license". Please see the file `LICENSE.txt` in this distribution
for license terms.

Thanks to the `cargo-readme` crate for generation of this `README`.
