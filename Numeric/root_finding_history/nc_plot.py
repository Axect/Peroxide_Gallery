from netCDF4 import Dataset
import matplotlib.pyplot as plt

# Import netCDF file
ncfile = 'data/history.nc'
data = Dataset(ncfile)
var = data.variables

# Use latex
plt.rc('text', usetex=True)
plt.rc('font', family='serif')

# Prepare Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"Root Finding", fontsize=16)
plt.xlabel(r'$x$', fontsize=14)
plt.ylabel(r'$y$', fontsize=14)

# Prepare Data to Plot
x = var['x'][:]
y = var['y'][:]  
root_x = var['root_x'][:]
root_y = var['root_y'][:]

# Plot with Legends
plt.plot(x, y, label=r'$h(x) = f(x) - g(x)$')
plt.plot(root_x, root_y, 'r.', label=r"Newton's Method")

# Other options
plt.legend(fontsize=12)
plt.grid()
plt.xlim([0.5, 1.5])
plt.ylim([-1, 1])
plt.savefig("plot.png", dpi=300)
