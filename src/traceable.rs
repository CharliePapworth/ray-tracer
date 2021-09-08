use crate::vec::*;
use crate::ray::*;
use crate::material::*;

#[derive (Copy, Clone, Default)]
pub struct HitRecord{
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool
}

pub trait Traceable: Hit + Scatter + 'static{}
impl<T: Hit + Scatter + 'static> Traceable for T {}

#[derive (Default)]
pub struct TraceableList{
    list: Vec<Box<dyn Traceable>>
}

pub enum TraceResult{
    Missed,
    Absorbed,
    Scattered((Color, Ray))
}


impl HitRecord{
    pub fn new(p: &Point3, normal: &Vec3, t: f64, r: &Ray, outward_normal: &Vec3) -> HitRecord{
        let mut rec = HitRecord{p: *p, normal: *normal, t, front_face: true};
        rec.set_face_normal(r, outward_normal);
        rec        
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3){
        self.front_face = Vec3::dot(&r.direction(), outward_normal) <= 0.0;
        if self.front_face{
            self.normal = *outward_normal;
        } else{
            self.normal = -*outward_normal;
        }
    }

    pub fn p(&self) -> Vec3{
        self.p
    }

    pub fn normal(&self) -> Vec3{
        self.normal
    }

    pub fn t(&self) -> f64{
        self.t
    }

    pub fn front_face(&self) -> bool{
        self.front_face
    }
}

impl TraceableList{

    pub fn new() -> TraceableList{
        TraceableList{list: Vec::new()}
    } 

    pub fn add<T>(&mut self, new_traceable: Box<T>)
    where T:Traceable
    {
        self.list.push(new_traceable);
    }

    pub fn remove(&mut self, index: usize) -> Box<dyn Traceable>{
        self.list.remove(index)
    }

    pub fn len(&self) -> usize{
        self.list.len()
    }


    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> Option<usize>{
        
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut index:usize  = 0;
        let mut closest_index: usize = 0;

        for traceable in &self.list{
            if traceable.hit(r, t_min, closest_so_far, &mut temp_rec){
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
                closest_index = index;
            }
            index +=1;
        }
        if hit_anything{
            Some(closest_index)
        } else{
            None
        }
    }


    pub fn trace(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> TraceResult{
        
        let hit = self.hit(r, t_min, t_max, rec);
        if hit.is_some(){
            let closest_index = hit.unwrap();
            let result = self.list[closest_index].scatter(r, &rec);
            if result.is_some(){
                let (attenuation, scattered) = result.unwrap();
                TraceResult::Scattered((attenuation, scattered))
            } else{
            TraceResult::Absorbed
            }
        } else{
            TraceResult::Missed
        }
    }
}

pub trait Hit{
    fn hit(&self ,r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub trait Scatter{
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

#[cfg(test)]
mod tests {
    use crate::sphere::*;
    use super::*;

    #[test]
     fn test_add(){
        let mut list = TraceableList::new();
        let center = Vec3::new(0.0, 0.0, 0.0);
        let radius = 5.0;
        let mat = Lambertian::default();
        let s = Box::new(Sphere::new(center, radius, mat));
        list.add(s);
        assert_eq!(list.len(), 1);
     }

    #[test]
    fn test_remove(){
        let mut list = TraceableList::new();
        let center = Vec3::new(0.0, 0.0, 0.0);
        let radius = 5.0;
        let mat = Lambertian::default();
        let s = Box::new(Sphere::new(center, radius, mat));
        list.add(s);
        list.remove(0);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_hit(){
        //Case 1: No intersections
        let mut list = TraceableList::new();
        let center = Vec3::new(0.0, -10.0, 0.0);
        let radius = 5.0;
        let mat = Lambertian::default();
        let s = Box::new(Sphere::new(center, radius, mat));
        list.add(s);
        let r = Ray::new(Vec3::new(-10.0, 0.0, 0.0), Vec3::new( 1.0, 0.0, 0.0));
        let t_min = 0.0;
        let t_max = 100.0;
        let mut rec = HitRecord::default();
        let hit = list.hit(&r, t_min, t_max, &mut rec);
        assert!(hit.is_none());

        //Case 2: One intersection
        let center = Vec3::new(0.0, 0.0, 0.0);
        let radius = 5.0;
        let mat = Lambertian::default();
        let s = Box::new(Sphere::new(center, radius, mat));
        list.add(s);
        let hit = list.hit(&r, t_min, t_max, &mut rec);
        assert!(hit.is_some());
        assert_eq!(rec.t, 5.0);  
        
        //Case 3: Two intersections
        let center = Vec3::new(-2.0, 0.0, 0.0);
        let radius = 5.0;
        let mat = Lambertian::default();
        let s = Box::new(Sphere::new(center, radius, mat));
        list.add(s);
        let hit = list.hit(&r, t_min, t_max, &mut rec);
        assert!(hit.is_some());
    }
}
