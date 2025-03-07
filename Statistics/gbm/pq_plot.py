import pandas as pd
import matplotlib.pyplot as plt
import scienceplots

# Import parquet file
df = pd.read_parquet('GBM.parquet')

# Prepare Data to Plot
t = df['t']
Exact = df['Exact']
Euler = df['Euler']
Drift = df['Drift']

# Plot params
pparam = dict(
    xlabel = r'$t$',
    ylabel = r'$X_t$',
    title = r"Exact vs Euler-Maruyama (GBM)",
    xscale = 'linear',
    yscale = 'log',
)

# Plot
with plt.style.context(["science", "nature"]):
    fig, ax = plt.subplots()
    ax.autoscale(tight=True)
    ax.set(**pparam)
    ax.plot(t, Exact, label='Exact', color='darkblue')
    ax.plot(t, Euler, label='Euler-Maruyama', color='maroon', linestyle=':', alpha=0.7)
    ax.plot(t, Drift, label='Drift', color='darkgreen', linestyle='--')
    ax.legend()
    fig.savefig('plot.png', dpi=600, bbox_inches='tight')
