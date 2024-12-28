use std::f32::consts::PI;

use crate::{
    helpers::grid,
    protocols::{GridData, RGB},
    sighted,
};

pub fn heat_maps() {
    let num_width = 500;
    let num_height = 500;
    let grid_vector: GridData = vec![vec![RGB::from_intensity(0.0); num_width]; num_width];

    let middle_x = num_width / 2 as usize;
    let middle_y = num_height / 2 as usize;
    let max_distance = (middle_x.max(middle_y).pow(2) as f32).sqrt();

    let vector_range = 20.0 * PI;
    let num_frames = 100;
    let vector: Vec<f32> = (0..=num_frames).map(|v| v as f32 * vector_range / num_frames as f32).collect();
    vector.iter().for_each(|time| {
        let value: GridData = grid_vector
            .iter()
            .enumerate()
            .map(|(y, row)| {
                //println!("TIME: {}", time);
                return row
                    .iter()
                    .enumerate()
                    .map(move |(x, _)| {
                        let x_diff = middle_x.abs_diff(x).pow(2) as f32;
                        let y_diff = middle_y.abs_diff(y).pow(2) as f32;
                        let intensity = f32::sin(-time + 60.0 * f32::sin(*time / 10.0) * PI * (x_diff + y_diff).sqrt() / max_distance);
                        return RGB::newf32(1.0, 1.0 - (x_diff + y_diff).sqrt() / max_distance, intensity);
                    })
                    .collect();
            })
            .collect();
        sighted(grid(&value), "heat-map");
    });
}

// Rippling waves with interference patterns
pub fn interference_grid() {
    let num_width = 1000;
    let num_height = 1000;
    let grid_vector: GridData = vec![vec![RGB::from_intensity(0.0); num_width]; num_height];

    let sources = vec![(250.0, 250.0), (750.0, 750.0), (250.0, 750.0), (750.0, 250.0)];

    let num_frames = 100;
    let vector: Vec<f32> = (0..=num_frames).map(|v| v as f32 * 4.0 * PI / num_frames as f32).collect();

    vector.iter().for_each(|time| {
        let value: GridData = grid_vector
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, _)| {
                        let mut intensity = 0.0;
                        for (sx, sy) in &sources {
                            let dx = (x as f32 - sx).abs();
                            let dy = (y as f32 - sy).abs();
                            let distance = (dx * dx + dy * dy).sqrt();
                            intensity += (distance * 0.1 - time).sin() / (1.0 + distance * 0.05);
                        }
                        intensity = intensity * 0.5 + 0.5; // Normalize

                        RGB::newf32(intensity, 1.0 - intensity, (intensity * 2.0 * PI).sin() * 0.5 + 0.5)
                    })
                    .collect()
            })
            .collect();
        sighted(grid(&value), "interference-grid");
    });
}

// Cellular automaton inspired pattern
pub fn cellular_grid() {
    let num_width = 1000;
    let num_height = 1000;
    let grid_vector: GridData = vec![vec![RGB::from_intensity(0.0); num_width]; num_height];

    let num_frames = 100;
    let vector: Vec<f32> = (0..=num_frames).map(|v| v as f32 * 2.0 * PI / num_frames as f32).collect();

    vector.iter().for_each(|time| {
        let value: GridData = grid_vector
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, _)| {
                        let cell_size = 20.0;
                        let x_cell = (x as f32 / cell_size).floor();
                        let y_cell = (y as f32 / cell_size).floor();

                        let pattern = (x_cell + y_cell + time * 5.0).sin() * (x_cell * y_cell * 0.1 + time * 2.0).cos();

                        let hue = (pattern * 0.5 + 0.5) * 2.0 * PI;

                        RGB::newf32(
                            hue.sin() * 0.5 + 0.5,
                            (hue + 2.0 * PI / 3.0).sin() * 0.5 + 0.5,
                            (hue + 4.0 * PI / 3.0).sin() * 0.5 + 0.5,
                        )
                    })
                    .collect()
            })
            .collect();
        sighted(grid(&value), "cellular-grid");
    });
}

