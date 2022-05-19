from netCDF4 import Dataset
import matplotlib.pyplot as plt
from scipy.interpolate import CubicHermiteSpline, CubicSpline
import numpy as np

# Import netCDF file
ncfile = './data.nc'
data = Dataset(ncfile)
var = data.variables

# Use latex
plt.rc('text', usetex=True)
plt.rc('font', family='serif')

# Prepare Plot
plt.figure(figsize=(10,6), dpi=300)
plt.title(r"Various Cubic Splines", fontsize=16)
plt.xlabel(r'$x$', fontsize=14)
plt.ylabel(r'$y$', fontsize=14)

# Prepare Data to Plot
x = var['x'][:]
y = var['y'][:]  
new_x = var['new_x'][:]
y_cs = var['y_cs'][:]
y_akima = var['y_akima'][:]
y_quad = var['y_quad'][:]

# Plot with Legends
#cs = CubicSpline(x, y)
#cs_akima = CubicHermiteSpline(x, y, m_akima)
#cs_quad = CubicHermiteSpline(x, y, m_quad)


plt.plot(x, y, '.', label=r'Data')
#plt.plot(new_x, cs(new_x), label=r'Cubic Spline', alpha=0.5)
#plt.plot(new_x, cs_akima(new_x), label=r'Cubic Hermite Spline (Akima)', alpha=0.5)
#plt.plot(new_x, cs_quad(new_x), label=r'Cubic Hermite Spline (Quadratic)', alpha=0.5)
plt.plot(new_x, y_cs, label=r'Cubic Spline', alpha=0.5)
plt.plot(new_x, y_akima, label=r'Cubic Hermite Spline (Akima)', alpha=0.5)
plt.plot(new_x, y_quad, label=r'Cubic Hermite Spline (Quadratic)', alpha=0.5)

# Other options
plt.legend(fontsize=12)
plt.grid()
plt.savefig("plot.png", dpi=300)
