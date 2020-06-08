from netCDF4 import Dataset
import matplotlib.pyplot as plt

# Import netCDF file
ncfile = 'data/lda.nc'
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
d = var['d'][:]
b1 = var['b1'][:]
b2 = var['b2'][:]
bf = var['bf'][:]

# Plot with Legends
plt.scatter(x1, y1, label=r'Group1', color='r', alpha=0.5)
plt.scatter(x2, y2, label=r'Group2', color='b', alpha=0.5)
plt.plot(d, b1, label=r'Boundary1', color='r', alpha=0.5)
plt.plot(d, b2, label=r'Boundary2', color='b', alpha=0.5)
plt.plot(d, bf, label=r'Fisher', color='g', alpha=0.5)

# Other options
plt.xlim((-15, 15))
plt.ylim((-15, 15))
plt.legend(fontsize=12)
plt.grid()
plt.savefig("plot/least_square.png", dpi=300)
