from netCDF4 import Dataset
import matplotlib.pyplot as plt

# Import netCDF file
ncfile1 = './data/data.nc'
data1 = Dataset(ncfile1)
var1 = data1.variables

var_vec = []
for i in range(1,11):
    ncfile = './data/par/reg_lam_{}.nc'.format(i)
    data = Dataset(ncfile)
    var = data.variables
    var_vec.append(var)

# Use latex
plt.rc('text', usetex=True)
plt.rc('font', family='serif')

# Prepare Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"Non linear regression", fontsize=16)
plt.xlabel(r'$x$', fontsize=14)
plt.ylabel(r'$y$', fontsize=14)

# Prepare Data to Plot
x1 = var1['x'][:]
y1 = var1['y'][:]

# Plot with Legends
plt.scatter(x1, y1, label='data')
for i in range(1,11):
    var = var_vec[i-1]
    x = var['x']
    y = var['y']
    plt.plot(x, y, label=r'$\lambda = {}$'.format(i))

# Other options
plt.legend(fontsize=12)
plt.grid()
plt.savefig("plot/plot_par_10.png", dpi=300)
