use std::f32::consts::PI;

use rand::Rng;

use crate::{helpers::points3d, protocols::Point3D, sighted};

use super::helpers::vector;

pub fn room_plot() {
    let mut points: Vec<Point3D> = vec![];

    let vectorX = vector(300, -PI * 5.0, PI * 5.0);
    let vectorY = vector(300, -PI * 5.0, PI * 5.0);
    let offsets = vector(100, 0.1, PI * 100.0);

    for offset in &offsets {
        for xValue in &vectorX {
            for yValue in &vectorY {
                //sighted(text(format!("{}, {}", xValue, yValue)), "x and y");
                let point = Point3D {
                    x: *xValue,
                    y: *yValue,
                    z: 3.0 * f32::sin(*offset) * f32::sin(offset + f32::sqrt(xValue.powf(2.0) + yValue.powf(2.0))),
                };
                points.push(point)
            }
        }

        sighted(points3d(&points), "room_plot");
        points = vec![];
    }
}

pub fn animated_torus_plot() {
    let mut points: Vec<Point3D> = Vec::new();
    let num_frames = 60; // Number of frames in the animation
    let R = 10.0; // Major radius

    // Define frame parameters (e.g., minor radius oscillates with sine)
    let frames = vector(num_frames, 0.0, 2.0 * PI);

    for frame in &frames {
        let r = 3.0 + 1.0 * frame.sin(); // Minor radius oscillates between 2 and 4

        let num_steps = 100;
        let u_steps = vector(num_steps, 0.0, 2.0 * PI);
        let v_steps = vector(num_steps, 0.0, 2.0 * PI);

        for u in &u_steps {
            for v in &v_steps {
                let x = (R + r * v.cos()) * u.cos();
                let y = (R + r * v.cos()) * u.sin();
                let z = r * v.sin();
                points.push(Point3D { x, y, z });
            }
        }

        sighted(points3d(&points), "animated_torus_plot");
        points = vec![]; // Clear points for the next frame
    }
}

pub fn mobius_strip_plot() {
    let mut points: Vec<Point3D> = Vec::new();
    let num_steps = 200;
    let width = 2.0;

    let t_steps = vector(num_steps, 0.0, 2.0 * PI);
    let w_steps = vector(20, -width / 2.0, width / 2.0);

    for t in &t_steps {
        for w in &w_steps {
            let x = (1.0 + (w / 2.0) * t.cos() / 2.0) * t.cos();
            let y = (1.0 + (w / 2.0) * t.cos() / 2.0) * t.sin();
            let z = (w / 2.0) * t.sin() / 2.0;
            points.push(Point3D { x, y, z });
        }
    }

    sighted(points3d(&points), "mobius_strip_plot");
}

pub fn animated_spherical_grid_plot() {
    let mut points: Vec<Point3D> = Vec::new();
    let num_frames = 60; // Number of frames in the animation
    let r_max = 10.0;

    // Define frame parameters (e.g., rotation angle)
    let frames = vector(num_frames, 0.0, 2.0 * PI);

    for frame in &frames {
        // Rotation matrix around the Y-axis
        let cos_theta = frame.cos();
        let sin_theta = frame.sin();

        let num_r = 10;
        let num_theta = 20;
        let num_phi = 40;

        let r_steps = vector(num_r, 1.0, r_max);
        let theta_steps = vector(num_theta, 0.0, PI);
        let phi_steps = vector(num_phi, 0.0, 2.0 * PI);

        for r in &r_steps {
            for theta in &theta_steps {
                for phi in &phi_steps {
                    let x = r * theta.sin() * phi.cos();
                    let y = r * theta.sin() * phi.sin();
                    let z = r * theta.cos();

                    // Apply rotation around the Y-axis
                    let rotated_x = x * cos_theta + z * sin_theta;
                    let rotated_y = y;
                    let rotated_z = -x * sin_theta + z * cos_theta;

                    points.push(Point3D {
                        x: rotated_x,
                        y: rotated_y,
                        z: rotated_z,
                    });
                }
            }
        }

        sighted(points3d(&points), "animated_spherical_grid_plot");
        points = vec![]; // Clear points for the next frame
    }
}

// Particle structure
struct Particle {
    position: [f32; 3],
    velocity: [f32; 3],
}

