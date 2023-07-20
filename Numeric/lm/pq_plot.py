import pandas as pd
import matplotlib.pyplot as plt
import scienceplots

# Import parquet file
df = pd.read_parquet('data.parquet')

# Prepare Data to Plot
x = df['x']
y = df['y']
y_hat = df['y_hat']

# Plot params
pparam = dict(
    xlabel = r'$x$',
    ylabel = r'$y$',
    xscale = 'linear',
    yscale = 'linear',
    xlim = (0, 100),
)

# Plot
with plt.style.context(["science", "nature"]):
    fig, ax = plt.subplots()
    ax.autoscale(tight=True)
    ax.set(**pparam)
    ax.plot(x, y, '.', label=r'Data', alpha=0.5)
    ax.plot(x, y_hat, 'r-', label=r'Fit')
    ax.legend()
    fig.savefig('plot.png', dpi=600, bbox_inches='tight')
