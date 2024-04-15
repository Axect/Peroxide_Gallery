# Importance Sampling with Peroxide

## Overview

This project demonstrates the use of importance sampling techniques to perform numerical integration using the Rust library Peroxide. The code compares three different methods of integration:

1. **G7K15R**: A Gauss-Kronrod method with 7-point Gauss and 15-point Kronrod rules.
2. **Uniform Importance Sampling**: A basic importance sampling method using a uniform distribution.
3. **Weighted Uniform Importance Sampling**: A weighted importance sampling method that takes into account the function's characteristics.

The results, including the time taken and error for each method, are printed in a DataFrame.

## Requirements

- Rust programming language
- Peroxide crate

## Usage

Run the project using:

```bash
cargo run --release
```

The output will display the integration results for each method, including the time taken, error percentage, and the value of the integral.

## Functions

### `f(x: f64) -> f64`

The function to be integrated. It can be modified to represent any continuous function.

### `f_integrate_true(x: f64) -> f64`

A function that returns the true value of the integral for comparison purposes.

### `importance_sampling_uniform<F>(f: F, (a, b): (f64, f64), n: usize) -> f64`

A function that performs importance sampling using a uniform distribution.

### `importance_sampling_weighted<F>(f: F, (a, b): (f64, f64), n: usize) -> Result<f64>`

A function that performs importance sampling using a weighted uniform distribution.