pub fn animated_particle_swarm_plot() {
    let num_frames = 200; // Number of frames in the animation
    let num_particles = 100_000; // High number of particles for density
    let speed = 0.1; // Particle speed factor

    // Initialize particles with random positions and velocities
    let mut particles = Vec::with_capacity(num_particles);
    let mut rng = rand::thread_rng();
    for _ in 0..num_particles {
        particles.push(Particle {
            position: [rng.gen_range(-50.0..50.0), rng.gen_range(-50.0..50.0), rng.gen_range(-50.0..50.0)],
            velocity: [rng.gen_range(-speed..speed), rng.gen_range(-speed..speed), rng.gen_range(-speed..speed)],
        });
    }

    for _ in 0..num_frames {
        let mut points: Vec<Point3D> = Vec::new();
        // Update particle positions
        for particle in &mut particles {
            particle.position[0] += particle.velocity[0];
            particle.position[1] += particle.velocity[1];
            particle.position[2] += particle.velocity[2];

            // Optional: Implement boundary conditions (e.g., wrap around)
            for i in 0..3 {
                if particle.position[i] > 50.0 {
                    particle.position[i] = -50.0;
                } else if particle.position[i] < -50.0 {
                    particle.position[i] = 50.0;
                }
            }

            // Collect points for rendering
            points.push(Point3D {
                x: particle.position[0],
                y: particle.position[1],
                z: particle.position[2],
            });
        }

        // Render the current frame
        sighted(points3d(&points), "animated_particle_swarm_plot");
    }
}

// Recursive function to generate fractal points
fn generate_fractal_cloud(origin: [f32; 3], radius: f32, depth: usize, points: &mut Vec<Point3D>) {
    if depth == 0 {
        points.push(Point3D {
            x: origin[0],
            y: origin[1],
            z: origin[2],
        });
        return;
    }

    let mut rng = rand::thread_rng();
    for _ in 0..8 {
        // Subdivide into 8 octants
        let theta = rand::random::<f32>() * PI;
        let phi = rand::random::<f32>() * 2.0 * PI;
        let r = radius * rand::random::<f32>() * 0.5;

        let x = origin[0] + r * theta.sin() * phi.cos();
        let y = origin[1] + r * theta.sin() * phi.sin();
        let z = origin[2] + r * theta.cos();

        generate_fractal_cloud([x, y, z], radius * 0.5, depth - 1, points);
    }
}

pub fn animated_fractal_cloud_plot() {
    let num_frames = 100; // Number of frames in the animation
    let max_depth = 6; // Depth of recursion
    let initial_radius = 20.0; // Initial sphere radius

    for frame in 0..num_frames {
        let mut points: Vec<Point3D> = Vec::new();

        // Vary the initial radius over time to animate cloud expansion/contraction
        let radius = initial_radius + (frame as f32 / num_frames as f32 * 10.0).sin() * 5.0;

        // Generate fractal cloud
        generate_fractal_cloud([0.0, 0.0, 0.0], radius, max_depth, &mut points);

        // Render the current frame
        sighted(points3d(&points), "animated_fractal_cloud_plot");
    }
}

pub struct PlotParams {
    pub resolution: usize,   // Points per axis
    pub x_range: (f32, f32), // Min/max X values
    pub y_range: (f32, f32), // Min/max Y values
    pub time_steps: usize,   // Number of animation frames
    pub time_scale: f32,     // Speed of animation
    pub wave_count: usize,   // Number of interfering waves
    pub amplitude: f32,      // Height multiplier
    pub frequency: f32,      // Wave frequency
    pub turbulence: f32,     // Chaos factor
}

impl Default for PlotParams {
    fn default() -> Self {
        PlotParams {
            resolution: 300,
            x_range: (-PI * 5.0, PI * 5.0),
            y_range: (-PI * 5.0, PI * 5.0),
            time_steps: 150,
            time_scale: PI * 3.0,
            wave_count: 4,
            amplitude: 3.0,
            frequency: 1.0,
            turbulence: 0.5,
        }
    }
}

pub fn enhanced_room_plot() {
    let mut points: Vec<Point3D> = Vec::new();
    let params = PlotParams::default();

    // Generate coordinate vectors
    let x_vector = vector(params.resolution, params.x_range.0, params.x_range.1);
    let y_vector = vector(params.resolution, params.y_range.0, params.y_range.1);
    let time_vector = vector(params.time_steps, 0.1, params.time_scale);

    // Animation loop
    for time in &time_vector {
        for x in &x_vector {
            for y in &y_vector {
                let distance = f32::sqrt(x.powf(2.0) + y.powf(2.0));

                // Combine multiple wave patterns
                let mut z = 0.0;
                for i in 0..params.wave_count {
                    let phase = (i as f32) * PI / params.wave_count as f32;
                    let freq_mult = 1.0 + (i as f32) * params.frequency;

                    // Primary wave
                    z += f32::sin(*time + phase) * f32::sin(time + distance * freq_mult);

                    // Add spiral effect
                    let angle = y.atan2(*x);
                    z += f32::cos(angle * params.wave_count as f32 + *time) * f32::exp(-distance * params.turbulence);

                    // Add radial waves
                    z += f32::sin(distance * freq_mult - *time) * f32::cos(phase + *time * 0.5);
                }

                // Normalize and apply amplitude
                z *= params.amplitude / params.wave_count as f32;

                // Add turbulent noise
                let noise = f32::sin(x * params.turbulence) * f32::cos(y * params.turbulence) * f32::sin(*time);
                z += noise * params.amplitude * 0.2;

                points.push(Point3D { x: *x, y: *y, z });
            }
        }

        // Render frame
        sighted(points3d(&points), "enhanced_room_plot");
        points.clear();
    }
}
