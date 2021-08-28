use crate::vec::*;
use crate::ray::*;
use crate::bvh::*;
use std::clone;
use std::ops::Index;
use core::cmp::Ordering;

#[derive (Copy, Clone)]
pub struct HitRecord<'a>{
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: &'a dyn Scatter
}

pub trait Traceable: Hit{
    fn box_clone(&self) -> Box<dyn Traceable>;
    fn trace(&self, r: &Ray, t_min: f64, t_max: f64) -> TraceResult;
}
 impl<T: Hit> Traceable for T where T: Clone + 'static{
     fn box_clone(&self) -> Box<dyn Traceable>{
         Box::new((*self).clone())
     }
     fn trace(&self, r: &Ray, t_min: f64, t_max: f64) -> TraceResult{
        
        if let Some(hit_rec) = self.hit(r, t_min, t_max) {
            if let Some((attenuation, scattered)) = hit_rec.mat.scatter(r, &hit_rec){
                TraceResult::Scattered((attenuation, scattered))
            } else{
                TraceResult::Absorbed
            }
        } else{
            TraceResult::Missed
        }
    }
 }

pub struct TraceableList{
    list: Vec<Box<dyn Traceable>>
}

pub enum TraceResult{
    Missed,
    Absorbed,
    Scattered((Color, Ray))
}


impl<'a> HitRecord<'a>{
    pub fn new(p: Point3, normal: Vec3, t: f64, r: Ray, mat: &'a dyn Scatter) -> HitRecord<'a>{
        let mut rec = HitRecord{p, normal, t, front_face: true, mat};
        rec.set_face_normal(&r, &normal);
        rec      
    }

    pub fn from_mat(mat: &'a dyn Scatter) -> HitRecord<'a>{
        HitRecord::new(Point3::default(), Vec3::default(), 0.0, Ray::default(), mat)
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
    

    pub fn add(&mut self, new_traceable: Box<dyn Traceable>)
    {
        self.list.push(new_traceable);
    }

    pub fn remove(&mut self, index: usize) -> Box<dyn Traceable>{
        self.list.remove(index)
    }

    pub fn get(&self, index: usize) -> &dyn Traceable{
        &*self.list[index]
    }

    pub fn len(&self) -> usize{
        self.list.len()
    }

    pub fn empty(&self) -> bool{
        self.list.len() == 0
    }

    pub fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&Box<dyn Traceable>, &Box<dyn Traceable>) -> Ordering,
    {
        self.list.sort_by(compare);
    }

    pub fn split_off(&mut self, at: usize) -> TraceableList{
        TraceableList{list: self.list.split_off(at)}
    }

    pub fn to_Bvh(self) -> BvhNode{
        BvhNode::new(self)
    }
}

impl Hit for TraceableList{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>{
        
        let mut closest_so_far = t_max;
        let mut rec_out = None;

        for traceable in &self.list{
            if let Some(rec_temp) = traceable.hit(r, t_min, closest_so_far){
                rec_out = Some(rec_temp);
                closest_so_far = rec_temp.t;
            }
        }
        rec_out
    }

    fn bounding_box(&self) -> Option<Aabb>{
        if self.empty(){
           None
        }else{
            let mut output_box = Aabb::default();
            let mut first_box = true;
            for traceable in &self.list{
                match (traceable.bounding_box(), first_box){
                    (None,_) => {return None}
                    (Some(temp_box),true) => {
                        output_box = temp_box;
                        first_box = false;
                    }
                    (Some(temp_box), false) => {output_box = Aabb::surrounding_box(output_box, temp_box);}
                }
            }
            Some(output_box) 
        }
    }
}

impl Clone for Box<dyn Traceable>{
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

impl Clone for TraceableList{
    fn clone(&self) -> Self {
        TraceableList{list: self.list.clone()}
    }
}


pub trait Hit{
    fn hit(&self ,r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<Aabb>;
}

pub trait Scatter{
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

#[cfg(test)]
mod tests {
    use crate::sphere::*;
    use crate::material::*;
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
    fn test_clone(){
        let mut list = TraceableList::new();
        let center = Vec3::new(0.0, 0.0, 0.0);
        let radius = 5.0;
        let mat = Lambertian::default();
        let s = Box::new(Sphere::new(center, radius, mat));
        list.add(s);

        let list_clone = list.clone();
        assert_eq!(list_clone.len(), 1);
    }

    #[test]
    fn test_hit(){
        let mut list = TraceableList::new();
        let r = Ray::new(Vec3::new(-10.0, 0.0, 0.0), Vec3::new( 1.0, 0.0, 0.0));
        let t_min = 0.0;
        let t_max = 100.0;

        //Case 1: No intersections
        let center = Vec3::new(0.0, -10.0, 0.0);
        let radius = 5.0;
        let mat = Lambertian::default();
        let s = Box::new(Sphere::new(center, radius, mat));
        list.add(s);
        let hit = list.hit(&r, t_min, t_max);
        assert!(hit.is_none());

        //Case 2: One intersection
        let center = Vec3::new(0.0, 0.0, 0.0);
        let radius = 5.0;
        let mat = Lambertian::default();
        let s = Box::new(Sphere::new(center, radius, mat));
        list.add(s);
        let hit = list.hit(&r, t_min, t_max);
        assert!(hit.is_some());
        let rec = hit.unwrap();
        assert_eq!(rec.t, 5.0);  
        
        //Case 3: Two intersections
        let center = Vec3::new(-2.0, 0.0, 0.0);
        let radius = 5.0;
        let mat = Lambertian::default();
        let s = Box::new(Sphere::new(center, radius, mat));
        list.add(s);
        let hit = list.hit(&r, t_min, t_max);
        assert!(hit.is_some());
        let rec = hit.unwrap();
        assert_eq!(rec.t, 3.0); 
    }

    #[test]
    fn test_sort_by(){
        let mut list = TraceableList::new();
        let t_min = 0.0;
        let t_max = 100.0;
        for i in 0..101{
            let center = Vec3::new(500.0 - 5.0*(i as f64), 0.0, 0.0);
            let radius = 1.0;
            let mat = Lambertian::default();
            let s = Box::new(Sphere::new(center, radius, mat));
            list.add(s);
        }
        let r = Ray::new(Vec3::new(0.0, -10.0, 0.0), Vec3::new( 0.0, 1.0, 0.0));
        let hit = list.hit(&r, t_min, t_max).unwrap();
        assert_eq!(hit.t, 9.0);
        list.sort_by(|a, b| Aabb::box_compare(a, b, 0));
        list.remove(0);
        let hit = list.hit(&r, t_min, t_max);
        assert!(hit.is_none());
    }
}
