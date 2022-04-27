use std::f64::consts::PI;

const MACHINE_EPISOLON:f64= (std::f32::EPSILON * 0.5) as f64;

pub fn deg_to_rad(deg:f64) -> f64{
    deg*PI/180.0
}

//Generates random numbers between [min_inc, max_exc)
pub fn rand_double(min_inc: f64, max_exc: f64) -> f64{
    fastrand::f64()*(max_exc - min_inc) + min_inc
}

pub fn bound(x: f64, min: f64, max:f64) -> f64{
    if x < min{return min}
    if x > max{return max}
    x
}

pub fn import_obj(file_name: &str) -> (Vec<tobj::Model>, Option<Vec<tobj::Material>>){

    let load_options = &tobj::LoadOptions{single_index: true,
        triangulate: true,
        ignore_lines: true,
        ignore_points: true
    };
        
    let obj = tobj::load_obj(file_name,load_options);
    let (models, materials_res) = obj.expect("Invalid file name.");
    match materials_res{
        Ok(mat) => {
            if !mat.is_empty(){
                (models, Some(mat))
            }else{
                (models, None)
            }
        }
        Err(_) => (models, None)
    }
}


pub fn gamma(n: i64) -> f64{
    let n = n as f64;
    (n * MACHINE_EPISOLON)/(1.0 - n * MACHINE_EPISOLON)
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use super::*;

    #[test]
    fn test_bound(){
        let x = 10.0;
        let max_x = bound(x, 5.0, 7.0);
        let min_x = bound(x, 11.0, 14.0);
        assert_eq!(max_x, 7.0);
        assert_eq!(min_x, 11.0);
    }

    #[test]
    fn test_deg_2_rad(){
        let deg = 180.0;
        let rad = deg_to_rad(deg);
        assert_eq!(PI, rad);
    }

}
