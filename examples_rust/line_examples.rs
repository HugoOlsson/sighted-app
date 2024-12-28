use crate::{
    helpers::lines,
    protocols::{LinesData, Point2D},
    sighted,
};
use rand::Rng;
use std::f32::consts::PI;

use super::helpers::vector;

pub fn generate_lines() {
    let mut rng = rand::thread_rng();
    //Creating a vector
    let vector = vector(1000, 0.0, 2.0 * PI * 5.0);
    for time in 0..200 {
        let lines_data: LinesData = (0..2)
            .map(|i| {
                vector
                    .iter()
                    .map(|value| {
                        let modified_value = value + i as f32 + time as f32 / 50.0;
                        Point2D::new(*value, modified_value.cos() * value.log10().cos() + rng.gen::<f32>() / ((time as f32) / 20.0 + 0.1))
                    })
                    .collect()
            })
            .collect();
        sighted(lines(&lines_data, None), "line-chart-id");
    }
}
/// Generates damped oscillation curves with varying damping coefficients
pub fn generate_damped_oscillations() {
    let mut rng = rand::thread_rng();
    let num_frames = 200; // Total number of frames in the animation
    let num_points = 1000; // Number of points per line
    let time_max = 10.0; // Maximum time value

    for frame in 0..num_frames {
        let mut lines_data = Vec::new();

        // Generate two damped oscillation curves with different damping coefficients
        for i in 0..2 {
            // Amplitude and frequency can vary per curve
            let A = 1.0 + i as f32 * 0.5;
            let omega = 2.0 * PI * (1.0 + i as f32); // Frequency increases with i

            // Damping coefficient varies over time to animate the damping effect
            let gamma = 0.1 + (frame as f32 / num_frames as f32) * 0.5;

            // Phase shift
            let delta = 0.0;

            let mut points = Vec::with_capacity(num_points);

            for j in 0..num_points {
                let t = j as f32 / num_points as f32 * time_max;
                let y = A * (-gamma * t).exp() * (omega * t + delta).cos();
                points.push(Point2D::new(t, y));
            }

            lines_data.push(points);
        }

        // Save the lines data for the current frame
        sighted(lines(&lines_data, None), "damped-oscillation-id");
    }
}

/// Generates Fourier series approximations of a square wave with increasing harmonics
pub fn generate_fourier_square_wave() {
    let num_frames = 100; // Total number of frames in the animation
    let num_points = 1000; // Number of points per line
    let time_max = 2.0 * PI; // One period

    for frame in 1..=num_frames {
        let mut lines_data = Vec::new();

        // Each frame adds more harmonics to the approximation
        for i in 1..=2 {
            let n = 2 * i - 1; // Only odd harmonics
            let mut points = Vec::with_capacity(num_points);

            for j in 0..num_points {
                let t = j as f32 / num_points as f32 * time_max;
                let y = (4.0 / PI) * (1.0 / n as f32) * (n as f32 * t).sin();
                points.push(Point2D::new(t, y));
            }

            lines_data.push(points);
        }

        // Save the lines data for the current frame

        sighted(lines(&lines_data, None), "fourier_square_wave_frame_");
    }
}

/// Generates animated sine and cosine waves with varying frequencies and amplitudes
pub fn generate_sine_cosine_waves() {
    let mut rng = rand::thread_rng();
    let num_frames = 200; // Total number of frames in the animation
    let num_points = 1000; // Number of points per line
    let x_values: Vec<f32> = (0..num_points).map(|i| i as f32 / num_points as f32 * 4.0 * PI).collect();

    for frame in 0..num_frames {
        let mut lines_data = Vec::new();

        // Generate two lines: sine and cosine with varying parameters
        for i in 0..2 {
            // Vary frequency and amplitude over time for dynamic visualization
            let frequency = 1.0 + (frame as f32 / num_frames as f32) * 5.0 + i as f32;
            let amplitude = 1.0 + (i as f32) * 0.5;
            let phase_shift = (frame as f32 / num_frames as f32) * PI;

            let mut points = Vec::with_capacity(num_points);

            for &x in &x_values {
                let y = amplitude * (frequency * x + phase_shift).sin();
                points.push(Point2D::new(x, y));
            }

            lines_data.push(points);
        }

        // Add noise to the cosine wave for visual interest
        let noise_amplitude = 0.1;
        if let Some(cosine_wave) = lines_data.get_mut(1) {
            for point in cosine_wave.iter_mut() {
                point.y += rng.gen_range(-noise_amplitude..noise_amplitude);
            }
        }

        // Add the lines to the "sine_cosine_group"
        sighted(lines(&lines_data, None), "sine_cosine_group");
    }
}

