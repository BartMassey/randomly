![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

# {{crate}}: randomly choose an alternative
Bart Massey 2022 (version {{version}})

{{readme}}

To use `{{crate}}`, add the following to your `Cargo.toml`
```
[dependencies.{{crate}}]
version = "{{version}}"
git = "https://github.com/BartMassey/{{crate}}"
```
(Do *not* use version 0.1.0 of `{{crate}}`, as it generates
code exponentially sized in the number of actions, and could
thus increase your compile times drastically.)


Full crate
[rustdoc](https://bartmassey.github.io/{{crate}}/{{crate}}/index.html)
is available.

This crate is made available under the "{{license}}
license". Please see the file `LICENSE.txt` in this distribution
for license terms.

Thanks to the `cargo-readme` crate for generation of this `README`.
