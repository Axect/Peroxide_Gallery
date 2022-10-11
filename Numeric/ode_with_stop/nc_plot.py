from netCDF4 import Dataset
import matplotlib.pyplot as plt

# Use latex
plt.rc('text', usetex=True)
plt.rc('font', family='serif')

# Import netCDF file
ncfile = './data/data.nc'
data = Dataset(ncfile)
var = data.variables

# Prepare Data to Plot
x = var['x'][:]
y = var['y'][:]  

# Prepare Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"ODE with Stop", fontsize=16)
plt.xlabel(r'$x$', fontsize=14)
plt.ylabel(r'$y$', fontsize=14)

# Plot
plt.plot(x, y)

# Other options
plt.grid()
plt.savefig("plot.png", dpi=300)
