use core::panic;
use std::f64::consts::PI;

use line_drawing::{BresenhamCircle, Midpoint, Bresenham};

use crate::geometry::lines::{Line3, LinePlaneIntersection};
use crate::rasterizing::*;
use crate::geometry::plane::Plane;
use crate::{geometry::*, camera};
use crate::vec::{Vec2, Vec3};
use crate::primitives::bvh::*;
use crate::material::*;
use crate::camera::*;
use crate::points::{Point2, Point3};
use crate::raytracing::{HitRecord, TraceResult, Hit, Ray, RayPlaneIntersection};


#[derive (Copy, Clone)]
pub struct Circle {
    center: Point2,
    radius: f64
}

impl Circle {
    pub fn new(center: Point2, radius: f64) -> Circle {
        Circle {center, radius}
    }

    pub fn scale(&self, scale: f64) -> Circle {
        Circle::new(self.center * scale, self.radius * scale)
    }

    pub fn draw(&self) -> Vec<((i32, i32))> {
        let mut pixels = vec![];
        for i in (-self.radius).ceil() as i32..(self.radius).ceil() as i32 {
            for j in (- self.radius).ceil() as i32..(self.radius).ceil() as i32 {
                if i * i + j * j <= (self.radius * self.radius).round() as i32 {
                    pixels.push((i + self.center[0].ceil() as i32,j + self.center[1].ceil() as i32));
                }
            }
        }
        pixels
    }
}

#[derive (Copy, Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Material
}

impl Sphere{
    pub fn new(cen: Point3, rad: f64, mat: Material) -> Sphere{
        Sphere{center: cen, radius: rad, material: mat}
    }

    pub fn center(&self) -> Point3{
        self.center
    }

    //https://stackoverflow.com/questions/21648630/radius-of-projected-sphere-in-screen-space
    //https://math.stackexchange.com/questions/1367710/perspective-projection-of-a-sphere-on-a-plane
    //https://zingl.github.io/Bresenham.pdf
    pub fn project(&self, cam: &Camera) -> Option<Vec<Line3>>{

        let mut lines = Vec::with_capacity(100);
        let camera_plane = Plane::new(cam.orientation, cam.lower_left_corner);
        let visible_plane = Plane::new(cam.orientation, cam.origin);
        //Check if sphere is completely out of view


        //Find the closest point on the sphere to the viewport
        let mut closest_point = self.center + self.radius * visible_plane.orientation.w;
        let origin_in_view = self.center.is_in_front(visible_plane);
        if !origin_in_view {
            closest_point = self.center - visible_plane.orientation.w;
        }
        let closest_point_in_view = closest_point.is_in_front(visible_plane);

        if !origin_in_view && !closest_point_in_view {
            return None;
        }
        
        let origin_in_front_of_camera = self.center.is_in_front(camera_plane);
        let closest_point_in_front_of_camera = closest_point.is_in_front(camera_plane);
        let b = !origin_in_front_of_camera || !closest_point_in_front_of_camera;

        if b == true {
            let a = 1;
        }
        
        //Calculate the projected radius
        let radius_origin_vector = self.center - cam.origin;
        let radius_origin_distance = (self.center - cam.origin).length();
        let horizon_radius = (radius_origin_distance.powi(2) - self.radius.powi(2)).sqrt() * self.radius / radius_origin_distance;
        let view_angle = (horizon_radius / self.radius).acos();
        let horizon_vector = radius_origin_vector.perpendicular(cam.orientation.w).unit_vector() * horizon_radius;
        let horizon_vector_2 = horizon_vector.perpendicular(radius_origin_vector).unit_vector() * horizon_radius;
        let horizon_center_offset = (self.radius.powi(2) - horizon_radius.powi(2)).sqrt();
        let origin_horizon_center = radius_origin_vector.unit_vector() * (radius_origin_distance - horizon_center_offset);

        let number_of_lines = 100;
        for i in 0..number_of_lines {
            let new_angle = (i as f64 + 1.0) * 2.0 * PI / (number_of_lines as f64);
            let old_angle = (i as f64) * 2.0 * PI/ (number_of_lines as f64);
            let line_start = cam.origin + origin_horizon_center + horizon_vector * f64::cos(old_angle) 
                                                                    + horizon_vector_2 * f64::sin(old_angle);

            let line_end = cam.origin + origin_horizon_center + horizon_vector * f64::cos(new_angle) 
                                                                  + horizon_vector_2 * f64::sin(new_angle);
            let line = Line3::new(line_start, line_end);
            lines.push(line);
        }


        Some(lines)

    }
}

