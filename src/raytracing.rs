extern crate fastrand;

use crate::camera::Camera;
use crate::geometry::plane::Plane;
use crate::image::Image;
use crate::image::Pixel;
use crate::image::RaytracedImage;
use crate::rasterizing::*;
use crate::ray_color;
use crate::util::rand_double;
use crate::vec::*;
use crate::primitives::bvh::*;
use crate::material::*;
use crate::primitives::triangle::*;
use crate::primitives::*;
use crate::enum_dispatch::*;
use crate::points::{Point2, Point3};

use core::cmp::Ordering;
use std::convert::TryFrom;

#[derive (Copy, Clone)]
pub struct HitRecord{
    
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub p_err: Vec3,
}


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