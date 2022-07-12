from netCDF4 import Dataset
import matplotlib.pyplot as plt
import seaborn as sns

# Import netCDF file
ncfile = './result.nc'
data = Dataset(ncfile)
var = data.variables

# Use latex
plt.rc('text', usetex=True)
plt.rc('font', family='serif')

# Prepare Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"End points of Random walks", fontsize=16)
plt.xlabel(r'End points', fontsize=14)
plt.ylabel(r'Density', fontsize=14)

# Prepare Data to Plot
y = var['prob'][:]  
y2 = var['randomwalk'][:]

# Plot with Legends
sns.distplot(y, label=r"Direct Sampling from $W(n)$", bins=100)
sns.distplot(y2, label=r"End points of random walks", bins=100)

# Other options
plt.legend(fontsize=12)
plt.grid()
plt.savefig("hist.png", dpi=300)
