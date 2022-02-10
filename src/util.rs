use crate::vec::*;

use std::f64::INFINITY;
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
        ignore_points: true,
        ..Default::default()};
        
    let obj = tobj::load_obj(file_name,load_options);
    let (models, materials_res) = obj.expect("Invalid file name.");
    match materials_res{
        Ok(mat) => {
            if mat.len() > 0{
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
