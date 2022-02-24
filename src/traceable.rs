extern crate fastrand;

use crate::vec::*;
use crate::ray::*;
use crate::bvh::*;
use crate::material::*;
use crate::triangle::*;
use crate::primitive::*;
use crate::enum_dispatch::*;

use core::cmp::Ordering;
use std::convert::TryFrom;

#[derive (Copy, Clone)]
pub struct HitRecord{
    
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub p_err: Vec3,
}

#[derive (Default, Clone)]
pub struct TraceableList {
    list: Vec<Primitive>
}

pub enum TraceResult{
    Missed,
    Absorbed(Color),
    Scattered((Color, Ray))
}

impl HitRecord{
    pub fn new(p: Point3, normal: Vec3, t: f64, r: Ray, p_err: Vec3) -> HitRecord{
        let mut rec = HitRecord{p, normal, t, front_face: true, p_err};
        rec.set_face_normal(&r, &normal);
        rec      
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3){
        self.front_face = r.direction().dot(*outward_normal) <= 0.0;
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

    pub fn new() -> TraceableList {
        TraceableList{list: Vec::new()}
    } 
    

    pub fn add(&mut self, new_traceable: Primitive) {
        self.list.push(new_traceable);
    }

    pub fn remove(&mut self, index: usize) -> Primitive {
        self.list.remove(index)
    }

    pub fn get(&self, index: usize) -> Primitive {
        self.list[index]
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&Primitive, &Primitive) -> Ordering,
    {
        self.list.sort_by(compare);
    }

    pub fn measure_extent(&self, axis_index: usize) -> Option<f64> {
        
        if self.len() == 0 {
            return None
        }
        let mut min_val = f64::INFINITY;
        let mut max_val = f64:: NEG_INFINITY;

        for primitive in &self.list {
            let bb_option = primitive.bounding_box();
            match bb_option {
                None => return None,
                Some(bb) => {
                    min_val = min_val.min(bb.centroid()[axis_index]);
                    max_val = max_val.max(bb.centroid()[axis_index]);
                }
            }
        }
        Some(max_val - min_val)
    }

    pub fn get_largest_extent(&self) -> Option<usize>{
        if self.len() == 0 {
            return None
        }
        let mut largest_index = 1;
        let mut largest_extent = f64::NEG_INFINITY;
        for i in 0..3{
            let extent_option = self.measure_extent(i);
            match extent_option{
                None => return None,
                Some(extent_i) => {
                    if extent_i > largest_extent {
                        largest_extent = extent_i;
                        largest_index = i;
                    }
                }
            }
          
        }

        Some(largest_index)
    }

    pub fn split_off(&mut self, at: usize) -> TraceableList{
        TraceableList{list: self.list.split_off(at)}
    }

    pub fn to_bvh(self) -> BvhNode {
        BvhNode::new(self)
    }

    pub fn add_obj(&mut self, models: Vec<tobj::Model>, materials_opt: Option<Vec<tobj::Material>>){
        for  m in models.iter(){
           //if m.name == "wheel_fr_Circle.050_MAIN"{
                let mesh = &m.mesh;
                let pos = &mesh.positions;
                let norms = &mesh.normals;
                let model_color:Color;
                match &materials_opt{
                    Some(mat) =>{
                        let mat_id = mesh.material_id.unwrap();
                        model_color = Color::new(mat[mat_id].diffuse[0] as f64, mat[mat_id].diffuse[1] as f64, mat[mat_id].diffuse[2] as f64);
                    }
                    None =>{
                        model_color = Vec3::new(0.5, 0.5, 0.5);
                    }
                }
                for face_indices in mesh.indices.chunks(3){
                    let mut tri_vert = [Point3::default();3];
                    let mut tri_norm = [Vec3::default(); 3];
                    for vertex in 0..3{
                        tri_vert[vertex] = Point3::new(pos[usize::try_from(face_indices[vertex]*3    ).unwrap()].into(),
                                                    pos[usize::try_from(face_indices[vertex]*3 + 1).unwrap()].into(),
                                                    pos[usize::try_from(face_indices[vertex]*3 + 2).unwrap()].into());
                        tri_norm[vertex] = Vec3::new(norms[usize::try_from(face_indices[vertex]*3    ).unwrap()].into(),
                                                    norms[usize::try_from(face_indices[vertex]*3 + 1).unwrap()].into(),
                                                    norms[usize::try_from(face_indices[vertex]*3 + 2).unwrap()].into());
                    }

                    let tri = Triangle::new(tri_vert, tri_norm, Material::Lambertian(Lambertian::new(model_color)));
                    self.add(Primitive::Triangle(tri));
                }
            //}
        }
    }
}

impl Hit for TraceableList{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<(HitRecord, &Material)> {
        
        let mut closest_so_far = t_max;
        let mut hit_out: Option<(HitRecord, &Material)> = None;

        for traceable in &self.list{
            if let Some(hit_temp) = traceable.hit(r, t_min, closest_so_far){
                hit_out = Some(hit_temp);
                closest_so_far = hit_temp.0.t;
            }
        }
        hit_out
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


#[enum_dispatch]
pub trait Hit: Send + Sync{
    fn hit(&self ,r: &Ray, t_min: f64, t_max: f64) -> Option<(HitRecord, &Material)>;
    fn bounding_box(&self) -> Option<Aabb>;

