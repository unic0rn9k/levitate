## Numbers [![GitHub Workflow Status](https://img.shields.io/github/workflow/status/unic0rn9k/num/Rust?label=tests&logo=github)](https://github.com/unic0rn9k/num/actions/workflows/rust.yml)

## Thanks
Heavy inspiration has been taken from crates listed bellow.

- https://lib.rs/crates/num
- https://lib.rs/crates/fast-floats

## Example

Bellow an example of eulers identity is shown.

e^(i * pi) = -1

```rust
use num::*;

assert_eq!(
    im(std::f32::consts::PI).exp_().re,
    -1.
);
```
