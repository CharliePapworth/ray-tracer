use std::f32::consts::PI;

const MACHINE_EPISOLON: f32 = (std::f32::EPSILON * 0.5) as f32;

pub fn deg_to_rad(deg: f32) -> f32 {
    deg * PI / 180.0
}

//Generates random numbers between [min_inc, max_exc)
pub fn rand_double(min_inc: f32, max_exc: f32) -> f32 {
    fastrand::f32() * (max_exc - min_inc) + min_inc
}

pub fn bound_f32(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}

/// Linearly interpolates between the start and the end. The interpolation time must be between
/// 0 and 1.
///
/// # Example
/// ```
/// lerp(4.0, 5.0, 0.0) == 4.0;
/// lerp(4.0, 5.0, 0.0) == 5.0;
/// lerp(4.0, 5.0, 0.2) == 4.2;
/// ```
pub fn lerp(start: f32, end: f32, interpolation_time: f32) -> f32 {
    if interpolation_time < 0.0 || interpolation_time > 1.0 {
        panic!("interpolation_time must be between 0.0 and 1.0 (inclusive)")
    }
    (1.0 - interpolation_time) * start + interpolation_time * end
}

pub fn import_obj(file_name: &str) -> (Vec<tobj::Model>, Option<Vec<tobj::Material>>) {
    let load_options = &tobj::LoadOptions {
        single_index: true,
        triangulate: true,
        ignore_lines: true,
        ignore_points: true,
    };

    let obj = tobj::load_obj(file_name, load_options);
    let (models, materials_res) = obj.expect("Invalid file name.");
    match materials_res {
        Ok(mat) => {
            if !mat.is_empty() {
                (models, Some(mat))
            } else {
                (models, None)
            }
        }
        Err(_) => (models, None),
    }
}

pub fn gamma(n: i64) -> f32 {
    let n = n as f32;
    (n * MACHINE_EPISOLON) / (1.0 - n * MACHINE_EPISOLON)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_bound() {
        let x = 10.0;
        let max_x = bound_f32(x, 5.0, 7.0);
        let min_x = bound_f32(x, 11.0, 14.0);
        assert_eq!(max_x, 7.0);
        assert_eq!(min_x, 11.0);
    }

    #[test]
    fn test_deg_2_rad() {
        let deg = 180.0;
        let rad = deg_to_rad(deg);
        assert_eq!(PI, rad);
    }
}
