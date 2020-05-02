from netCDF4 import Dataset
import matplotlib.pyplot as plt

# Import netCDF file
ncfile = 'data/phys.nc'
data = Dataset(ncfile)
var = data.variables

# Use latex
plt.rc('text', usetex=True)
plt.rc('font', family='serif')

# Prepare Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"Simulation", fontsize=16)
plt.xlabel(r'$t$', fontsize=14)
plt.ylabel(r'$s$', fontsize=14)

# Prepare Data to Plot
t = var['t'][:]
s = var['s'][:]  

# Plot with Legends
plt.plot(t, s, label=r'$s(t)$')

# Other options
plt.legend(fontsize=12)
plt.grid()
plt.savefig("plot.png", dpi=300)