pub fn generate_spiral_waves() {
    let mut rng = rand::thread_rng();
    let vector = vector(1000, 0.0, 2.0 * PI * 3.0);

    for time in 0..200 {
        let lines_data: LinesData = (0..2)
            .map(|i| {
                vector
                    .iter()
                    .map(|value| {
                        let angle = value + (time as f32 * 0.02);
                        let radius = value.sqrt() + (i as f32 * 0.5);
                        let x = angle.cos() * radius;
                        let y = angle.sin() * radius + rng.gen::<f32>() * 0.1;
                        Point2D::new(x, y)
                    })
                    .collect()
            })
            .collect();
        sighted(lines(&lines_data, None), "spiral-waves-chart");
    }
}

// Harmonic patterns with interference
pub fn generate_harmonic_waves() {
    let mut rng = rand::thread_rng();
    let vector = vector(1000, -PI, PI);

    for time in 0..200 {
        let t = time as f32 * 0.05;
        let lines_data: LinesData = (0..3)
            .map(|i| {
                vector
                    .iter()
                    .map(|value| {
                        let freq = 1.0 + (i as f32 * 0.5);
                        let amplitude = 1.0 / (freq + 1.0);
                        let phase = t * freq;

                        let y = amplitude * (value * freq + phase).sin() + amplitude * 0.5 * (value * freq * 2.0 - phase).cos() + rng.gen::<f32>() * 0.01;

                        Point2D::new(*value, y)
                    })
                    .collect()
            })
            .collect();
        sighted(lines(&lines_data, None), "harmonic-waves-chart");
    }
}

// Circular motion with varying radii
pub fn generate_circular_dance() {
    let mut rng = rand::thread_rng();
    let vector = vector(1000, 0.0, 2.0 * PI);

    for time in 0..200 {
        let t = time as f32 * 0.03;
        let lines_data: LinesData = (0..4)
            .map(|i| {
                let radius = 1.0 + (i as f32 * 0.5);
                vector
                    .iter()
                    .map(|value| {
                        let x = radius * (value + t).cos() + (rng.gen::<f32>() * 0.1 - 0.05);
                        let y = radius * (value + t).sin() + (rng.gen::<f32>() * 0.1 - 0.05);
                        Point2D::new(x, y)
                    })
                    .collect()
            })
            .collect();
        sighted(lines(&lines_data, None), "circular-dance-chart");
    }
}

// Fractal-like pattern with recursive modulation
pub fn generate_fractal_flow() {
    let mut rng = rand::thread_rng();
    let vector = vector(1000, -2.0 * PI, 2.0 * PI);

    for time in 0..200 {
        let t = time as f32 * 0.03;
        let lines_data: LinesData = (0..4)
            .map(|i| {
                vector
                    .iter()
                    .map(|value| {
                        let scale = 1.0 + (i as f32 * 0.5);
                        let x = value;
                        let mut y = 0.0;

                        // Create fractal-like pattern through recursive addition
                        for j in 1..5 {
                            let freq = j as f32 * scale;
                            y += (value * freq + t * freq).sin() / freq;
                            y *= 1.0 + (value * 0.1).cos() * 0.2;
                        }

                        y += rng.gen::<f32>() * 0.05 / (scale + 0.5);
                        Point2D::new(*x, y)
                    })
                    .collect()
            })
            .collect();
        sighted(lines(&lines_data, None), "fractal-flow-chart");
    }
}

// Quantum-inspired wave packet simulation
pub fn generate_quantum_waves() {
    let mut rng = rand::thread_rng();
    let vector = vector(1200, -3.0 * PI, 3.0 * PI);

    for time in 0..200 {
        let t = time as f32 * 0.04;
        let lines_data: LinesData = (0..3)
            .map(|i| {
                let packet_width = 1.0 + (i as f32 * 0.5);
                vector
                    .iter()
                    .map(|value| {
                        // Gaussian envelope
                        let envelope = (-value * value / (2.0 * packet_width)).exp();

                        // Wave packet with dispersion
                        let k = 2.0 + (i as f32 * 0.5); // Wave number
                        let omega = k * k * 0.5; // Angular frequency
                        let phase = k * value - omega * t;

                        let y = envelope * phase.cos() * (1.0 + 0.2 * (t * 0.5).sin());
                        let noise = rng.gen::<f32>() * 0.02 * envelope;

                        Point2D::new(*value, y + noise)
                    })
                    .collect()
            })
            .collect();
        sighted(lines(&lines_data, None), "quantum-waves-chart");
    }
}
