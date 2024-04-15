# Newmark-Beta method

## Description

Implement *Newmark Beta* method for damped harmonic oscillation

$$
m\ddot{u} + c\dot{u} + ku = 0
$$

* $m = 1\text{kg}$
* $k = 200 \text{N} / \text{m}$
* $c = \zeta \cdot 2\sqrt{km}$
* $\zeta = 0.01$

## Build Process

```sh
# If there is no data directory
mkdir data

# Build & Run
cargo run --release

# Plot
python pq_plot.py
```

## Result

![position](./position.png)

![velocity](./velocity.png)

![acceleration](./acceleration.png)

![energy](./energy.png)