    fn trace(&self, r: &Ray, t_min: f64, t_max: f64) -> TraceResult{
        if let Some((hit_rec, mat)) = self.hit(r, t_min, t_max) {
            if let Some((attenuation, scattered)) = mat.scatter(r, &hit_rec){
                TraceResult::Scattered((mat.emit() + attenuation, scattered))
            } else{
                TraceResult::Absorbed(mat.emit())
            }
        } else{
            TraceResult::Missed
        }
    }
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
        let mat = Material::Lambertian(Lambertian::default());
        let s = Sphere::new(center, radius, mat);
        list.add(Primitive::Sphere(s));
        assert_eq!(list.len(), 1);
     }

    #[test]
    fn test_remove(){
        let mut list = TraceableList::new();
        let center = Vec3::new(0.0, 0.0, 0.0);
        let radius = 5.0;
        let mat = Material::Lambertian(Lambertian::default());
        let s = Sphere::new(center, radius, mat);
        list.add(Primitive::Sphere(s));
        list.remove(0);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_clone(){
        let mut list = TraceableList::new();
        let center = Vec3::new(0.0, 0.0, 0.0);
        let radius = 5.0;
        let mat = Material::Lambertian(Lambertian::default());
        let s = Primitive::Sphere(Sphere::new(center, radius, mat));
        list.add(s);

        let list_clone = list;
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
        let mat = Material::Lambertian(Lambertian::default());
        let s = Primitive::Sphere(Sphere::new(center, radius, mat));
        list.add(s);
        let hit = list.hit(&r, t_min, t_max);
        assert!(hit.is_none());

        //Case 2: One intersection
        let center = Vec3::new(0.0, 0.0, 0.0);
        let radius = 5.0;
        let mat = Material::Lambertian(Lambertian::default());
        let s = Primitive::Sphere(Sphere::new(center, radius, mat));
        list.add(s);
        let hit = list.hit(&r, t_min, t_max);
        assert!(hit.is_some());
        let (rec, _) = hit.unwrap();
        assert_eq!(rec.t, 5.0);  
        
        //Case 3: Two intersections
        let center = Vec3::new(-2.0, 0.0, 0.0);
        let radius = 5.0;
        let mat = Material::Lambertian(Lambertian::default());
        let s = Primitive::Sphere(Sphere::new(center, radius, mat));
        list.add(s);
        let hit = list.hit(&r, t_min, t_max);
        assert!(hit.is_some());
        let (rec, _) = hit.unwrap();
        assert_eq!(rec.t, 3.0); 
    }

    #[test]
    fn test_sort_by(){
        let mut list = TraceableList::new();
        for i in 0..101{
            let center = Vec3::new(500.0 - 5.0*(i as f64), 0.0, 0.0);
            let radius = 1.0;
            let mat = Material::Lambertian(Lambertian::default());
            let s = Primitive::Sphere(Sphere::new(center, radius, mat));
            list.add(s);
        }

        list.sort_by(|a, b| Aabb::box_compare(a, b, 0));

        for i in 0..101{
            match list.get(i) {
                Primitive::Sphere(sphere) => {
                    let center = sphere.center();
                    assert_eq!(sphere.center(), Vec3::new(5.0 * (i as f64), 0.0, 0.0));
                }
                _ => panic!()
            }
        }
    }

    #[test]
    fn test_largest_extent() {
        let mut list = TraceableList::new();
        assert!(list.get_largest_extent().is_none());

        //Sphere 1
        let center = Vec3::new(0.0, 0.0, 0.0);
        let radius = 5.0;
        let mat = Material::Lambertian(Lambertian::default());
        let s = Sphere::new(center, radius, mat);
        list.add(Primitive::Sphere(s));

        //Sphere 2
        let center = Vec3::new(-2.0, -10.0, 3.0);
        let radius = 5.0;
        let mat = Material::Lambertian(Lambertian::default());
        let s = Sphere::new(center, radius, mat);
        list.add(Primitive::Sphere(s));

        assert_eq!(list.get_largest_extent().unwrap(), 1 as usize)

    }

    // #[test]
    // fn add_obj(){
    //     let mut mesh_1 = tobj::Mesh::default();
    //     mesh_1.positions = vec!(-2.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 2.0, 0.0,
    //                             -2.0, 0.0, 5.0, 2.0, 0.0, 5.0, 0.0, 2.0, 5.0,
    //                             -2.0, 0.0, 8.0, 2.0, 0.0, 8.0, 0.0, 2.0, 8.0);

    //     mesh_2.positions = vec!(-2.0, 0.0, 5.0, 2.0, 0.0, 5.0, 0.0, 2.0, 5.0);

    //     let test_1 = tobj::Model::new(mesh_1, "test_1".to_string());

    //     let mesh_2 = tobj::Mesh::default();
    //     let test_2 = tobj::Model::new(mesh_2, "test_2".to_string());

    //     let mesh_3 = tobj::Mesh::default();
    //     let test_3 = tobj::Model::new(mesh_3, "test_3".to_string());

        

    // }
}
