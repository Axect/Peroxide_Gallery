from netCDF4 import Dataset
import matplotlib.pyplot as plt
import numpy as np

# Import netCDF file
ncfile = './kde.nc'
data = Dataset(ncfile)
var = data.variables

# Use latex
plt.rc('text', usetex=True)
plt.rc('font', family='serif')

# Prepare Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"KDE for Random walk", fontsize=16)
plt.xlabel(r'End points', fontsize=14)
plt.ylabel(r'Density', fontsize=14)

# Prepare Data to Plot
x = var['x'][:]
y = var['pdf'][:]  

# Plot with Legends
plt.plot(x, y, 'r--', label=r'KDE ($\lambda=1$)', alpha=0.8)
plt.plot(x, np.exp(-x**2 / 20) / np.sqrt(20 * np.pi), 'b--', label=r'$\displaystyle w(x,\,10)=\frac{1}{\sqrt{20\pi}}e^{-\frac{x^2}{20}}$', alpha=0.5)

# Other options
plt.legend(fontsize=14)
plt.grid()
plt.savefig("plot.png", dpi=300)
