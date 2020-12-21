from netCDF4 import Dataset
import matplotlib.pyplot as plt

# Import netCDF file
ncfile = './data/data.nc'
data = Dataset(ncfile)
var = data.variables

# Use latex
plt.rc('text', usetex=True)
plt.rc('font', family='serif')

# Prepare Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"Simple Plot", fontsize=16)
plt.xlabel(r'$x$', fontsize=14)
plt.ylabel(r'$y$', fontsize=14)

# Prepare Data to Plot
x = var['x'][:]
y = var['y'][:]  

# Plot with Legends
plt.plot(x, y, label=r'$y=x^2$')

# Other options
plt.legend(fontsize=12)
plt.grid()
plt.savefig("plot.png", dpi=300)
