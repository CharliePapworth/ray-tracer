use std::f32::consts::PI;
use crate::camera::camera::Camera;
use crate::geometry::lines::Line3;
use crate::geometry::plane::Plane;
use crate::geometry::points::Point3ExtensionMethods;
use crate::material::*;
use crate::nalgebra::{Point3, Vector3};
use crate::primitives::bvh::*;
use crate::rasterizing::*;
use crate::raytracing::{Hit, HitRecord, Ray};
use crate::util::gamma;

#[derive(Clone)]
pub struct Sphere {
    center: Point3<f32>,
    radius: f32,
    material: Material,
}

impl Sphere {
    ///Initialises a new sphere
    pub fn new(cen: Point3<f32>, rad: f32, mat: Material) -> Sphere {
        Sphere {
            center: cen,
            radius: rad,
            material: mat,
        }
    }

    /// Returns the center of the sphere
    pub fn center(&self) -> Point3<f32> {
        self.center
    }

    /// Checks whether the sphere is at least partially in front of the plane.
    ///
    /// A sphere is defined as being in front of the plane if any point on its
    /// surface is in front of the plane. A point is defined as being in
    /// front of the plane if the plane normal points away from it.
    pub fn is_in_front(&self, plane: Plane) -> bool {
        //Check if the sphere is in view
        let mut closest_point = self.center + self.radius * plane.orientation.w.into_inner();
        let origin_in_view = self.center.is_in_front(plane);
        if !origin_in_view {
            closest_point = self.center - plane.orientation.w.into_inner();
        }
        let closest_point_in_view = closest_point.is_in_front(plane);

        origin_in_view || closest_point_in_view
    }

    ///Wraps the horizon of the sphere in a mesh of lines
    //The following links contain useful information:
    //https://stackoverflow.com/questions/21648630/radius-of-projected-sphere-in-screen-space
    //https://math.stackexchange.com/questions/1367710/perspective-projection-of-a-sphere-on-a-plane
    //https://zingl.github.io/Bresenham.pdf
    pub fn wrap_horizon(&self, cam: &Camera) -> Option<Vec<Line3>> {
        const NUMBER_OF_LINES: usize = 200;
        let mut lines = Vec::with_capacity(NUMBER_OF_LINES);
        let visible_plane = Plane::new(cam.orientation, cam.origin);

        //Check if the sphere is in view
        if !self.is_in_front(visible_plane) {
            return None;
        }

        //Find the horizon of the sphere
        let radius_origin_vector = self.center - cam.origin;
        let radius_origin_distance = (self.center - cam.origin).norm();
        let horizon_radius = (radius_origin_distance.powi(2) - self.radius.powi(2)).sqrt() * self.radius / radius_origin_distance;
        let horizon_basis_vector_a = radius_origin_vector.cross(&cam.orientation.w).normalize() * horizon_radius;
        let horizon_basis_vector_b = horizon_basis_vector_a.cross(&radius_origin_vector).normalize() * horizon_radius;
        let horizon_center_offset = (self.radius.powi(2) - horizon_radius.powi(2)).sqrt();
        let origin_horizon_center = radius_origin_vector.normalize() * (radius_origin_distance - horizon_center_offset);

        //Approximate the boundary of the horizon with straight lines
        for i in 0..NUMBER_OF_LINES {
            let new_angle = (i as f32 + 1.0) * 2.0 * PI / (NUMBER_OF_LINES as f32);
            let old_angle = (i as f32) * 2.0 * PI / (NUMBER_OF_LINES as f32);
            let line_start = cam.origin
                + origin_horizon_center
                + horizon_basis_vector_a * f32::cos(old_angle)
                + horizon_basis_vector_b * f32::sin(old_angle);

            let line_end = cam.origin
                + origin_horizon_center
                + horizon_basis_vector_a * f32::cos(new_angle)
                + horizon_basis_vector_b * f32::sin(new_angle);
            let line = Line3::new(line_start, line_end);
            lines.push(line);
        }

        Some(lines)
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().norm_squared();
        let half_b = oc.dot(&r.direction());
        let c = oc.norm_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            None
        } else {
            let sqrtd = discriminant.sqrt();
            let mut root = (-half_b - sqrtd) / a;
            if root < t_min || t_max < root {
                root = (-half_b + sqrtd) / a;
                if root < t_min || t_max < root {
                    return None;
                }
            }

            let t = root;
            let p = r.at(t);
            let outward_normal = (p - self.center) / self.radius;
            let new_rec = HitRecord::new(p, outward_normal, self.material, -r.dir, root, *r, p.coords * gamma(5));
            Some(new_rec)
        }
    }

    fn bounding_box(&self) -> Option<Aabb> {
        let output_box = Aabb::new(
            self.center - Vector3::<f32>::new(self.radius, self.radius, self.radius),
            self.center + Vector3::<f32>::new(self.radius, self.radius, self.radius),
        );
        Some(output_box)
    }
}

