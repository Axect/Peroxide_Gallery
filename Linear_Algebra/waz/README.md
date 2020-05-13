# WAZ Factorization

## Reference

Ke Chen, *Matrix Preconditioning Techniques and Applications*, Cambridge Monographs on Applied and Computational Mathematics

## Description

LU factorization requires substitution to solve linear system.
But there is substitution free algorithm - WAZ factorization.

`L^{-1} A U{-1} = W^T A Z`

## Build Process

```sh
# Build
cargo build --release

# Run
cargo run --release
```
