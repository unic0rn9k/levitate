## Numbers [![GitHub Workflow Status](https://img.shields.io/github/workflow/status/unic0rn9k/num/Rust?label=tests&logo=github)](https://github.com/unic0rn9k/num/actions/workflows/rust.yml)

## Example

Bellow an example of eulers identity is shown.

e^(i * pi) = -1

```rust
use levitate::*;

assert_eq!(
    im(std::f32::consts::PI).exp_().re,
    -1.
);
```

## Thanks
Heavy inspiration has been taken from crates listed bellow.

- [lib.rs/num](https://lib.rs/crates/num)
- [lib.rs/fast-floats](https://lib.rs/crates/fast-floats)

License: MIT
