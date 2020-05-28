from netCDF4 import Dataset
import matplotlib.pyplot as plt

# Import netCDF file
ncfile = './data.nc'
data = Dataset(ncfile)
var = data.variables

# Use latex
plt.rc('text', usetex=True)
plt.rc('font', family='serif')

# Prepare Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"Bramble Hilbert Lemma", fontsize=16)
plt.xlabel(r'$h$', fontsize=14)
plt.ylabel(r'Norm', fontsize=14)

# Prepare Data to Plot
h = var['h'][:]
e_l2 = var['e_l2'][:]  
e_h1 = var['e_h1'][:]  
u_h2 = var['u_h2'][:]
u_h1 = var['u_h1'][:]

# Plot with Legends
plt.plot(h, e_l2, label=r'$\Vert u - u_h \Vert_{L_2}$')
plt.plot(h, u_h2, label=r'$h^2\Vert u \Vert_{H_2}$')

# Other options
plt.gca().invert_xaxis()
plt.xscale('log')
plt.yscale('log')
plt.legend(fontsize=12)
plt.grid()
plt.savefig("plot/t2m0.png", dpi=300)

# Prepare Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"Bramble Hilbert Lemma", fontsize=16)
plt.xlabel(r'$h$', fontsize=14)
plt.ylabel(r'Norm', fontsize=14)

# Plot with Legends
plt.plot(h, e_h1, label=r'$\Vert u - u_h \Vert_{H_1}$')
plt.plot(h, u_h1, label=r'$h\Vert u \Vert_{H_2}$')

# Other options
plt.gca().invert_xaxis()
plt.xscale('log')
plt.yscale('log')
plt.legend(fontsize=12)
plt.grid()
plt.savefig("plot/t2m1.png", dpi=300)
