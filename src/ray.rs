use crate::vec::*;
#[derive (Copy, Clone, Default, PartialEq, Debug)]
pub struct Ray{
    orig: Point3,
    dir: Vec3,
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