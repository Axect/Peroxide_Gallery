import pandas as pd
import matplotlib.pyplot as plt
import scienceplots
import numpy as np

# Import parquet file
df = pd.read_parquet('dct.parquet')

# Prepare Data to Plot
t = df['t']
x = df['x']
y = df['y']
x_hat = df['x_hat']
index = np.arange(len(x))

# Plot params
pparam = dict(
    xlabel = r'Frequency Index',
    ylabel = r'$y$',
    xscale = 'linear',
    yscale = 'linear',
    xlim   = (0, 100),
)

# Plot
with plt.style.context(["science", "nature"]):
    fig, ax = plt.subplots()
    ax.autoscale(tight=True)
    ax.set(**pparam)
    ax.plot(index, y, label=r'$y={\rm DCT}(x)$')
    ax.legend()
    fig.savefig('dct_plot.png', dpi=600, bbox_inches='tight')

# Plot params
pparam = dict(
    xlabel = r'$t$',
    ylabel = r'$x$',
    xscale = 'linear',
    yscale = 'linear',
)

# Plot
with plt.style.context(["science", "nature"]):
    fig, ax = plt.subplots()
    ax.autoscale(tight=True)
    ax.set(**pparam)
    ax.plot(t, x_hat, label=r'$\hat{x} = {\rm iDCT}(y)$')
    ax.plot(t, x, ':', label=r'$x_{\rm true}$')
    ax.legend()
    fig.savefig('idct_plot.png', dpi=600, bbox_inches='tight')
