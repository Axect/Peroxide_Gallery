# ODE with Environment

## Description

### Prepare environment

* `x` : [1, 2, 3, ..., 10]
* `y` = [1^5, 2^4, 3^3, ..., 10^{-5}]
* `c` : `CubicSpline` of `x`, `y`

### Target ODE

* dy/dx = c(x) (c is `CubicSpline`)

## Process

```sh
# Build
cargo build --release

# Run
cargo run --release

# Plot
python nc_plot.py
```
