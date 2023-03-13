import matplotlib.pyplot as plt
import pandas as pd
import numpy as np
import scienceplots

# Import netCDF file
df = pd.read_parquet("./svm.parquet")

# Prepare Data to Plot
x       = df['x']
y       = df['y']
g       = df['g']
g_hat   = df['g_hat']
w       = df['w']
b       = df['b']
A       = df['A'][0]
B       = df['B'][0]

domain  = np.linspace(x.min(), x.max(), 1000)
hyper   = -w[0] * domain + b[0]
platt   = 1 / (1 + np.exp(A * x + B))
platd   = 1 / (1 + np.exp(A * domain + B))


# Plot params
pparam = dict(
    xlabel = r'$x$',
    ylabel = r'$y$',
    title = r"SVM",
    xscale = 'linear',
    yscale = 'linear',
    xlim = (x.min(), x.max()),
    ylim = (y.min(), y.max()),
)

# Plot
with plt.style.context(["science", "nature"]):
    fig, ax = plt.subplots()
    ax.autoscale(tight=True)
    ax.set(**pparam)
    ax.plot(domain, hyper, 'k--', label=r'$\hat{g}(x)$')
    ax.scatter(x, y, c=g, cmap='bwr', s=1, alpha=0.5, label=r'$g(x)$')
    ax.legend()
    fig.savefig('svm.png', dpi=300, bbox_inches='tight')

# Plot
with plt.style.context(["science", "nature"]):
    fig, ax = plt.subplots()
    ax.autoscale(tight=True)
    ax.set(**pparam)
    ax.set(title="Platt Scaling")
    ax.set(ylim=(-0.1, 1.1))
    ax.set(ylabel=r'$P(y=1|x)$')
    ax.plot(domain, platd, 'k--', label=r'$\hat{g}(x)$')
    ax.scatter(x, platt, c=g, cmap='bwr', s=1, alpha=0.5, label=r'$g(x)$')
    ax.axhline(0.5, color='purple', linestyle='--', alpha=0.5, label=r'$P(y=1|x)=0.5$')
    ax.legend()
    fig.savefig('platt.png', dpi=300, bbox_inches='tight')
