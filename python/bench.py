# SPDX-License-Identifier: GPL-3.0-only
# (C) 2025 Vassilij Nadarajah
# packages@vassilijnadarajah.de

import numpy as np
import matplotlib.pyplot as plt
import rdp 
import rdp2
import time

# Example data
polygon: np.ndarray = np.genfromtxt("python/test_data.txt", delimiter=None)

# Numpy implementation of rdp algo from Hirschmann (https://github.com/fhirschmann/rdp)
print("RDP")
print("--" * 20)
start_time = time.time()
rdp_points = rdp.rdp(polygon, epsilon=1.0)
end_time = time.time()
time_numpy = (end_time - start_time) * 1_000
print(f"Time taken: {time_numpy:.3f} milliseconds\n")

# Rust implementation of rdp algo from Nadarajah (https://github.com/vassilijnadarajah/rdp2
print("RDP2")
print("--" * 20)
 
start_time = time.time()
rdp_points = rdp2.rdp(polygon, epsilon=1.0)
nice_line = np.array(polygon)
end_time = time.time()
time_rust = (end_time - start_time) * 1_000
print(f"Time taken: {time_rust:.3f} milliseconds\n")


# Plot the time results
fig, ax = plt.subplots(figsize=(5, 3))
ax.bar(["rdp (numpy backend)", "rdp2 (rust backend)"], [time_numpy, time_rust], color=['tab:blue', 'tab:orange'])
ax.set_ylabel('Runtime [ms]', fontsize=10)
ax.grid(axis='y')
ax.tick_params(axis='x', which='both',      # both major and minor ticks are affected
                        bottom=False,      # ticks along the bottom edge are off
                        top=False,         # ticks along the top edge are off
                        labelbottom=True) # labels along the bottom edge are off
plt.title('Runtime comparison between rdp and rdp2', fontsize=12)

plt.show()