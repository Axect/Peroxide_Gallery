from netCDF4 import Dataset
import matplotlib.pyplot as plt

# Use latex
plt.rc('text', usetex=True)
plt.rc('font', family='serif')

# Import netCDF file
ncfile = './data/newmark_beta.nc'
data = Dataset(ncfile)
var = data.variables

# Prepare Data to Plot
t = var['t'][:]
x = var['x'][:]
v = var['v'][:]  
a = var['a'][:]

# Prepare Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"Newmark Beta method for Damped SHO", fontsize=16)
plt.xlabel(r'$t$', fontsize=14)
plt.ylabel(r'$x$', fontsize=14)

# Plot with Legends
plt.plot(t, x, label=r'Position')
#plt.plot(t, v, label=r'Velocity')
#plt.plot(t, a, label=r'Acceleration')

# Other options
plt.legend(fontsize=12)
plt.grid()
plt.savefig("position.png", dpi=300)

# Prepare Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"Newmark Beta method for Damped SHO", fontsize=16)
plt.xlabel(r'$t$', fontsize=14)
plt.ylabel(r'$v$', fontsize=14)

# Plot with Legends
#plt.plot(t, x, label=r'Position')
plt.plot(t, v, label=r'Velocity')
#plt.plot(t, a, label=r'Acceleration')

# Other options
plt.legend(fontsize=12)
plt.grid()
plt.savefig("velocity.png", dpi=300)

# Prepare Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"Newmark Beta method for Damped SHO", fontsize=16)
plt.xlabel(r'$t$', fontsize=14)
plt.ylabel(r'$a$', fontsize=14)

# Plot with Legends
#plt.plot(t, x, label=r'Position')
#plt.plot(t, v, label=r'Velocity')
plt.plot(t, a, label=r'Acceleration')

# Other options
plt.legend(fontsize=12)
plt.grid()
plt.savefig("acceleration.png", dpi=300)

# Prepare Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"Newmark Beta method for Damped SHO", fontsize=16)
plt.xlabel(r'$t$', fontsize=14)
plt.ylabel(r'$E$', fontsize=14)

# Plot with Legends
#plt.plot(t, x, label=r'Position')
#plt.plot(t, v, label=r'Velocity')
#plt.plot(t, a, label=r'Acceleration')
plt.plot(t, 0.5*(v**2) + 0.5*200*x**2, label=r'Energy')

# Other options
plt.legend(fontsize=12)
plt.grid()
plt.savefig("energy.png", dpi=300)

