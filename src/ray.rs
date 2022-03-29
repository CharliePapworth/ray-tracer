use crate::{vec::*, geometry::{Plane, LinePlaneIntersection}};
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