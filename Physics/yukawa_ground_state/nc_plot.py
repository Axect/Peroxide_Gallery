from netCDF4 import Dataset
import matplotlib.pyplot as plt

# Import netCDF file
ncfile = 'data/newton.nc'
data = Dataset(ncfile)
var = data.variables

# Use latex
plt.rc('text', usetex=True)
plt.rc('font', family='serif')

# Prepare Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"Yukawa Potential Ground state", fontsize=16)
plt.xlabel(r'$r_0/\lambda$', fontsize=14)
plt.ylabel(r'$\frac{E}{mc^2}$', fontsize=14)

# Prepare Data to Plot
x = var['r0'][:]
y1 = var['E'][:]  
y2 = var['EN'][:]

# Plot with Legends
plt.plot(x, y1, label=r'Taylor', alpha=0.5, color='blue')
plt.plot(x, y2, label=r'Newton', alpha=0.5, color='red')

# Other options
plt.legend(fontsize=12)
plt.grid()
plt.savefig("plot.png", dpi=300)
