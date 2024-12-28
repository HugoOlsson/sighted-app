import numpy as np
from typing import List
from dataclasses import dataclass
from sighted import RGB, grid, sighted, sighted_setup, sighted_wait

def clamp(x):
    """Clamp values between 0 and 1"""
    return np.clip(x, 0, 1)

def create_rgb_grid(r: np.ndarray, g: np.ndarray, b: np.ndarray) -> List[List[RGB]]:
    """Convert RGB arrays to grid of RGB objects with proper value clamping"""
    # Clamp values between 0 and 1, then scale to 0-255
    r = (clamp(r) * 255).astype(np.uint8)
    g = (clamp(g) * 255).astype(np.uint8)
    b = (clamp(b) * 255).astype(np.uint8)
    
    return [[RGB(int(r[i, j]), int(g[i, j]), int(b[i, j]))
             for j in range(r.shape[1])]
            for i in range(r.shape[0])]

def heat_maps():
    num_width = 500
    num_height = 500
    
    # Create coordinate arrays
    y, x = np.mgrid[0:num_height, 0:num_width]
    middle_x = num_width // 2
    middle_y = num_height // 2
    max_distance = np.sqrt(max(middle_x, middle_y)**2)
    
    # Compute distances once
    x_diff = np.square(np.abs(x - middle_x)).astype(np.float32)
    y_diff = np.square(np.abs(y - middle_y)).astype(np.float32)
    distance = np.sqrt(x_diff + y_diff)
    normalized_distance = distance / max_distance
    
    for time in np.linspace(0, 20*np.pi, 100):
        # Vectorized computation
        intensity = np.sin(-time + 60.0 * np.sin(time/10.0) * np.pi * distance / max_distance)
        
        # Create RGB channels
        r = np.ones_like(intensity)
        g = 1.0 - normalized_distance
        b = (intensity + 1) / 2  # Normalize sine to 0-1 range
        
        grid_data = create_rgb_grid(r, g, b)
        sighted(grid(grid_data), "heat-map")

def interference_grid():
    num_width = 1000
    num_height = 1000
    
    # Create coordinate arrays
    y, x = np.mgrid[0:num_height, 0:num_width]
    sources = np.array([(250., 250.), (750., 750.), (250., 750.), (750., 250.)])
    
    for time in np.linspace(0, 4*np.pi, 100):
        # Initialize intensity array
        intensity = np.zeros((num_height, num_width), dtype=np.float32)
        
        # Vectorized computation for all sources
        for sx, sy in sources:
            dx = np.abs(x - sx)
            dy = np.abs(y - sy)
            distance = np.sqrt(dx**2 + dy**2)
            intensity += np.sin(distance * 0.1 - time) / (1.0 + distance * 0.05)
        
        # Normalize intensity
        intensity = intensity * 0.5 + 0.5
        
        # Create RGB channels
        r = intensity
        g = 1.0 - intensity
        b = np.sin(intensity * 2.0 * np.pi) * 0.5 + 0.5
        
        grid_data = create_rgb_grid(r, g, b)
        sighted(grid(grid_data), "interference-grid")

def cellular_grid():
    num_width = 1000
    num_height = 1000
    cell_size = 20.0
    
    # Create coordinate arrays
    y, x = np.mgrid[0:num_height, 0:num_width]
    x_cell = np.floor(x / cell_size)
    y_cell = np.floor(y / cell_size)
    
    for time in np.linspace(0, 2*np.pi, 100):
        # Compute pattern
        pattern = np.sin(x_cell + y_cell + time * 5.0) * np.cos(x_cell * y_cell * 0.1 + time * 2.0)
        hue = (pattern * 0.5 + 0.5) * 2.0 * np.pi
        
        # Create RGB channels using vectorized operations
        r = np.sin(hue) * 0.5 + 0.5
        g = np.sin(hue + 2.0 * np.pi / 3.0) * 0.5 + 0.5
        b = np.sin(hue + 4.0 * np.pi / 3.0) * 0.5 + 0.5
        
        grid_data = create_rgb_grid(r, g, b)
        sighted(grid(grid_data), "cellular-grid")

