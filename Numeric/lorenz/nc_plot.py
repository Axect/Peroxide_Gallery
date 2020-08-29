from netCDF4 import Dataset
import matplotlib.pyplot as plt

# Import netCDF file
ncfile1 = './data/euler.nc'
data1 = Dataset(ncfile1)
var1 = data1.variables
ncfile2 = './data/rk4.nc'
data2 = Dataset(ncfile2)
var2 = data2.variables

# Use latex
plt.rc('text', usetex=True)
plt.rc('font', family='serif')

# Prepare Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"Lorenz Butterfly (Euler)", fontsize=16)
plt.xlabel(r'$x$', fontsize=14)
plt.ylabel(r'$z$', fontsize=14)

# Prepare Data to Plot
x1 = var1['x'][:]
z1 = var1['z'][:]  

# Plot with Legends
plt.plot(x1, z1, label=r'Lorenz (Euler)')

# Other options
plt.legend(fontsize=12)
plt.grid()
plt.savefig("euler.png", dpi=300)

# Prepare Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"Lorenz Butterfly (RK4)", fontsize=16)
plt.xlabel(r'$x$', fontsize=14)
plt.ylabel(r'$z$', fontsize=14)

# Prepare Data to Plot
x2 = var2['x'][:]
z2 = var2['z'][:]  

# Plot with Legends
plt.plot(x2, z2, label=r'Lorenz (RK4)')

# Other options
plt.legend(fontsize=12)
plt.grid()
plt.savefig("rk4.png", dpi=300)
