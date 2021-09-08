use crate::vec::*;
use crate::ray::*;
use crate::traceable::*;
use crate::bvh::*;

#[derive (Copy, Clone)]
pub struct Sphere<M> where M: Scatter{
    center: Point3,
    radius: f64,
    material: M
}

impl<M> Sphere<M> where M: Scatter{
    pub fn new(cen: Point3, rad: f64, mat: M) -> Sphere<M>{
        Sphere{center: cen, radius: rad, material: mat}
    }
}

impl<M> Hit for Sphere<M> where M: Scatter{
    fn hit(&self, r:&Ray, t_min: f64, t_max: f64) -> Option<HitRecord>{
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(&oc, &r.direction());
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
            let new_rec = HitRecord::new(p, outward_normal, root, *r, self);
            Some(new_rec)
        }
    }

    fn bounding_box(&self) -> Option<Aabb> {
        let output_box = Aabb::new(self.center - Vec3::new(self.radius, self.radius, self.radius),
                                   self.center + Vec3::new(self.radius, self.radius, self.radius));
        Some(output_box)
    }
}

impl<M> Scatter for Sphere<M> where M: Scatter{
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>{
        let result= self.material.scatter(r_in, rec);
        if result.is_some(){
            let (attenuation, scattered) = result.unwrap();
            Some((attenuation, scattered))
        } else{
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
        let mat = Lambertian::default();
        let s = Sphere::new(center, radius, mat);
        assert_eq!(s.center, Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(s.radius, 5.0);
    }

    #[test]
    fn test_hit(){

        //Case 1: Intersection from outside of sphere
        let center = Vec3::new(0.0, 0.0, 0.0);
        let radius = 5.0;
        let mat = Lambertian::default();
        let s = Sphere::new(center, radius, mat);
        let r = Ray::new(Vec3::new(-10.0, 0.0, 0.0), Vec3::new( 1.0, 0.0, 0.0));
        let t_min = 0.0;
        let t_max = 100.0;
        let rec_wrapper = s.hit(&r, t_min, t_max);
        assert!(rec_wrapper.is_some());
        let rec = rec_wrapper.unwrap();
        assert_eq!(rec.t(), 5.0);
        assert_eq!(rec.normal(), Vec3::new(-1.0, 0.0, 0.0));
        assert_eq!(rec.p(), Point3::new(-5.0, 0.0, 0.0));
        assert_eq!(rec.front_face(), true);

        //Case 2: Intersection from inside of sphere
        let r = Ray::new(Vec3::new(1.0, 0.0, 0.0), Vec3::new( -2.0, 0.0, 0.0));
        let rec_wrapper = s.hit(&r, t_min, t_max);
        assert!(rec_wrapper.is_some());
        let rec = rec_wrapper.unwrap();
        assert_eq!(rec.t(), 3.0);
        assert_eq!(rec.normal(), Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(rec.p(), Point3::new(-5.0, 0.0, 0.0));
        assert_eq!(rec.front_face(), false);

        //Case 3: Intersection tangent to sphere
        let r = Ray::new(Vec3::new(-5.0, 5.0, 0.0), Vec3::new( 0.0, -1.0, 0.0));
        let rec_wrapper = s.hit(&r, t_min, t_max);
        assert!(rec_wrapper.is_some());
        let rec = rec_wrapper.unwrap();
        assert_eq!(rec.t(), 5.0);
        assert_eq!(rec.normal(), Vec3::new(-1.0, 0.0, 0.0));
        assert_eq!(rec.p(), Point3::new(-5.0, 0.0, 0.0));
        assert_eq!(rec.front_face(), true);

        //Case 4: Intersection of inverted sphere (negative radius)
        let s = Sphere::new(center, -radius, mat);
        let r = Ray::new(Vec3::new(0.0, -10.0, 0.0), Vec3::new( 0.0, 1.0, 0.0));
        let rec_wrapper = s.hit(&r, t_min, t_max);
        assert!(rec_wrapper.is_some());
        let rec = rec_wrapper.unwrap();
        assert_eq!(rec.t(), 5.0);
        assert_eq!(rec.normal(), Vec3::new(0.0, -1.0, 0.0));
        assert_eq!(rec.p(), Point3::new(0.0, -5.0, 0.0));
        assert_eq!(rec.front_face(), false);
    }

    #[test]
    fn test_bounding_box(){
        let center = Vec3::new(0.0, -3.0, 2.0);
        let radius = 5.0;
        let mat = Lambertian::default();
        let s = Sphere::new(center, radius, mat);
        let bb = s.bounding_box().unwrap();
        assert_eq!(bb.min(), Point3::new(-5.0, -8.0, -3.0));
        assert_eq!(bb.max(), Point3::new(5.0, 2.0, 7.0));
    } 
}