def fractal_grid():
    NUM_WIDTH = 1000
    NUM_HEIGHT = 1000
    TARGET_X = -0.7435685
    TARGET_Y = 0.131405
    
    # Create coordinate arrays once
    y, x = np.mgrid[-2:2:NUM_HEIGHT*1j, -2:2:NUM_WIDTH*1j]
    
    for time, time_sin in [(t, np.sin(2.0 * t)) for t in np.linspace(0, 2*np.pi, 2)]:
        zoom = np.exp(time)
        spiral_radius = 1.0 / (1.0 + time * 0.1)
        spiral_angle = time * 3.0
        
        center_x = TARGET_X + spiral_radius * np.cos(spiral_angle) / zoom
        center_y = TARGET_Y + spiral_radius * np.sin(spiral_angle) / zoom
        
        # Scale coordinates
        c = (x / zoom + center_x) + 1j * (y / zoom + center_y)
        
        max_iter = 500 + int(np.log(zoom) * 20.0)
        s = 0.8 + 0.2 * time_sin
        
        # Initialize arrays
        z = np.zeros_like(c, dtype=complex)
        divtime = np.full_like(x, max_iter, dtype=np.float32)
        
        # Mandelbrot iteration
        for i in range(max_iter):
            # Update z for points that haven't diverged
            mask = np.abs(z) <= 2.0
            z[mask] = z[mask]**2 + c[mask]
            
            # Record divergence time
            diverge = np.abs(z) > 2.0
            divtime[diverge & (divtime == max_iter)] = i
        
        # Color calculation
        z_mag = np.abs(z)
        smooth_iter = divtime.copy()
        not_max = divtime < max_iter
        smooth_iter[not_max] += 1.0 - np.log2(np.log2(np.maximum(z_mag[not_max], 1e-10)))
        normalized = smooth_iter / max_iter
        
        # Create mask for points inside the set (never escaped)
        inside_set = divtime == max_iter
        
        # HSV to RGB conversion for escaped points
        h = ((normalized * 8.0 + time * 0.2) % 1.0) * 6.0
        i = h.astype(int)
        f = h - i
        
        # Pre-calculate common terms
        vs = s * np.ones_like(h)
        v = np.ones_like(h)
        
        # RGB calculation with black for points inside the set
        r = np.select([
            inside_set,  # First condition: inside the set
            i % 6 == 0,
            i % 6 == 1,
            i % 6 == 2,
            i % 6 == 3,
            i % 6 == 4,
            True
        ], [
            np.zeros_like(h),  # First result: black for inside set
            v,
            v * (1.0 - f),
            v * (1.0 - s),
            v * (1.0 - s),
            v * f,
            v
        ])
        
        g = np.select([
            inside_set,  # First condition: inside the set
            i % 6 == 0,
            i % 6 == 1,
            i % 6 == 2,
            i % 6 == 3,
            i % 6 == 4,
            True
        ], [
            np.zeros_like(h),  # First result: black for inside set
            v * f,
            v,
            v,
            v * (1.0 - f),
            v * (1.0 - s),
            v * (1.0 - s)
        ])
        
        b = np.select([
            inside_set,  # First condition: inside the set
            i % 6 == 0,
            i % 6 == 1,
            i % 6 == 2,
            i % 6 == 3,
            i % 6 == 4,
            True
        ], [
            np.zeros_like(h),  # First result: black for inside set
            v * (1.0 - s),
            v * (1.0 - s),
            v * f,
            v,
            v,
            v * (1.0 - f)
        ])
        
        grid_data = create_rgb_grid(r, g, b)
        sighted(grid(grid_data), "fractal-grid")

def vector_field_grid():
    num_width = 1000
    num_height = 1000
    
    # Create normalized coordinate arrays
    y, x = np.mgrid[0:num_height, 0:num_width]
    x_norm = x / num_width - 0.5
    y_norm = y / num_height - 0.5
    
    # Precompute magnitude
    magnitude = np.sqrt(x_norm**2 + y_norm**2)
    
    for time in np.linspace(0, 2*np.pi, 100):
        # Calculate vector field
        angle = np.sin(x_norm * y_norm * 10.0 + time) * np.pi
        flow_x = np.cos(angle)
        flow_y = np.sin(angle)
        
        # Compute intensity and phase
        intensity = np.sin(x_norm * flow_x + y_norm * flow_y * 4.0 + time * 2.0) * 0.5 + 0.5
        phase = np.sin(magnitude * 10.0 - time * 3.0) * 0.5 + 0.5
        
        # Create RGB channels
        r = intensity
        g = phase
        b = (intensity + phase) * 0.5
        
        grid_data = create_rgb_grid(r, g, b)
        sighted(grid(grid_data), "vector-field-grid")