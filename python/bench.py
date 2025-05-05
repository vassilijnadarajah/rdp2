import numpy as np
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
print(f"Time taken: {(end_time - start_time) * 1_000_000:.2f} microseconds\n")

# Rust implementation of rdp algo from Nadarajah (https://github.com/vassilijnadarajah/rdp2
print("RDP2")
print("--" * 20)
 
start_time = time.time()
rdp_points = rdp2.rdp(polygon, epsilon=1.0)
nice_line = np.array(polygon)
end_time = time.time()
print(f"Time taken: {(end_time - start_time) * 1_000_000:.2f} microseconds\n")