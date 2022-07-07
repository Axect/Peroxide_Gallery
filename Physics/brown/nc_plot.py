from netCDF4 import Dataset
import matplotlib.pyplot as plt

# Import netCDF file
ncfile = './path.nc'
data = Dataset(ncfile)
var = data.variables

# Use latex
plt.rc('text', usetex=True)
plt.rc('font', family='serif')

# Prepare Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"Random walk", fontsize=16)
plt.xlabel(r'Trial', fontsize=14)
plt.ylabel(r'Path', fontsize=14)

# Prepare Data to Plot
x = var['trial'][:]
y = var['path'][:]  

# Plot with Legends
plt.plot(x, y) 

# Other options
plt.grid()
plt.savefig("plot.png", dpi=300)
