# Linear regression with additional error term

Solve peroxide issue [#26](https://github.com/Axect/Peroxide/issues/26).

## Description

* Data : `y = 2*x + 1 + eps_true`
* Model : `y_i = w_0 + w_1 * x_i + eps_i`

## Version info (Cargo.toml)

```toml
peroxide = "0.25"
```

## Build Process

```sh
cargo run --release
```

