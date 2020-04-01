# RK4 with netcdf

## Requirements

* netcdf
* python netcdf4 library
* python matplotlib library (Draw plot)
* texlive or alternatives (Draw plot)

## Build process

```sh
# Build (Can simply : cargo build --release)
RUSTFLAGS="-C target-cpu=native" cargo build --release

# Run (Can simply : cargo run --release)
RUSTFLAGS="-C target-cpu=native" cargo run --release

# Draw Plot
python nc_plot.py
```