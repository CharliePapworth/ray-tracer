use crate::triangle::*;
use crate::traceable::*;
use crate::ray::*;
use crate::material::*;
use crate::vec::*;
use crate::bvh::*;

#[derive (Copy, Clone)]
pub struct BoundingBox{
    bb: Aabb,
    mat: Material
}

impl BoundingBox{
    pub fn from_triangle(triangle: Triangle) -> BoundingBox{
        BoundingBox{bb: triangle.bounding_box().unwrap(), mat: Material::Lambertian(Lambertian::default())}
    }
}

impl Hit for BoundingBox{
    fn hit(&self, r:&Ray, t_min: f64, t_max: f64) -> Option<(HitRecord, &Material)> {
        match self.bb.hit(r, t_min, t_max){
            true => {
                Some((HitRecord::new(Vec3::default(), Vec3::default(), 10.0, *r, Vec3::default()), &self.mat))
            }
            false => None
        }
    }

    fn bounding_box(&self) -> Option<Aabb>{
       Some(self.bb)
    }
}