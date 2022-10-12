extern crate fastrand;

use crate::enum_dispatch::*;
use crate::camera::Color;
use crate::material::*;
use crate::nalgebra::{Point3, UnitVector3, Vector3};
use crate::primitives::bvh::*;
use crate::primitives::*;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub point_in_scene: Point3<f32>,
    pub surface_normal: Vector3<f32>,
    pub surface_material: Material,
    pub outbound_ray_direction: UnitVector3<f32>,
    pub time: f32,
    pub front_face: bool,
    /// Gives a conservative bound on the error in the position of the
    /// ray-surface intersection
    pub error_bound: Vector3<f32>,
}

pub enum TraceResult {
    Missed,
    Absorbed(Color),
    Scattered((Color, Ray)),
}

impl HitRecord {
    pub fn new(
        point_in_scene: Point3<f32>,
        surface_normal: Vector3<f32>,
        surface_material: Material,
        outbound_ray_direction: UnitVector3<f32>,
        time: f32,
        r: Ray,
        error_bound: Vector3<f32>,
    ) -> HitRecord {
        let mut rec = HitRecord {
            point_in_scene,
            surface_normal,
            surface_material,
            outbound_ray_direction,
            time,
            front_face: true,
            error_bound,
        };
        rec.set_face_normal(&r, &surface_normal);
        rec
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vector3<f32>) {
        self.front_face = r.direction().dot(outward_normal) <= 0.0;
        if self.front_face {
            self.surface_normal = *outward_normal;
        } else {
            self.surface_normal = -*outward_normal;
        }
    }

    /// Spawns a ray from the intersection point in a given direction,
    /// accounting for error bounds in the intersection.
    pub fn spawn_ray(&self, direction: UnitVector3<f32>) -> Ray {
        let offset = Ray::offset_origin(self.error_bound, self.surface_normal, direction);
        Ray::new(self.point_in_scene + offset, direction)
    }
}

#[enum_dispatch]
pub trait Hit: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<(HitRecord)>;
    fn bounding_box(&self) -> Option<Aabb>;
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Ray {
    pub orig: Point3<f32>,
    pub dir: UnitVector3<f32>,
}

impl Ray {
    pub fn new(origin: Point3<f32>, direction: UnitVector3<f32>) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> Point3<f32> {
        self.orig
    }

    pub fn direction(&self) -> UnitVector3<f32> {
        self.dir
    }

    pub fn at(&self, t: f32) -> Point3<f32> {
        self.orig + self.dir.into_inner() * t
    }

    /// Calculates the offset in the origin of the ray based on the error-bound
    /// of the intersection point, the surface normal and the direction of
    /// the ray.
    pub fn offset_origin(error_bound: Vector3<f32>, norm: Vector3<f32>, direction: UnitVector3<f32>) -> Vector3<f32> {
        let d = norm.abs().dot(&error_bound);
        let mut offset = d * norm;
        if direction.dot(&norm) < 0.0 {
            offset = -offset;
        }
        offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let orig = Point3::<f32>::new(0.0, 0.0, 0.0);
        let dir = Vector3::<f32>::new(1.0, 2.0, 3.0);
        let ray = Ray::new(orig, dir);
        assert_eq!(ray.orig, orig);
        assert_eq!(ray.dir, dir);
    }

    #[test]
    fn test_direction() {
        let orig = Point3::<f32>::new(0.0, 0.0, 0.0);
        let dir = Vector3::<f32>::new(1.0, 2.0, 3.0);
        let ray = Ray::new(orig, dir);
        assert_eq!(ray.direction(), dir);
    }

    #[test]
    fn test_origin() {
        let orig = Point3::<f32>::new(0.0, 0.0, 0.0);
        let dir = Vector3::<f32>::new(1.0, 2.0, 3.0);
        let ray = Ray::new(orig, dir);
        assert_eq!(ray.origin(), orig);
    }

    #[test]
    fn test_at() {
        let orig = Point3::<f32>::new(0.0, 0.0, 0.0);
        let dir = Vector3::<f32>::new(1.0, 2.0, 3.0);
        let ray = Ray::new(orig, dir);
        let t = 2.0;
        assert_eq!(ray.at(t), orig + 2.0 * dir);
    }
}
