#[macro_use] 

extern crate impl_ops;
extern crate fastrand;

mod vec;
mod ray;
mod sphere;
mod traceable;
mod camera;
mod material;
mod util;
mod bvh;

use crate::vec::*;
use crate::ray::*;
use crate::sphere::*;
use crate::traceable::*;
use crate::camera::*;
use crate::material::*;
use crate::util::*;

use std::f64::INFINITY;
use std::fs::OpenOptions;
use std::io::Write;

pub fn ray_color(r: &Ray, world: &dyn Traceable, depth: i32) -> Color {

    //If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0{
        return Color::new(0.0,0.0,0.0)
    }

    let result = world.trace(r, 0.001, INFINITY);
    match result{
        TraceResult::Scattered((attenuation, scattered)) => attenuation.elementwise_mult(&ray_color(&scattered, world, depth-1)),
        TraceResult::Absorbed => Color::new(0.0, 0.0, 0.0),
        TraceResult::Missed =>{
            let unit_dir = r.direction().unit_vector();
            let t = 0.5*(unit_dir.y() + 1.0);
            (1.0-t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0)
        }         
    }
}

fn main(){

    //Image
    const IMAGE_WIDTH:i32 = 800;
    const IMAGE_HEIGHT:i32 = ((IMAGE_WIDTH as f64)/ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    //World
    let mut world = TraceableList::new();
    let mat_ground = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    let ground = Box::new(Sphere::new(Point3::new(0.0,-1000.0,0.0), 1000.0, mat_ground));
    world.add(ground);

    for a in -11..12{
        for b in -11..12{
            let choose_mat = rand_double(0.0, 1.0);
            let center = Point3::new(a as f64 + 0.9*rand_double(0.0, 1.0), 0.2, b as f64 + 0.9*rand_double(0.0, 1.0));

            if choose_mat < 0.6{
                let albedo = Color::rand(0.0, 1.0).elementwise_mult(&Color::rand(0.0, 1.0));
                let sphere_material = Lambertian::new(albedo);
                let sphere = Box::new(Sphere::new(center, 0.2, sphere_material));
                world.add(sphere);
            } else if choose_mat < 0.9{
                let albedo = Color::rand(0.5, 1.0);
                let fuzz = rand_double(0.0, 0.5);
                let sphere_material = Metal::new(albedo, fuzz);
                let sphere = Box::new(Sphere::new(center, 0.2, sphere_material));
                world.add(sphere);
            } else {
                let sphere_material = Dielectric::new(1.5);
                let sphere = Box::new(Sphere::new(center, 0.2, sphere_material));
                world.add(sphere);
            }
        }
    }
    let mat_center = Dielectric::new(1.5);
    let mat_left = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    let mat_right = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);

    let sphere_center = Box::new(Sphere::new(Point3::new(0.0,1.0,0.0), 1.0, mat_center));
    let sphere_left = Box::new(Sphere::new(Point3::new(-4.0,1.0,0.0), 1.0, mat_left));
    let sphere_right = Box::new(Sphere::new(Point3::new(4.0,1.0,0.0), 1.0, mat_right));
    

    world.add(sphere_center);
    world.add(sphere_left);
    world.add(sphere_right);

    let world = world.to_Bvh();

    //Camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let v_up = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(look_from, look_at, v_up, 20.0, ASPECT_RATIO, aperture, dist_to_focus);

    //Render
    let path = "results.ppm";
    let mut file = OpenOptions::new()
                                    .create(true)
                                    .write(true)
                                    .open(path)
                                    .unwrap();

    write!(file, "P3\n{} {} \n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).unwrap();
    let mut perc:i32 = 0;
    println!("{}",IMAGE_WIDTH*IMAGE_HEIGHT);
    for j in 0..IMAGE_HEIGHT{
        // Loading Bar Output
        if perc != ((j as f64)/(IMAGE_HEIGHT as f64)*100.0) as i32 {
            perc = ((j as f64)/(IMAGE_HEIGHT as f64)*100.0) as i32;
            println!("{}", ((j as f64)/(IMAGE_HEIGHT as f64)*100.0) as i32);
        }
        for i in 0..IMAGE_WIDTH{
            let mut pixel_color = Color::new(0.0,0.0,0.0);
            for _ in 0..SAMPLES_PER_PIXEL{
                let u = (rand_double(0.0, 1.0) + i as f64)/(IMAGE_WIDTH as f64 - 1.0);
                let v = (rand_double(0.0, 1.0) + (IMAGE_HEIGHT - j) as f64)/((IMAGE_HEIGHT - 1) as f64);
                let r = cam.get_ray(u,v);
                pixel_color = pixel_color + ray_color(&r, &world, MAX_DEPTH);
            }
            pixel_color.write_color(&mut file, SAMPLES_PER_PIXEL);
        }
    }
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
