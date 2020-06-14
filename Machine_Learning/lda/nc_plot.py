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
plt.title(r"Linear Discriminant", fontsize=16)
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
r1 = var['r1'][:]
r2 = var['r2'][:]

# Plot with Legends
plt.scatter(x1, y1, label=r'Group1', c=r1, alpha=1, vmin=-5, vmax=5)
plt.scatter(x2, y2, label=r'Group2', c=r2, alpha=1, vmin=-5, vmax=5)
plt.plot(d, b1, label=r'$LS_1$', color='r', alpha=0.5)
plt.plot(d, b2, label=r'$LS_2$', color='b', alpha=0.5)
plt.plot(d, bf, label=r'Fisher', color='g', alpha=0.9)

# Other options
plt.xlim((-8, 8))
plt.ylim((-8, 8))
plt.legend(fontsize=12)
plt.grid()
#plt.savefig("plot/lda.png", dpi=300)
plt.savefig("plot/lda.png", dpi=300)
