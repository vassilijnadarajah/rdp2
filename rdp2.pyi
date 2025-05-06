"""
## Ramer-Douglas-Peucker Algorithm

This module provides a method to simplify a given set of 2D points using the Ramer-Douglas-Peucker algorithm.

### Overview
The algorithm reduces the number of points in a curve that is approximated by a series of points. 
It does this by recursively splitting the curve and removing points that are within a specified distance (epsilon) from the line segment connecting the endpoints of the curve.
The backend is implemented in Rust for performance reasons, and the interface is provided in Python for ease of use.

### Usage
```
import rdp2
```
"""

from typing import List, Tuple, Union
import numpy as np

def rdp(points: Union[List[Tuple[float, float]], List[List[float, float]], np.ndarray], epsilon: float = 1e-3) -> List[List[float, float]]:
    """
    Simplifies a given set of 2D points using the Ramer-Douglas-Peucker algorithm.

    Parameters:
        points: A list of (x, y) tuples, a list of [x, y] lists, or a numpy array representing the points to simplify.
        epsilon: The maximum distance for a point to be considered part of the simplified curve.

    Returns:
        A simplified list of [x, y] coordinates.

    Example:
        >>> points = [(0, 0), (1, 1), (2, 2), (3, 3)]
        >>> epsilon = 0.5
        >>> simplified_points = rdp(points, epsilon)
        >>> print(simplified_points)
        [[0, 0], [3, 3]]
    """
    ...