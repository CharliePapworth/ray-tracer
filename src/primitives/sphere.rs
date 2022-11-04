use crate::camera::camera::Camera;
use crate::material::*;
use crate::nalgebra::{Point3, Similarity3, Vector3};
use crate::primitives::bvh::*;
use crate::raytracing::{HitRecord, Ray};
use crate::util::gamma;
use std::f32::consts::PI;

use super::Hit;

#[derive(Clone)]
pub struct Sphere {
    object_to_world: Similarity3<f32>,
    world_to_object: Similarity3<f32>,
    radius: f32,
    material: Material,
}

impl Sphere {
    ///Initialises a new sphere
    pub fn new(object_to_world: Similarity3<f32>, material: Material) -> Sphere {
        let world_to_object = object_to_world.inverse();
        let radius = object_to_world.scaling();
        Sphere {
            object_to_world,
            world_to_object,
            radius,
            material,
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // let oc = r.origin - self.center;
        // let a = r.direction.norm_squared();
        // let half_b = oc.dot(&r.direction);
        // let c = oc.norm_squared() - self.radius * self.radius;
        // let discriminant = half_b * half_b - a * c;
        // if discriminant < 0.0 {
        //     None
        // } else {
        //     let sqrtd = discriminant.sqrt();
        //     let mut root = (-half_b - sqrtd) / a;
        //     if root < t_min || t_max < root {
        //         root = (-half_b + sqrtd) / a;
        //         if root < t_min || t_max < root {
        //             return None;
        //         }
        //     }

        //     let t = root;
        //     let p = r.at(t);
        //     let outward_normal = (p - self.center) / self.radius;
        //     let new_rec = HitRecord::new(p, outward_normal, self.material, -r.direction, root, *r, p.coords * gamma(5));
        //     Some(new_rec)
        // }
        todo!()
    }

    fn bounding_box(&self) -> Option<AxisAlignedBoundingBox> {
        // let output_box = AxisAlignedBoundingBox::new(
        //     self.center - Vector3::<f32>::new(self.radius, self.radius, self.radius),
        //     self.center + Vector3::<f32>::new(self.radius, self.radius, self.radius),
        // );
        // Some(output_box)
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::{lambertian::Lambertian, *};

    // #[test]
    // fn test_new() {
    //     let center = Point3::<f32>::new(0.0, 0.0, 0.0);
    //     let radius = 5.0;
    //     let mat = Material::Lambertian(Lambertian::default());
    //     let s = Sphere::new(center, radius, mat);
    //     assert_eq!(s.center, Point3::<f32>::new(0.0, 0.0, 0.0));
    //     assert_eq!(s.radius, 5.0);
    // }

    // #[test]
    // fn test_hit() {
    //     //Case 1: Intersection from outside of sphere
    //     let center = Point3::<f32>::new(0.0, 0.0, 0.0);
    //     let radius = 5.0;
    //     let mat = Material::Lambertian(Lambertian::default());
    //     let s = Sphere::new(center, radius, mat);
    //     let r = Ray::new(Point3::<f32>::new(-10.0, 0.0, 0.0), Vector3::<f32>::new(1.0, 0.0, 0.0));
    //     let t_min = 0.0;
    //     let t_max = 100.0;
    //     let rec_wrapper = s.hit(&r, t_min, t_max);
    //     assert!(rec_wrapper.is_some());
    //     let rec = rec_wrapper.unwrap();
    //     assert_eq!(rec.time, 5.0);
    //     assert_eq!(rec.surface_normal, Vector3::<f32>::new(-1.0, 0.0, 0.0));
    //     assert_eq!(rec.point_in_scene, Point3::<f32>::new(-5.0, 0.0, 0.0));
    //     assert_eq!(rec.front_face, true);

    //     //Case 2: Intersection from inside of sphere
    //     let r = Ray::new(Point3::<f32>::new(1.0, 0.0, 0.0), Vector3::<f32>::new(-2.0, 0.0, 0.0));
    //     let rec_wrapper = s.hit(&r, t_min, t_max);
    //     assert!(rec_wrapper.is_some());
    //     let rec = rec_wrapper.unwrap();
    //     assert_eq!(rec.time, 3.0);
    //     assert_eq!(rec.surface_normal, Vector3::<f32>::new(1.0, 0.0, 0.0));
    //     assert_eq!(rec.point_in_scene, Point3::<f32>::new(-5.0, 0.0, 0.0));
    //     assert_eq!(rec.front_face, false);

    //     //Case 3: Intersection tangent to sphere
    //     let r = Ray::new(Point3::<f32>::new(-5.0, 5.0, 0.0), Vector3::<f32>::new(0.0, -1.0, 0.0));
    //     let rec_wrapper = s.hit(&r, t_min, t_max);
    //     assert!(rec_wrapper.is_some());
    //     let rec = rec_wrapper.unwrap();
    //     assert_eq!(rec.time, 5.0);
    //     assert_eq!(rec.surface_normal, Vector3::<f32>::new(-1.0, 0.0, 0.0));
    //     assert_eq!(rec.point_in_scene, Point3::<f32>::new(-5.0, 0.0, 0.0));
    //     assert_eq!(rec.front_face, true);

    //     //Case 4: Intersection of inverted sphere (negative radius)
    //     let s = Sphere::new(center, -radius, mat);
    //     let r = Ray::new(Point3::<f32>::new(0.0, -10.0, 0.0), Vector3::<f32>::new(0.0, 1.0, 0.0));
    //     let rec_wrapper = s.hit(&r, t_min, t_max);
    //     assert!(rec_wrapper.is_some());
    //     let rec = rec_wrapper.unwrap();
    //     assert_eq!(rec.time, 5.0);
    //     assert_eq!(rec.surface_normal, Vector3::<f32>::new(0.0, -1.0, 0.0));
    //     assert_eq!(rec.point_in_scene, Point3::<f32>::new(0.0, -5.0, 0.0));
    //     assert_eq!(rec.front_face, false);
    // }

    // #[test]
    // fn test_bounding_box() {
    //     let center = Point3::<f32>::new(0.0, -3.0, 2.0);
    //     let radius = 5.0;
    //     let mat = Material::Lambertian(Lambertian::default());
    //     let s = Sphere::new(center, radius, mat);
    //     let bb = s.bounding_box().unwrap();
    //     assert_eq!(bb.min(), Point3::<f32>::new(-5.0, -8.0, -3.0));
    //     assert_eq!(bb.max(), Point3::<f32>::new(5.0, 2.0, 7.0));
    // }
}
