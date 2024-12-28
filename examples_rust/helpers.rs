pub fn vector(num_elements: usize, start_value: f32, end_value: f32) -> Vec<f32> {
    let mut vector: Vec<f32> = vec![0.0; num_elements];

    let delta = if num_elements != 1 {
        (end_value - start_value) / (num_elements - 1) as f32
    } else {
        end_value - start_value
    };

    for i in 0..vector.len() {
        vector[i] = start_value + delta * i as f32; //Should add start_value, right?
    }

    return vector;
}
