import numpy as np
from typing import List
from dataclasses import dataclass
from sighted import Point3D, points3d, sighted

def points_from_arrays(x: np.ndarray, y: np.ndarray, z: np.ndarray) -> List[Point3D]:
    """Convert 3D NumPy arrays to list of Point3D objects"""
    return [Point3D(float(x), float(y), float(z)) 
            for x, y, z in zip(x.flatten(), y.flatten(), z.flatten())]

def room_plot():
    # Create coordinate meshgrid
    x = np.linspace(-5*np.pi, 5*np.pi, 300)
    y = np.linspace(-5*np.pi, 5*np.pi, 300)
    X, Y = np.meshgrid(x, y)
    
    # Generate frames using vectorized operations
    offsets = np.linspace(0.1, 100*np.pi, 100)
    for offset in offsets:
        R = np.sqrt(X**2 + Y**2)
        Z = 3.0 * np.sin(offset) * np.sin(offset + R)
        
        points = points_from_arrays(X, Y, Z)
        sighted(points3d(points), "room_plot")

def animated_torus_plot():
    # Create parameter spaces
    u = np.linspace(0, 2*np.pi, 100)
    v = np.linspace(0, 2*np.pi, 100)
    frames = np.linspace(0, 2*np.pi, 60)
    R = 10.0  # Major radius
    
    # Generate frames
    for frame in frames:
        r = 3.0 + np.sin(frame)  # Oscillating minor radius
        
        # Create meshgrid for parameters
        U, V = np.meshgrid(u, v)
        
        # Vectorized computation of torus coordinates
        X = (R + r * np.cos(V)) * np.cos(U)
        Y = (R + r * np.cos(V)) * np.sin(U)
        Z = r * np.sin(V)
        
        points = points_from_arrays(X, Y, Z)
        sighted(points3d(points), "animated_torus_plot")

def mobius_strip_plot():
    # Create parameter spaces with broadcasting
    t = np.linspace(0, 2*np.pi, 200)[:, np.newaxis]
    w = np.linspace(-1.0, 1.0, 20)
    
    # Broadcasting creates a grid of points
    X = (1.0 + (w/2) * np.cos(t)/2) * np.cos(t)
    Y = (1.0 + (w/2) * np.cos(t)/2) * np.sin(t)
    Z = (w/2) * np.sin(t)/2
    
    points = points_from_arrays(X, Y, Z)
    sighted(points3d(points), "mobius_strip_plot")

def animated_spherical_grid_plot():
    # Create parameter spaces using spherical coordinates
    r = np.linspace(1, 10, 10)
    theta = np.linspace(0, np.pi, 20)
    phi = np.linspace(0, 2*np.pi, 40)
    frames = np.linspace(0, 2*np.pi, 60)
    
    # Create meshgrid for all parameters
    R, THETA, PHI = np.meshgrid(r, theta, phi, indexing='ij')
    
    # Generate frames with rotation
    for frame in frames:
        # Convert to Cartesian coordinates (vectorized)
        X = R * np.sin(THETA) * np.cos(PHI)
        Y = R * np.sin(THETA) * np.sin(PHI)
        Z = R * np.cos(THETA)
        
        # Apply rotation matrix around Y-axis (vectorized)
        Xrot = X * np.cos(frame) + Z * np.sin(frame)
        Yrot = Y
        Zrot = -X * np.sin(frame) + Z * np.cos(frame)
        
        points = points_from_arrays(Xrot, Yrot, Zrot)
        sighted(points3d(points), "animated_spherical_grid_plot")

def animated_particle_swarm_plot():
    num_particles = 100_000
    num_frames = 200
    
    # Initialize particles using NumPy arrays
    positions = np.random.uniform(-50, 50, (num_particles, 3))
    velocities = np.random.uniform(-0.1, 0.1, (num_particles, 3))
    
    for _ in range(num_frames):
        # Update positions (vectorized)
        positions += velocities
        
        # Boundary conditions (vectorized)
        positions = np.where(positions > 50, -50, positions)
        positions = np.where(positions < -50, 50, positions)
        
        # Convert to points
        points = [Point3D(float(x), float(y), float(z)) 
                 for x, y, z in positions]
        
        sighted(points3d(points), "animated_particle_swarm_plot")

def generate_fractal_cloud_points(depth: int, frame: float):
    def generate_points(origin: np.ndarray, radius: float, depth: int) -> np.ndarray:
        if depth == 0:
            return origin.reshape(1, 3)
        
        # Generate random spherical coordinates for 8 points
        n_points = 8
        theta = np.arccos(2 * np.random.random(n_points) - 1)
        phi = 2 * np.pi * np.random.random(n_points)
        r = radius * np.random.random(n_points) * 0.5
        
        # Convert to Cartesian coordinates (vectorized)
        points = np.column_stack([
            r * np.sin(theta) * np.cos(phi),
            r * np.sin(theta) * np.sin(phi),
            r * np.cos(theta)
        ])
        
        # Add origin offset
        points += origin
        
        # Recursively generate more points
        return np.vstack([generate_points(p, radius * 0.5, depth - 1)
                         for p in points])
    
    # Initial radius varies with frame
    radius = 20.0 + 5.0 * np.sin(frame * 0.1)
    points_array = generate_points(np.zeros(3), radius, depth)
    
    return [Point3D(float(x), float(y), float(z)) 
            for x, y, z in points_array]

def animated_fractal_cloud_plot():
    frames = np.linspace(0, 2*np.pi, 100)
    for frame in frames:
        points = generate_fractal_cloud_points(depth=6, frame=frame)
        sighted(points3d(points), "animated_fractal_cloud_plot")

@dataclass
class PlotParams:
    resolution: int = 300
    x_range: tuple = (-5*np.pi, 5*np.pi)
    y_range: tuple = (-5*np.pi, 5*np.pi)
    time_steps: int = 150
    time_scale: float = 3*np.pi
    wave_count: int = 4
    amplitude: float = 3.0
    frequency: float = 1.0
    turbulence: float = 0.5

def enhanced_room_plot():
    params = PlotParams()
    
    # Create coordinate meshgrid
    x = np.linspace(*params.x_range, params.resolution)
    y = np.linspace(*params.y_range, params.resolution)
    X, Y = np.meshgrid(x, y)
    R = np.sqrt(X**2 + Y**2)
    ANGLE = np.arctan2(Y, X)
    
    # Time vector
    times = np.linspace(0.1, params.time_scale, params.time_steps)
    
    for t in times:
        # Initialize z with zeros
        Z = np.zeros_like(X)
        
        # Vectorized wave computations
        for i in range(params.wave_count):
            phase = i * np.pi / params.wave_count
            freq_mult = 1.0 + i * params.frequency
            
            # Primary wave
            Z += np.sin(t + phase) * np.sin(t + R * freq_mult)
            
            # Spiral effect
            Z += np.cos(ANGLE * params.wave_count + t) * np.exp(-R * params.turbulence)
            
            # Radial waves
            Z += np.sin(R * freq_mult - t) * np.cos(phase + t * 0.5)
        
        # Normalize and apply amplitude
        Z *= params.amplitude / params.wave_count
        
        # Add turbulent noise (vectorized)
        noise = np.sin(X * params.turbulence) * np.cos(Y * params.turbulence) * np.sin(t)
        Z += noise * params.amplitude * 0.2
        
        points = points_from_arrays(X, Y, Z)
        sighted(points3d(points), "enhanced_room_plot")