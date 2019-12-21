from netCDF4 import Dataset
import matplotlib.pyplot as plt

# Import netCDF file
ncfile = 'data/plot.nc'
data = Dataset(ncfile)
var = data.variables

# Use latex
plt.rc('text', usetex=True)
plt.rc('font', family='serif')

# Prepare Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"Test Plot", fontsize=16)
plt.xlabel(r'$x$', fontsize=14)
plt.ylabel(r'$y$', fontsize=14)

# Prepare Data to Plot
x = var['x'][:]
y1 = var['x'][:]
y2 = var['x2'][:]
y3 = var['x3'][:]

# Plot with Legends
plt.plot(x, y1, label=r'$y=x$')
plt.plot(x, y2, label=r'$y=x^2$')
plt.plot(x, y3, label=r'$y=x^3$')

# Other options
plt.legend(fontsize=12)
plt.grid()
plt.savefig("plot.png", dpi=300)
