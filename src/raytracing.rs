extern crate fastrand;

use crate::camera::Camera;
use crate::geometry::plane::Plane;
use crate::image::Image;
use crate::image::Pixel;
use crate::image::RaytracedImage;
use crate::rasterizing::*;
use crate::util::rand_double;
use crate::vec::*;
use crate::primitives::bvh::*;
use crate::material::*;
use crate::primitives::triangle::*;
use crate::primitives::*;
use crate::enum_dispatch::*;
use crate::points::{Point2, Point3};
use std::iter::zip;
use std::ops;

use core::cmp::Ordering;
use std::convert::TryFrom;
use std::f64::INFINITY;
use std::ops::Add;

#[derive (Copy, Clone)]
pub struct HitRecord{
    
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub p_err: Vec3,
}

const COEFFICIENTCOUNT: usize = 60;
pub struct CoefficientSpectrum {
    coefficients: [f32; COEFFICIENTCOUNT],
}

impl CoefficientSpectrum {
    pub fn new(constant: f32) -> CoefficientSpectrum {
        let coefficients = [constant; COEFFICIENTCOUNT];
        CoefficientSpectrum { coefficients } 
    }

    fn elementwise_binary_operation(&self, rhs: &CoefficientSpectrum, operation: fn(f32, f32) -> f32) -> CoefficientSpectrum {
        let mut coefficients = [0f32; COEFFICIENTCOUNT];
        for i in 0..COEFFICIENTCOUNT {
            coefficients[i] = operation(self.coefficients[i], rhs.coefficients[i]);
        }
        CoefficientSpectrum { coefficients } 
    }

    fn elementwise_binary_operation_in_place(&mut self, rhs: &CoefficientSpectrum, operation: fn(f32, f32) -> f32) {
        for i in 0..COEFFICIENTCOUNT {
            self.coefficients[i] = operation(self.coefficients[i], rhs.coefficients[i]);
        }
    }
}

impl_op_ex!(+ |lhs: CoefficientSpectrum, rhs: CoefficientSpectrum| -> CoefficientSpectrum {
        lhs.elementwise_binary_operation(&rhs, |a, b| a + b)
    }
);

impl_op_ex!(+= |lhs: &mut CoefficientSpectrum, rhs: CoefficientSpectrum| {
        lhs.elementwise_binary_operation_in_place(&rhs, |a, b| a + b);
    }
);

impl_op_ex!(- |lhs: CoefficientSpectrum, rhs: CoefficientSpectrum| -> CoefficientSpectrum {
        lhs.elementwise_binary_operation(&rhs, |a, b| a - b)
    }
);

impl_op_ex!(-= |lhs: &mut CoefficientSpectrum, rhs: CoefficientSpectrum| {
        lhs.elementwise_binary_operation_in_place(&rhs, |a, b| a - b);
    }
);

impl_op_ex!(* |lhs: CoefficientSpectrum, rhs: CoefficientSpectrum| -> CoefficientSpectrum {
        lhs.elementwise_binary_operation(&rhs, |a, b| a * b)
    }
);

impl_op_ex!(*= |lhs: &mut CoefficientSpectrum, rhs: CoefficientSpectrum| {
        lhs.elementwise_binary_operation_in_place(&rhs, |a, b| a * b);
    }
);

impl_op_ex!(/ |lhs: CoefficientSpectrum, rhs: CoefficientSpectrum| -> CoefficientSpectrum {
        lhs.elementwise_binary_operation(&rhs, |a, b| a / b)
    }
);

impl_op_ex!(/= |lhs: &mut CoefficientSpectrum, rhs: CoefficientSpectrum| {
        lhs.elementwise_binary_operation_in_place(&rhs, |a, b| a / b);
    }
);



pub enum TraceResult{
    Missed,
    Absorbed(Color),
    Scattered((Color, Ray))
}

impl HitRecord{
    pub fn new(p: Point3, normal: Vec3, t: f64, r: Ray, p_err: Vec3) -> HitRecord{
        let mut rec = HitRecord{p, normal, t, front_face: true, p_err};
        rec.set_face_normal(&r, &normal);
        rec      
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3){
        self.front_face = r.direction().dot(*outward_normal) <= 0.0;
        if self.front_face{
            self.normal = *outward_normal;
        } else{
            self.normal = -*outward_normal;
        }
    }

    pub fn p(&self) -> Vec3{
        self.p
    }

    pub fn normal(&self) -> Vec3{
        self.normal
    }

    pub fn t(&self) -> f64{
        self.t
    }

    pub fn front_face(&self) -> bool{
        self.front_face
    }
}


#[enum_dispatch]
pub trait Hit: Send + Sync{
    fn hit(&self ,r: &Ray, t_min: f64, t_max: f64) -> Option<(HitRecord, &Material)>;
    fn bounding_box(&self) -> Option<Aabb>;

