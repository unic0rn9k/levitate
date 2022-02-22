## Numbers

## Thanks
Heavy inspiration has been taken from crates listed bellow.

- https://lib.rs/crates/num
- https://lib.rs/crates/fast-floats

## Example

Bellow an example of eulers identity is shown.

<img src="https://render.githubusercontent.com/render/math?math=e^{i \pi} = -1">

```rust
use num::*;

assert!(
    (re(fast(std::f32::consts::PI)) * im(fast(1.))).exp_().into_primitive()
    .abs() - 1. < 0.0000002
);
```
