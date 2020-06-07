from netCDF4 import Dataset
import matplotlib.pyplot as plt

# Import netCDF file
ncfile = 'data/least_square.nc'
data = Dataset(ncfile)
var = data.variables

# Use latex
plt.rc('text', usetex=True)
plt.rc('font', family='serif')

# Prepare Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"Least Square", fontsize=16)
plt.xlabel(r'$x$', fontsize=14)
plt.ylabel(r'$y$', fontsize=14)

# Prepare Data to Plot
x1 = var['x1'][:]
y1 = var['y1'][:]  
x2 = var['x2'][:]
y2 = var['y2'][:]
domain = var['domain'][:]
fitted = var['fitted'][:]

# Plot with Legends
plt.scatter(x1, y1, label=r'Group1')
plt.scatter(x2, y2, label=r'Group2')
plt.plot(domain, fitted, label=r'Boundary')

# Other options
plt.legend(fontsize=12)
plt.grid()
plt.savefig("plot/least_square.png", dpi=300)
