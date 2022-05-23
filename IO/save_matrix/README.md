# Save matrix with DataFrame & netcdf

## Description

1. Make `Matrix`
2. Flatten to `Vec<f64>` and write a vector with shape information as `Vec<usize>`
3. Put them together into `DataFrame`
4. Write `DataFrame` with netcdf format
5. Read `DataFrame` from netcdf format
6. Restore `Matrix` with flatten vector & shape information

## Build Process

```sh
# Build
cargo build --release

# Run
cargo run --release
```