    fn trace(&self, r: &Ray, t_min: f64, t_max: f64) -> TraceResult{
        if let Some((hit_rec, mat)) = self.hit(r, t_min, t_max) {
            if let Some((attenuation, scattered)) = mat.scatter(r, &hit_rec){
                TraceResult::Scattered((mat.emit() + attenuation, scattered))
            } else{
                TraceResult::Absorbed(mat.emit())
            }
        } else{
            TraceResult::Missed
        }
    }
}

#[derive (Copy, Clone, Default, PartialEq, Debug)]
pub struct Ray{
    pub orig: Point3,
    pub dir: Vec3,
}

pub enum RayPlaneIntersection {
    Ray(Ray),
    Point(Point3),
    None
}

impl Ray{
    pub fn new(origin: Point3, direction: Vec3) -> Ray{
        Ray{orig: origin, dir: direction}
    }

    pub fn origin(&self) -> Point3{
        self.orig
    }

    pub fn direction(&self) -> Vec3{
        self.dir
    }

    pub fn at(&self, t:f64) -> Vec3{
        self.orig + self.dir*t
    }

    pub fn offset_origin(&self,  p_err: Vec3, norm: Vec3) -> Ray{
        let d = norm.abs().dot(p_err);
        let mut offset = d * norm;
        if self.dir.dot(norm) < 0.0{
            offset = -offset;
        }
        Ray::new(self.orig + offset, self.dir)
    }

    pub fn plane_intersection(&self, plane: Plane) -> RayPlaneIntersection {
        let dir = self.dir;
        let plane_normal = plane.orientation.w;

        //Check if line is parallel to plane
        if dir.dot(plane_normal) == 0.0 {
            //If so, check if the line lies in the plane.
            if (plane.origin - self.orig).dot(plane_normal) == 0.0 {
                return RayPlaneIntersection::Ray(*self)
            } else {
                return RayPlaneIntersection::None
            }
        } 

        let time_of_intersection = (plane.origin - self.orig).dot(plane_normal) / (dir.dot(plane_normal));
        let intersection_point = self.orig + time_of_intersection * dir;
        RayPlaneIntersection::Point(intersection_point)
    }
}

pub fn raytrace_pixel(mut image: RaytracedImage, cam: Camera, background: Color, primitives: &Primitives, max_depth: i32, pixel_position: (usize, usize))  -> RaytracedImage {
    let image_width = image.image.image_width;
    let image_height = image.image.image_height;
    let i = pixel_position.0;
    let j = pixel_position.1;

    let u = (rand_double(0.0, 1.0) + i as f64)/(image_width as f64 - 1.0);
    let v = (rand_double(0.0, 1.0) + (image_height - j) as f64)/((image_height - 1) as f64);
    let r = cam.get_ray(u,v);
    let pixel_index = (j*image_width + i) as usize;
    image.image.pixels[pixel_index] = Pixel::new(ray_color(&r, background, primitives, max_depth), 1.0);
    
    image
}

pub fn ray_color<T>(r: &Ray, background: Color, world: &T, depth: i32) -> Color where T: Hit {

    //If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0{
        return Color::new(0.0,0.0,0.0)
    }

    let result = world.trace(r, 0.001, INFINITY);
    match result {
        TraceResult::Scattered((attenuation, scattered)) => attenuation.elementwise_mult(&ray_color(&scattered, background, world, depth-1)),
        TraceResult::Absorbed(emitted) => emitted,
        TraceResult::Missed => background      
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new(){
        let orig = Vec3::new(0.0, 0.0, 0.0);
        let dir = Vec3::new(1.0, 2.0, 3.0);
        let ray = Ray::new(orig, dir);
        assert_eq!(ray.orig, orig);
        assert_eq!(ray.dir, dir);
    }

    #[test]
    fn test_direction(){
        let orig = Vec3::new(0.0, 0.0, 0.0);
        let dir = Vec3::new(1.0, 2.0, 3.0);
        let ray = Ray::new(orig, dir);
        assert_eq!(ray.direction(), dir);
    }

    #[test]
    fn test_origin(){
        let orig = Vec3::new(0.0, 0.0, 0.0);
        let dir = Vec3::new(1.0, 2.0, 3.0);
        let ray = Ray::new(orig, dir);
        assert_eq!(ray.origin(), orig);
    }

    
    #[test]
    fn test_at(){
        let orig = Vec3::new(0.0, 0.0, 0.0);
        let dir = Vec3::new(1.0, 2.0, 3.0);
        let ray = Ray::new(orig, dir);
        let t = 2.0;
        assert_eq!(ray.at(t), orig+2.0*dir);
    }
}