use line_drawing::{BresenhamCircle, Midpoint, Bresenham};

use crate::{geometry::*, camera};
use crate::vec::{Vec2, Point2, Vec3, Point3};
use crate::ray::*;
use crate::traceable::*;
use crate::bvh::*;
use crate::material::*;
use crate::camera::*;

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

    pub fn project(&self, cam: &Camera) -> Option<Circle>{
        let point = self.center + self.radius * cam.orientation.u;
        let line = Line3::new(self.center, point);
        if let Some(projected_line) = line.project(Plane::new(cam.orientation, cam.lower_left_corner), cam.origin) {
            let projected_origin = projected_line[0];
            let projected_radius = projected_line.length();
            Some(Circle::new(projected_origin, projected_radius))
        }
        else {
            None
        }
        
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

impl Outline for Sphere {
    fn outline(&self, cam: &Camera) -> Option<Vec<[usize; 2]>>{
        //Check if the sphere is at least partially in front of the camera window
        let camera_plane = Plane::new(cam.orientation, cam.origin);
        let in_front = self.center.is_in_front(camera_plane);
        let distance_to_plane = self.center.distance_to_plane(camera_plane);
        if self.radius == 1000.0 {
            let a = 1;
        }
        if !self.center.is_in_front(camera_plane) && self.radius < self.center.distance_to_plane(camera_plane) {
            return None;
        }

        if let Some(mut circle) = self.project(cam) {
            let scale = cam.resoloution.1 as f64/ cam.vertical.length();
            circle = circle.scale(scale);
            let x = circle.center.x().round() as i32;
            let y = circle.center.y().round() as i32;
            let radius = circle.radius.round() as i32;
            let pixels = BresenhamCircle::new(x, y, radius).filter(|(x, y)| *x > 0 && *x < (cam.horizontal.length() * scale) as i32 && *y > 0 && *y < (cam.vertical.length() * scale) as i32)
                                                                                             .map(|(x, y)| [x as usize, y as usize])
                                                                                             .collect();                            
            Some(pixels)
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