pub fn fractal_grid() {
    const NUM_WIDTH: usize = 1000;
    const NUM_HEIGHT: usize = 1000;
    const NUM_FRAMES: usize = 2;

    // Pre-calculate constants
    const TARGET_X: f32 = -0.7435685;
    const TARGET_Y: f32 = 0.131405;

    // Single allocation for the grid
    let grid_vector: GridData = vec![vec![RGB::from_intensity(0.0); NUM_WIDTH]; NUM_HEIGHT];

    // Pre-calculate time values and store sine values
    let vector: Vec<(f32, f32)> = (0..=NUM_FRAMES)
        .map(|v| {
            let time = v as f32 * 2.0 * PI / NUM_FRAMES as f32;
            (time, (time * 2.0).sin())
        })
        .collect();

    vector.iter().for_each(|&(time, time_sin)| {
        let zoom = time.exp();
        let spiral_radius = 1.0 / (1.0 + time * 0.1);
        let spiral_angle = time * 3.0;
        let center_x = TARGET_X + spiral_radius * spiral_angle.cos() / zoom;
        let center_y = TARGET_Y + spiral_radius * spiral_angle.sin() / zoom;

        // Pre-calculate zoom-related values
        let max_iter = 500 + (zoom.ln() * 20.0) as i32;
        let inv_zoom = 1.0 / zoom;
        let s = 0.8 + 0.2 * time_sin;

        let value: GridData = grid_vector
            .iter()
            .enumerate()
            .map(|(y, row)| {
                let y_scaled_base = (y as f32 / NUM_HEIGHT as f32) * 4.0 - 2.0;

                row.iter()
                    .enumerate()
                    .map(|(x, _)| {
                        let x_scaled = ((x as f32 / NUM_WIDTH as f32) * 4.0 - 2.0) * inv_zoom + center_x;
                        let y_scaled = y_scaled_base * inv_zoom + center_y;

                        let mut zx = 0.0;
                        let mut zy = 0.0;
                        let mut iter = 0;

                        while iter < max_iter {
                            let zx2 = zx * zx;
                            let zy2 = zy * zy;

                            if zx2 + zy2 > 4.0 {
                                break;
                            }

                            zy = 2.0 * zx * zy + y_scaled;
                            zx = zx2 - zy2 + x_scaled;
                            iter += 1;
                        }

                        if iter == max_iter {
                            RGB::newf32(0.0, 0.0, 0.0)
                        } else {
                            let smooth_iter = iter as f32 + 1.0 - (zx * zx + zy * zy).ln().ln() / 2.0_f32.ln();
                            let normalized = smooth_iter / max_iter as f32;
                            let h = ((normalized * 8.0 + time * 0.2) % 1.0) * 6.0;
                            let i = h.floor();
                            let f = h - i;
                            let v = 1.0;

                            match i as i32 % 6 {
                                0 => RGB::newf32(v, v * f, v * (1.0 - s)),
                                1 => RGB::newf32(v * (1.0 - f), v, v * (1.0 - s)),
                                2 => RGB::newf32(v * (1.0 - s), v, v * f),
                                3 => RGB::newf32(v * (1.0 - s), v * (1.0 - f), v),
                                4 => RGB::newf32(v * f, v * (1.0 - s), v),
                                _ => RGB::newf32(v, v * (1.0 - s), v * (1.0 - f)),
                            }
                        }
                    })
                    .collect()
            })
            .collect();

        sighted(grid(&value), "fractal-grid");
    });
}

// Flowing vector field visualization
pub fn vector_field_grid() {
    let num_width = 1000;
    let num_height = 1000;
    let grid_vector: GridData = vec![vec![RGB::from_intensity(0.0); num_width]; num_height];

    let num_frames = 100;
    let vector: Vec<f32> = (0..=num_frames).map(|v| v as f32 * 2.0 * PI / num_frames as f32).collect();

    vector.iter().for_each(|time| {
        let value: GridData = grid_vector
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, _)| {
                        let x_norm = x as f32 / num_width as f32 - 0.5;
                        let y_norm = y as f32 / num_height as f32 - 0.5;

                        // Calculate vector field
                        let angle = (x_norm * y_norm * 10.0 + time).sin() * PI;
                        let magnitude = (x_norm * x_norm + y_norm * y_norm).sqrt();

                        let flow_x = angle.cos();
                        let flow_y = angle.sin();

                        let intensity = ((x_norm * flow_x + y_norm * flow_y) * 4.0 + time * 2.0).sin() * 0.5 + 0.5;
                        let phase = (magnitude * 10.0 - time * 3.0).sin() * 0.5 + 0.5;

                        RGB::newf32(intensity, phase, (intensity + phase) * 0.5)
                    })
                    .collect()
            })
            .collect();
        sighted(grid(&value), "vector-field-grid");
    });
}