impl Hit for Sphere{
    fn hit(&self, r:&Ray, t_min: f64, t_max: f64) -> Option<(HitRecord, &Material)> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - self.radius*self.radius;
        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0{
            None
        }else{
            let sqrtd = discriminant.sqrt();
            let mut root = (-half_b - sqrtd)/a;
            if root < t_min || t_max < root{
                root = (-half_b + sqrtd)/a;
                if root < t_min || t_max < root{
                    return None
                }
            }

            let t = root;
            let p = r.at(t);
            let outward_normal = (p - self.center)/self.radius;
            let new_rec = HitRecord::new(p, outward_normal, root, *r, Vec3::default());
            Some((new_rec, &self.material))
        }
    }

    fn bounding_box(&self) -> Option<Aabb> {
        let output_box = Aabb::new(self.center - Vec3::new(self.radius, self.radius, self.radius),
                                   self.center + Vec3::new(self.radius, self.radius, self.radius));
        Some(output_box)
    }
}

impl Rasterize for Sphere {
    fn outline(&self, cam: &Camera) -> Option<Vec<[usize; 2]>>{
        let camera_plane = Plane::new(cam.orientation, cam.origin);
        
        //Check if the sphere is at least partially in front of the camera window
        if !self.center.is_in_front(camera_plane) && self.radius < self.center.distance_to_plane(camera_plane) {
            return None;
        }

        if let Some(lines) = self.project(cam) {
            lines.outline(cam)
        } else {
            None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::*;

    #[test]
    fn test_new(){
        let center = Vec3::new(0.0, 0.0, 0.0);
        let radius = 5.0;
        let mat = Material::Lambertian(Lambertian::default());
        let s = Sphere::new(center, radius, mat);
        assert_eq!(s.center, Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(s.radius, 5.0);
    }

    #[test]
    fn test_hit(){

        //Case 1: Intersection from outside of sphere
        let center = Vec3::new(0.0, 0.0, 0.0);
        let radius = 5.0;
        let mat = Material::Lambertian(Lambertian::default());
        let s = Sphere::new(center, radius, mat);
        let r = Ray::new(Vec3::new(-10.0, 0.0, 0.0), Vec3::new( 1.0, 0.0, 0.0));
        let t_min = 0.0;
        let t_max = 100.0;
        let rec_wrapper = s.hit(&r, t_min, t_max);
        assert!(rec_wrapper.is_some());
        let (rec, _) = rec_wrapper.unwrap();
        assert_eq!(rec.t(), 5.0);
        assert_eq!(rec.normal(), Vec3::new(-1.0, 0.0, 0.0));
        assert_eq!(rec.p(), Point3::new(-5.0, 0.0, 0.0));
        assert_eq!(rec.front_face(), true);

        //Case 2: Intersection from inside of sphere
        let r = Ray::new(Vec3::new(1.0, 0.0, 0.0), Vec3::new( -2.0, 0.0, 0.0));
        let rec_wrapper = s.hit(&r, t_min, t_max);
        assert!(rec_wrapper.is_some());
        let rec = rec_wrapper.unwrap().0;
        assert_eq!(rec.t(), 3.0);
        assert_eq!(rec.normal(), Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(rec.p(), Point3::new(-5.0, 0.0, 0.0));
        assert_eq!(rec.front_face(), false);

        //Case 3: Intersection tangent to sphere
        let r = Ray::new(Vec3::new(-5.0, 5.0, 0.0), Vec3::new( 0.0, -1.0, 0.0));
        let rec_wrapper = s.hit(&r, t_min, t_max);
        assert!(rec_wrapper.is_some());
        let (rec, _) = rec_wrapper.unwrap();
        assert_eq!(rec.t(), 5.0);
        assert_eq!(rec.normal(), Vec3::new(-1.0, 0.0, 0.0));
        assert_eq!(rec.p(), Point3::new(-5.0, 0.0, 0.0));
        assert_eq!(rec.front_face(), true);

        //Case 4: Intersection of inverted sphere (negative radius)
        let s = Sphere::new(center, -radius, mat);
        let r = Ray::new(Vec3::new(0.0, -10.0, 0.0), Vec3::new( 0.0, 1.0, 0.0));
        let rec_wrapper = s.hit(&r, t_min, t_max);
        assert!(rec_wrapper.is_some());
        let (rec, _) = rec_wrapper.unwrap();
        assert_eq!(rec.t(), 5.0);
        assert_eq!(rec.normal(), Vec3::new(0.0, -1.0, 0.0));
        assert_eq!(rec.p(), Point3::new(0.0, -5.0, 0.0));
        assert_eq!(rec.front_face(), false);
    }

    #[test]
    fn test_bounding_box(){
        let center = Vec3::new(0.0, -3.0, 2.0);
        let radius = 5.0;
        let mat = Material::Lambertian(Lambertian::default());
        let s = Sphere::new(center, radius, mat);
        let bb = s.bounding_box().unwrap();
        assert_eq!(bb.min(), Point3::new(-5.0, -8.0, -3.0));
        assert_eq!(bb.max(), Point3::new(5.0, 2.0, 7.0));
    } 
}