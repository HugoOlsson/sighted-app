import numpy as np
from typing import List
from sighted import Point2D, lines, sighted, sighted_setup, sighted_wait

def points_from_arrays(x: np.ndarray, y: np.ndarray) -> List[Point2D]:
    """Convert 2D NumPy arrays to list of Point2D objects"""
    return [Point2D(float(x_val), float(y_val)) 
            for x_val, y_val in zip(x.flatten(), y.flatten())]

def generate_lines():
    # Create base vector using numpy
    x = np.linspace(0, 2 * np.pi * 5, 1000)
    
    for time in range(200):
        time_factor = time / 50.0
        noise_factor = 1 / (time / 20.0 + 0.1)
        
        lines_data = []
        for i in range(2):
            modified_values = x + i + time_factor
            y = np.cos(modified_values) * np.cos(np.log10(x)) + \
                np.random.random(x.shape) * noise_factor
            lines_data.append(points_from_arrays(x, y))
            
        sighted(lines(lines_data, None), "line-chart-id")

def generate_damped_oscillations():
    num_points = 1000
    time_max = 10.0
    t = np.linspace(0, time_max, num_points)
    
    for frame in range(200):
        gamma = 0.1 + (frame / 200) * 0.5
        lines_data = []
        
        for i in range(2):
            A = 1.0 + i * 0.5
            omega = 2.0 * np.pi * (1.0 + i)
            
            y = A * np.exp(-gamma * t) * np.cos(omega * t)
            lines_data.append(points_from_arrays(t, y))
            
        sighted(lines(lines_data, None), "damped-oscillation-id")

def generate_fourier_square_wave():
    num_points = 1000
    t = np.linspace(0, 2*np.pi, num_points)
    
    for frame in range(1, 101):
        lines_data = []
        
        for i in range(1, 3):
            n = 2 * i - 1  # Only odd harmonics
            y = (4.0 / np.pi) * (1.0 / n) * np.sin(n * t)
            lines_data.append(points_from_arrays(t, y))
            
        sighted(lines(lines_data, None), "fourier_square_wave_frame")

def generate_sine_cosine_waves():
    num_points = 1000
    x = np.linspace(0, 4*np.pi, num_points)
    
    for frame in range(200):
        frame_factor = frame / 200
        lines_data = []
        
        for i in range(2):
            frequency = 1.0 + frame_factor * 5.0 + i
            amplitude = 1.0 + i * 0.5
            phase_shift = frame_factor * np.pi
            
            y = amplitude * np.sin(frequency * x + phase_shift)
            
            if i == 1:  # Add noise to second line
                y += np.random.uniform(-0.1, 0.1, num_points)
                
            lines_data.append(points_from_arrays(x, y))
            
        sighted(lines(lines_data, None), "sine_cosine_group")

def generate_spiral_waves():
    base_vector = np.linspace(0, 2*np.pi * 3, 1000)
    
    for time in range(200):
        angle = base_vector + (time * 0.02)
        lines_data = []
        
        for i in range(2):
            radius = np.sqrt(base_vector) + (i * 0.5)
            x = np.cos(angle) * radius
            y = np.sin(angle) * radius + np.random.random(angle.shape) * 0.1
            lines_data.append(points_from_arrays(x, y))
            
        sighted(lines(lines_data, None), "spiral-waves-chart")

def generate_harmonic_waves():
    x = np.linspace(-np.pi, np.pi, 1000)
    
    for time in range(200):
        t = time * 0.05
        lines_data = []
        
        for i in range(3):
            freq = 1.0 + (i * 0.5)
            amplitude = 1.0 / (freq + 1.0)
            phase = t * freq
            
            y = amplitude * np.sin(x * freq + phase) + \
                amplitude * 0.5 * np.cos(x * freq * 2.0 - phase) + \
                np.random.random(x.shape) * 0.01
                
            lines_data.append(points_from_arrays(x, y))
            
        sighted(lines(lines_data, None), "harmonic-waves-chart")

def generate_circular_dance():
    theta = np.linspace(0, 2*np.pi, 1000)
    
    for time in range(200):
        t = time * 0.03
        lines_data = []
        
        for i in range(4):
            radius = 1.0 + (i * 0.5)
            x = radius * np.cos(theta + t) + np.random.uniform(-0.05, 0.05, theta.shape)
            y = radius * np.sin(theta + t) + np.random.uniform(-0.05, 0.05, theta.shape)
            lines_data.append(points_from_arrays(x, y))
            
        sighted(lines(lines_data, None), "circular-dance-chart")

def generate_fractal_flow():
    x = np.linspace(-2*np.pi, 2*np.pi, 1000)
    
    for time in range(200):
        t = time * 0.03
        lines_data = []
        
        for i in range(4):
            scale = 1.0 + (i * 0.5)
            y = np.zeros_like(x)
            
            # Vectorized fractal pattern generation
            for j in range(1, 5):
                freq = j * scale
                y += np.sin(x * freq + t * freq) / freq
                y *= 1.0 + np.cos(x * 0.1) * 0.2
                
            y += np.random.random(x.shape) * 0.05 / (scale + 0.5)
            lines_data.append(points_from_arrays(x, y))
            
        sighted(lines(lines_data, None), "fractal-flow-chart")

def generate_quantum_waves():
    x = np.linspace(-3*np.pi, 3*np.pi, 1200)
    
    for time in range(200):
        t = time * 0.04
        lines_data = []
        
        for i in range(3):
            packet_width = 1.0 + (i * 0.5)
            
            # Gaussian envelope (vectorized)
            envelope = np.exp(-x**2 / (2 * packet_width))
            
            # Wave packet parameters
            k = 2.0 + (i * 0.5)  # Wave number
            omega = k * k * 0.5   # Angular frequency
            phase = k * x - omega * t
            
            # Combine components (vectorized)
            y = envelope * np.cos(phase) * (1.0 + 0.2 * np.sin(t * 0.5))
            noise = np.random.random(x.shape) * 0.02 * envelope
            
            lines_data.append(points_from_arrays(x, y + noise))
            
        sighted(lines(lines_data, None), "quantum-waves-chart")