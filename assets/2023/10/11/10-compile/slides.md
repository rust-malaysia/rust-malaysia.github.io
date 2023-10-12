---
fonts:
  sans: 'Dejavu Sans'
  serif: 'Dejavu Serif'
  mono: 'Hack'
  provider: 'none'
---

# Compile Faster: Rust Optimization Tips!

---
---

> Rust is slow

Really?

Looking at 500+ crates when building a simple hello world webapp...

---
---

# Visualization

> 知彼知己百战百胜《孙子兵法》- The art of war

Show cargo build timings

```shell
$ cargo build --timings
```

![cargo-timings](/cargo-timings.avif)

---
---

# `cargo check` instead of `cargo build`

- skip compilation but have linting, type-checking and borrow-checking
- usually *when it compiles, it works* applies to **check** too
- alternative, use `rust-analyzer` LSP

---
---

# Remove unused dependencies

- https://github.com/est31/cargo-udeps
- `cargo install cargo-udeps`
- `cargo +nightly udeps`

---
---

# Find duplicate dependencies

- sometimes dependencies have late peer-dependencies
  - `A 1.0` depends on `B 1.0`
  - `C 1.0` depends on `B 0.1`
  - `B 1.0` and `B 0.1` both needs compilation
- find these with `cargo tree --duplicates`
  - contribute back to the community! benefits everyone

---
---

# Replace heavy dependencies

- `cargo tree` and `cargo bloat` https://github.com/RazrFalcon/cargo-bloat
- examples
  - `serde` -> `miniserde`
  - `reqwests` -> `ureq`
  - `clap` -> `pico-args`
- going extreme, prevent dependencies on procedural macro crates

---
---

# Use workspaces - split into mulitple small crates

- move one big crate into multiple small crates
- https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html

---
---

# Use cargo-nextest for faster test execution

- https://nexte.st
- > Up to 3× as fast as cargo test

---
---

# Combine all integration tests into a single binary

- https://azriel.im/will/2019/10/08/dev-time-optimization-part-1-1.9x-speedup-65-less-disk-usage/
- create a `main.rs` in your test folder and add your test files as `mod` in there

---
---

# Use a faster linker

- linux - [`mold`](https://github.com/rui314/mold)

---
---

# tl;dr

control your dependencies

---
---

# References

- https://endler.dev/2020/rust-compile-times/