impl Rasterize for Sphere {
    fn outline(&self, cam: &Camera) -> Option<Vec<[usize; 2]>> {
        let camera_plane = Plane::new(cam.orientation, cam.origin);

        //Check if the sphere is at least partially in front of the camera window
        if !self.center.is_in_front(camera_plane) && self.radius < self.center.distance_to_plane(camera_plane) {
            return None;
        }

        if let Some(lines) = self.wrap_horizon(cam) {
            lines.outline(cam)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::{lambertian::Lambertian, *};

    #[test]
    fn test_new() {
        let center = Point3::<f32>::new(0.0, 0.0, 0.0);
        let radius = 5.0;
        let mat = Material::Lambertian(Lambertian::default());
        let s = Sphere::new(center, radius, mat);
        assert_eq!(s.center, Point3::<f32>::new(0.0, 0.0, 0.0));
        assert_eq!(s.radius, 5.0);
    }

    #[test]
    fn test_hit() {
        //Case 1: Intersection from outside of sphere
        let center = Point3::<f32>::new(0.0, 0.0, 0.0);
        let radius = 5.0;
        let mat = Material::Lambertian(Lambertian::default());
        let s = Sphere::new(center, radius, mat);
        let r = Ray::new(Point3::<f32>::new(-10.0, 0.0, 0.0), Vector3::<f32>::new(1.0, 0.0, 0.0));
        let t_min = 0.0;
        let t_max = 100.0;
        let rec_wrapper = s.hit(&r, t_min, t_max);
        assert!(rec_wrapper.is_some());
        let rec = rec_wrapper.unwrap();
        assert_eq!(rec.time, 5.0);
        assert_eq!(rec.surface_normal, Vector3::<f32>::new(-1.0, 0.0, 0.0));
        assert_eq!(rec.point_in_scene, Point3::<f32>::new(-5.0, 0.0, 0.0));
        assert_eq!(rec.front_face, true);

        //Case 2: Intersection from inside of sphere
        let r = Ray::new(Point3::<f32>::new(1.0, 0.0, 0.0), Vector3::<f32>::new(-2.0, 0.0, 0.0));
        let rec_wrapper = s.hit(&r, t_min, t_max);
        assert!(rec_wrapper.is_some());
        let rec = rec_wrapper.unwrap();
        assert_eq!(rec.time, 3.0);
        assert_eq!(rec.surface_normal, Vector3::<f32>::new(1.0, 0.0, 0.0));
        assert_eq!(rec.point_in_scene, Point3::<f32>::new(-5.0, 0.0, 0.0));
        assert_eq!(rec.front_face, false);

        //Case 3: Intersection tangent to sphere
        let r = Ray::new(Point3::<f32>::new(-5.0, 5.0, 0.0), Vector3::<f32>::new(0.0, -1.0, 0.0));
        let rec_wrapper = s.hit(&r, t_min, t_max);
        assert!(rec_wrapper.is_some());
        let rec = rec_wrapper.unwrap();
        assert_eq!(rec.time, 5.0);
        assert_eq!(rec.surface_normal, Vector3::<f32>::new(-1.0, 0.0, 0.0));
        assert_eq!(rec.point_in_scene, Point3::<f32>::new(-5.0, 0.0, 0.0));
        assert_eq!(rec.front_face, true);

        //Case 4: Intersection of inverted sphere (negative radius)
        let s = Sphere::new(center, -radius, mat);
        let r = Ray::new(Point3::<f32>::new(0.0, -10.0, 0.0), Vector3::<f32>::new(0.0, 1.0, 0.0));
        let rec_wrapper = s.hit(&r, t_min, t_max);
        assert!(rec_wrapper.is_some());
        let rec = rec_wrapper.unwrap();
        assert_eq!(rec.time, 5.0);
        assert_eq!(rec.surface_normal, Vector3::<f32>::new(0.0, -1.0, 0.0));
        assert_eq!(rec.point_in_scene, Point3::<f32>::new(0.0, -5.0, 0.0));
        assert_eq!(rec.front_face, false);
    }

    #[test]
    fn test_bounding_box() {
        let center = Point3::<f32>::new(0.0, -3.0, 2.0);
        let radius = 5.0;
        let mat = Material::Lambertian(Lambertian::default());
        let s = Sphere::new(center, radius, mat);
        let bb = s.bounding_box().unwrap();
        assert_eq!(bb.min(), Point3::<f32>::new(-5.0, -8.0, -3.0));
        assert_eq!(bb.max(), Point3::<f32>::new(5.0, 2.0, 7.0));
    }
}
