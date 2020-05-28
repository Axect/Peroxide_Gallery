---
fontfamily: "libertine"
mainfont: "GFS Artemisa"
title: "FEM Homework"
author: [Tae Geun Kim]
date: 2020-05-28
subject: "Markdown"
keywords: [Markdown, Example]
subtitle: "Inverstigate Convergence1"
titlepage: true
toc-own-page: true
...

\tableofcontents

\newpage

# Problem

Take $I_h u$ to be interpolant of $u$ so that $I_h u \in P_1$.
Invetigate convergence of $I_h u$ to $u$.

1. $\Vert u - I_h u\Vert_{L^2} \leq c h^2 \Vert u \Vert_{H^2}$

2. $\Vert u - I_h u\Vert_{H^1} \leq c h \Vert u \Vert_{H^2}$

# Process

2D에 대한 구현이 어려워서 1D로 일단 구현해보았습니다. 구현과정은 다음과 같습니다.

1. 주어진 stepsize $h$에 대해 구간 $(0,1)$에서 주어진 함수 $u$를 Piecewise linear하게 interpolation합니다. 따라서 각 구간별로 다항식을 얻습니다.

2. $u$와 $I_h u$의 차이를 측정합니다.
    * $\Vert u - I_h u\Vert_{L^2}$를 측정하기 위해서 Order 15의 Gaussian-Legendre quadrature를 사용했습니다.
    * $\Vert u - I_h u\Vert_{H^1}$을 측정하기 위해서 Gradient는 Dual number structure에 대한 Automatic differentiation을 이용하여 계산하였고 적분은 위와 같이 Order 15의 Gaussian-Legendre quadrature를 사용했습니다.

3. $u$의 norm을 측정합니다.
    * $u$의 $H^2$ norm을 측정하기 위하여 Hessian은 Hyper dual number structure에 대한 Automatic differentiation을 이용하여 계산하였고 Order 15의 Gaussian-Legendre quadrature를 사용헀습니다.

4. $h=2^{-1}$ 부터 $h=2^{-10}$까지 총 10개의 stepsize에 대해서 1 ~ 3 과정을 반복하여 데이터를 얻습니다. $u=\sin \pi x$를 사용하였습니다. 얻은 데이터를 Log scale의 그래프로 그립니다. (스케일의 차이가 꽤 나서 $u$의 $H^2$ norm에는 0.01을 곱하였습니다.)

# Tools

* 모든 계산 코드는 Rust로 작성하였으며 제가 만든 Library인 Peroxide를 이용하였습니다. 모든 함수의 소스 코드는 [github.com/Axect/Peroxide](https://github.com/Axect/Peroxide)에 있습니다.

* 계산을 수행한 뒤 데이터는 netcdf 파일로 저장합니다. 이후 Python으로 해당 데이터를 로드한 뒤, matplotlib을 이용하여 그래프를 그렸습니다.

\newpage

# Results

![t=2, m=0](./plot/t2m0.png){ width=80% }

![t=2, m=1](./plot/t2m1.png){ width=80% }\newpage

\newpage

# Source code

코드 파일이 조금 길어서 다음 링크로 첨부합니다.

