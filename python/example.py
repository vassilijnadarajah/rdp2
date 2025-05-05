# SPDX-License-Identifier: GPL-3.0-only
# (C) 2025 Vassilij Nadarajah
# packages@vassilijnadarajah.de

import numpy as np
import matplotlib.pyplot as plt
import rdp2

# Example data
line: np.ndarray = np.array([44, 95, 26, 91, 22, 90, 21, 90,
    19, 89, 17, 89, 15, 87, 15, 86, 16, 85,
    20, 83, 26, 81, 28, 80, 30, 79, 32, 74,
    32, 72, 33, 71, 34, 70, 38, 68, 43, 66,
    49, 64, 52, 63, 52, 62, 53, 59, 54, 57,
    56, 56, 57, 56, 58, 56, 59, 56, 60, 56,
    61, 55, 61, 55, 63, 55, 64, 55, 65, 54,
    67, 54, 68, 54, 76, 53, 82, 52, 84, 52,
    87, 51, 91, 51, 93, 51, 95, 51, 98, 50,
    105, 50, 113, 49, 120, 48, 127, 48, 131, 47,
    134, 47, 137, 47, 139, 47, 140, 47, 142, 47,
    145, 46, 148, 46, 152, 46, 154, 46, 155, 46,
    159, 46, 160, 46, 165, 46, 168, 46, 169, 45,
    171, 45, 173, 45, 176, 45, 182, 45, 190, 44,
    204, 43, 204, 43, 207, 43, 215, 40, 215, 38,
    215, 37, 200, 37, 195, 41]).reshape(77, 2)
polygon: np.ndarray = np.genfromtxt("python/test_data.txt", delimiter=None)

# Apply rdp
rdp_line = rdp2.rdp(line, epsilon=1.0)
rdp_line = np.array(rdp_line)
rdp_polygon = rdp2.rdp(polygon, epsilon=0.1)
rdp_polygon = np.array(rdp_polygon)

# Plot the line
plt.figure(1)  
plt.plot(line[:, 0], line[:, 1], "ro-", label='Source data')
plt.plot(rdp_line[:, 0], rdp_line[:, 1], "bo-", markersize=8, label='RDP line')
plt.xlabel('X-axis')
plt.ylabel('Y-axis')
plt.title('Example Line')
plt.legend()
plt.grid()

# Plot the polygon
plt.figure(2) 
plt.plot(polygon[:, 0], polygon[:, 1], "ro-", label='Source data')
plt.plot(rdp_polygon[:, 0], rdp_polygon[:, 1], "bo-", markersize=8, label='RDP polygon')
plt.xlabel('X-axis')
plt.ylabel('Y-axis')
plt.title('Example Polygon')
plt.legend()
plt.grid()

# Show the plot
plt.show()