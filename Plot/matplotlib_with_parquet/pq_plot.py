import matplotlib.pyplot as plt
import pandas as pd

# Use latex
plt.rc('text', usetex=True)
plt.rc('font', family='serif')

# Import parquet file
df = pd.read_parquet('./data/plot.parquet')

# Prepare Data to Plot
x = df['x']
x2 = df['x2']
x3 = df['x3']

# Prepare Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"Plot", fontsize=16)
plt.xlabel(r'$x$', fontsize=14)
plt.ylabel(r'$y$', fontsize=14)

# Plot with Legends
plt.plot(x, x, label=r'$y=x$')
plt.plot(x, x2, label=r'$y=x^2$')
plt.plot(x, x3, label=r'$y=x^3$')

# Other options
plt.legend(fontsize=12)
plt.grid()
plt.savefig("plot.png", dpi=300)
