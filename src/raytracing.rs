extern crate fastrand;

use crate::camera::Camera;
use crate::geometry::plane::Plane;
use crate::image::Color;
use crate::image::Pixel;
use crate::image::RaytracedImage;
use crate::util::rand_double;
use crate::nalgebra::{Vector3, Point3};
use crate::primitives::bvh::*;
use crate::material::*;
use crate::primitives::*;
use crate::enum_dispatch::*;

use std::f32::INFINITY;

#[derive (Copy, Clone)]
pub struct HitRecord {
    
    pub point_in_scene: Point3<f32>,
    pub surface_normal: Vector3<f32>,
    pub surface_material: Material,
    pub outbound_ray_direction: Vector3<f32>,
    pub time: f32,
    pub front_face: bool,
    /// Gives a conservative bound on the error in the position of the ray-surface intersection
    pub error_bound: Vector3<f32>,
}

pub enum TraceResult{
    Missed,
    Absorbed(Color),
    Scattered((Color, Ray))
}

impl HitRecord{
    pub fn new(point_in_scene: Point3<f32>, surface_normal: Vector3<f32>, surface_material: Material, outbound_ray_direction: Vector3<f32>, time: f32, r: Ray, error_bound: Vector3<f32>) -> HitRecord{
        let mut rec = HitRecord{point_in_scene, surface_normal, surface_material, outbound_ray_direction, time, front_face: true, error_bound};
        rec.set_face_normal(&r, &surface_normal);
        rec      
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vector3<f32>){
        self.front_face = r.direction().dot(outward_normal) <= 0.0;
        if self.front_face{
            self.surface_normal = *outward_normal;
        } else{
            self.surface_normal = -*outward_normal;
        }
    }

    pub fn p(&self) -> Point3<f32>{
        self.point_in_scene
    }

    pub fn normal(&self) -> Vector3<f32>{
        self.surface_normal
    }

    pub fn t(&self) -> f32{
        self.time
    }

    pub fn front_face(&self) -> bool{
        self.front_face
    }
}


#[enum_dispatch]
pub trait Hit: Send + Sync{
    fn hit(&self ,r: &Ray, t_min: f32, t_max: f32) -> Option<(HitRecord)>;
    fn bounding_box(&self) -> Option<Aabb>;

    fn trace(&self, r: &Ray, t_min: f32, t_max: f32) -> TraceResult{
        if let Some(hit_rec) = self.hit(r, t_min, t_max) {
            let mat = hit_rec.surface_material;
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
    pub orig: Point3<f32>,
    pub dir: Vector3<f32>,
}

pub enum RayPlaneIntersection {
    Ray(Ray),
    Point(Point3<f32>),
    None
}

impl Ray{
    pub fn new(origin: Point3<f32>, direction: Vector3<f32>) -> Ray{
        Ray{orig: origin, dir: direction}
    }

    pub fn origin(&self) -> Point3<f32>{
        self.orig
    }

    pub fn direction(&self) -> Vector3<f32>{
        self.dir
    }

    pub fn at(&self, t:f32) -> Point3<f32>{
        self.orig + self.dir * t
    }

    pub fn offset_origin(&self,  p_err: Vector3<f32>, norm: Vector3<f32>) -> Ray {
        let d = norm.abs().dot(&p_err);
        let mut offset = d * norm;
        if self.dir.dot(&norm) < 0.0{
            offset = -offset;
        }
        Ray::new(self.orig + offset, self.dir)
    }

    pub fn plane_intersection(&self, plane: Plane) -> RayPlaneIntersection {
        let dir = self.dir;
        let plane_normal = plane.orientation.w;

        //Check if line is parallel to plane
        if dir.dot(&plane_normal) == 0.0 {
            //If so, check if the line lies in the plane.
            if (plane.origin - self.orig).dot(&plane_normal) == 0.0 {
                return RayPlaneIntersection::Ray(*self)
            } else {
                return RayPlaneIntersection::None
            }
        } 

        let time_of_intersection = (plane.origin - self.orig).dot(&plane_normal) / (dir.dot(&plane_normal));
        let intersection_point = self.orig + time_of_intersection * dir;
        RayPlaneIntersection::Point(intersection_point)
    }
}

pub fn raytrace_pixel(mut image: RaytracedImage, cam: Camera, background: Color, primitives: &Primitives, max_depth: i32, pixel_position: (usize, usize))  -> RaytracedImage {
    let image_width = image.image.image_width;
    let image_height = image.image.image_height;
    let i = pixel_position.0;
    let j = pixel_position.1;

    let u = (rand_double(0.0, 1.0) + i as f32)/(image_width as f32 - 1.0);
    let v = (rand_double(0.0, 1.0) + (image_height - j) as f32)/((image_height - 1) as f32);
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
        TraceResult::Scattered((attenuation, scattered)) => attenuation.component_mul(&ray_color(&scattered, background, world, depth-1)),
        TraceResult::Absorbed(emitted) => emitted,
        TraceResult::Missed => background      
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new(){
        let orig = Point3::<f32>::new(0.0, 0.0, 0.0);
        let dir = Vector3::<f32>::new(1.0, 2.0, 3.0);
        let ray = Ray::new(orig, dir);
        assert_eq!(ray.orig, orig);
        assert_eq!(ray.dir, dir);
    }

    #[test]
    fn test_direction(){
        let orig = Point3::<f32>::new(0.0, 0.0, 0.0);
        let dir = Vector3::<f32>::new(1.0, 2.0, 3.0);
        let ray = Ray::new(orig, dir);
        assert_eq!(ray.direction(), dir);
    }

    #[test]
    fn test_origin(){
        let orig = Point3::<f32>::new(0.0, 0.0, 0.0);
        let dir = Vector3::<f32>::new(1.0, 2.0, 3.0);
        let ray = Ray::new(orig, dir);
        assert_eq!(ray.origin(), orig);
    }

    
    #[test]
    fn test_at(){
        let orig = Point3::<f32>::new(0.0, 0.0, 0.0);
        let dir = Vector3::<f32>::new(1.0, 2.0, 3.0);
        let ray = Ray::new(orig, dir);
        let t = 2.0;
        assert_eq!(ray.at(t), orig + 2.0 * dir);
    }
}