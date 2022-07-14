pub mod bvh;
pub mod rect;
pub mod sphere;
pub mod triangle;

use crate::material::lambertian::Lambertian;
use crate::primitives::triangle::*;
use crate::primitives::sphere::*;
use crate::primitives::rect::*;
use crate::material::*;
use crate::nalgebra::{Vector3, Point3};
use crate::image::Color;
use crate::primitives::bvh::*;
use crate::enum_dispatch::*;
use crate::rasterizing::Rasterize;
extern crate fastrand;

use crate::camera::Camera;
use crate::raytracing::{HitRecord, Hit, Ray};
use crate::spectrum::Spectrum;

use core::cmp::Ordering;
use std::convert::TryFrom;



#[enum_dispatch(Hit)]
#[enum_dispatch(Rasterize)]
#[derive (Clone)]
pub enum GeometricPrimitive {
    Triangle(Triangle),
    Sphere(Sphere),
    Rect(Rect),
}

impl GeometricPrimitive {
    pub fn new_triangle(vertices: [Point3<f32>; 3], normals: [Vector3<f32>;3], mat: Material) -> GeometricPrimitive {
        GeometricPrimitive::Triangle(Triangle::new(vertices, normals, mat))
    }

    pub fn new_sphere(cen: Point3<f32>, rad: f32, mat: Material) -> GeometricPrimitive {
        GeometricPrimitive::Sphere(Sphere::new(cen, rad, mat))
    }

    pub fn new_rect(axes: RectAxes, axis1_min: f32, axis1_max: f32, axis2_min: f32, axis2_max: f32, k: f32, mat: Material) -> GeometricPrimitive {
        GeometricPrimitive::Rect(Rect::new(axes, axis1_min, axis1_max, axis2_min, axis2_max, k, mat))
    }
}

#[enum_dispatch(Hit)]
#[derive (Clone)]
pub enum Primitive {
    GeometricPrimitive(GeometricPrimitive),
    Bvh(BvhNode)
}

impl Primitive {

    pub fn new_triangle(vertices: [Point3<f32>; 3], normals: [Vector3<f32>;3], mat: Material) -> Primitive {
        Primitive::new_geometric_primitive(GeometricPrimitive::new_triangle(vertices, normals, mat))
    }

    pub fn new_sphere(cen: Point3<f32>, rad: f32, mat: Material) -> Primitive {
        Primitive::new_geometric_primitive(GeometricPrimitive::new_sphere(cen, rad, mat))
    }

    pub fn new_rect(axes: RectAxes, axis1_min: f32, axis1_max: f32, axis2_min: f32, axis2_max: f32, k: f32, mat: Material) -> Primitive {
        Primitive::new_geometric_primitive(GeometricPrimitive::new_rect(axes, axis1_min, axis1_max, axis2_min, axis2_max, k, mat))
    }

    pub fn new_geometric_primitive(geometric_primitive: GeometricPrimitive) -> Primitive{
        Primitive::GeometricPrimitive(geometric_primitive)
    }

    pub fn new_bvh(bvh: BvhNode) -> Primitive {
        Primitive::Bvh(bvh)
    }
}


#[derive (Default, Clone)]
pub struct Primitives {
    list: Vec<Primitive>
}

#[derive (Default, Clone)]
pub struct GeometricPrimitives {
    list: Vec<GeometricPrimitive>
}




impl GeometricPrimitives {

    pub fn new() -> GeometricPrimitives {
        GeometricPrimitives{list: Vec::new()}
    } 
    

    pub fn add(&mut self, new_traceable: GeometricPrimitive) {
        self.list.push(new_traceable);
    }

    pub fn remove(&mut self, index: usize) -> GeometricPrimitive {
        self.list.remove(index)
    }

    pub fn get(&self, index: usize) -> GeometricPrimitive {
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
        F: FnMut(&GeometricPrimitive, &GeometricPrimitive) -> Ordering,
    {
        self.list.sort_by(compare);
    }

    pub fn measure_extent(&self, axis_index: usize) -> Option<f32> {
        
        if self.len() == 0 {
            return None
        }
        let mut min_val = f32::INFINITY;
        let mut max_val = f32:: NEG_INFINITY;

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
        let mut largest_extent = f32::NEG_INFINITY;
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

    pub fn split_off(&mut self, at: usize) -> GeometricPrimitives{
        GeometricPrimitives{list: self.list.split_off(at)}
    }

    pub fn to_bvh(self) -> BvhNode {
        BvhNode::new(self)
    }

    pub fn add_obj(&mut self, models: Vec<tobj::Model>, materials_opt: Option<Vec<tobj::Material>>, spectrum: Spectrum){
        for  m in models.iter(){
           //if m.name == "wheel_fr_Circle.050_MAIN"{
                let mesh = &m.mesh;
                let pos = &mesh.positions;
                let norms = &mesh.normals;
                let model_color:Color;
                match &materials_opt{
                    Some(mat) =>{
                        let mat_id = mesh.material_id.unwrap();
                        model_color = Color::new(mat[mat_id].diffuse[0] as f32, mat[mat_id].diffuse[1] as f32, mat[mat_id].diffuse[2] as f32);
                    }
                    None =>{
                        model_color = Vector3::<f32>::new(0.5, 0.5, 0.5);
                    }
                }
                for face_indices in mesh.indices.chunks(3){
                    let mut tri_vert = [Point3::<f32>::default();3];
                    let mut tri_norm = [Vector3::<f32>::default(); 3];
                    for vertex in 0..3{
                        tri_vert[vertex] = Point3::<f32>::new(pos[usize::try_from(face_indices[vertex]*3    ).unwrap()].into(),
                                                    pos[usize::try_from(face_indices[vertex]*3 + 1).unwrap()].into(),
                                                    pos[usize::try_from(face_indices[vertex]*3 + 2).unwrap()].into());
                        tri_norm[vertex] = Vector3::<f32>::new(norms[usize::try_from(face_indices[vertex]*3    ).unwrap()].into(),
                                                    norms[usize::try_from(face_indices[vertex]*3 + 1).unwrap()].into(),
                                                    norms[usize::try_from(face_indices[vertex]*3 + 2).unwrap()].into());
                    }

                    let tri = GeometricPrimitive::new_triangle(tri_vert, tri_norm, Material::Lambertian(Lambertian::new(spectrum)));
                    self.add(tri);
                }
        }
    }
}

impl Hit for GeometricPrimitives {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        
        let mut closest_so_far = t_max;
        let mut hit_out: Option<HitRecord> = None;

        for traceable in &self.list{
            if let Some(hit_temp) = traceable.hit(r, t_min, closest_so_far){
                hit_out = Some(hit_temp);
                closest_so_far = hit_temp.time;
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

impl Rasterize for GeometricPrimitives {
    fn outline(&self, cam: &Camera) -> Option<Vec<[usize; 2]>> {
        self.list.outline(cam)
    }
}


impl Primitives {

    pub fn new() -> Primitives {
        Primitives{list: Vec::new()}
    } 
    

    pub fn add(&mut self, new_traceable: Primitive) {
        self.list.push(new_traceable);
    }

    pub fn remove(&mut self, index: usize) -> Primitive {
        self.list.remove(index)
    }

    pub fn get(&self, index: usize) -> Primitive {
        self.list[index].clone()
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

    pub fn measure_extent(&self, axis_index: usize) -> Option<f32> {
        
        if self.len() == 0 {
            return None
        }
        let mut min_val = f32::INFINITY;
        let mut max_val = f32:: NEG_INFINITY;

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
        let mut largest_extent = f32::NEG_INFINITY;
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

    pub fn split_off(&mut self, at: usize) -> Primitives{
        Primitives{list: self.list.split_off(at)}
    }


    pub fn add_obj(&mut self, models: Vec<tobj::Model>, materials_opt: Option<Vec<tobj::Material>>, model_spectrum: Spectrum){
        for  m in models.iter(){
           //if m.name == "wheel_fr_Circle.050_MAIN"{
                let mesh = &m.mesh;
                let pos = &mesh.positions;
                let norms = &mesh.normals;
                let model_color:Color;
                match &materials_opt{
                    Some(mat) =>{
                        let mat_id = mesh.material_id.unwrap();
                        model_color = Color::new(mat[mat_id].diffuse[0] as f32, mat[mat_id].diffuse[1] as f32, mat[mat_id].diffuse[2] as f32);
                    }
                    None =>{
                        model_color = Vector3::<f32>::new(0.5, 0.5, 0.5);
                    }
                }
                for face_indices in mesh.indices.chunks(3){
                    let mut tri_vert = [Point3::<f32>::default();3];
                    let mut tri_norm = [Vector3::<f32>::default(); 3];
                    for vertex in 0..3{
                        tri_vert[vertex] = Point3::<f32>::new(pos[usize::try_from(face_indices[vertex]*3    ).unwrap()].into(),
                                                    pos[usize::try_from(face_indices[vertex]*3 + 1).unwrap()].into(),
                                                    pos[usize::try_from(face_indices[vertex]*3 + 2).unwrap()].into());
                        tri_norm[vertex] = Vector3::<f32>::new(norms[usize::try_from(face_indices[vertex]*3    ).unwrap()].into(),
                                                    norms[usize::try_from(face_indices[vertex]*3 + 1).unwrap()].into(),
                                                    norms[usize::try_from(face_indices[vertex]*3 + 2).unwrap()].into());
                    }

                    let tri = Primitive::new_triangle(tri_vert, tri_norm, Material::new_lambertian(model_spectrum));
                    self.add(tri);
                }
        }
    }
}

impl Hit for Primitives {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        
        let mut closest_so_far = t_max;
        let mut hit_out: Option<HitRecord> = None;

        for traceable in &self.list{
            if let Some(hit_temp) = traceable.hit(r, t_min, closest_so_far){
                hit_out = Some(hit_temp);
                closest_so_far = hit_temp.time;
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





#[cfg(test)]
mod tests {
    use crate::primitives::sphere::*;
    use crate::material::*;
    use super::*;

    #[test]
     fn test_add(){
        let mut list = Primitives::new();
        let center = Point3::<f32>::new(0.0, 0.0, 0.0);
        let radius = 5.0;
        let mat = Material::Lambertian(Lambertian::default());
        let s = Primitive::new_sphere(center, radius, mat);
        list.add(s);
        assert_eq!(list.len(), 1);
     }

    #[test]
    fn test_remove(){
        let mut list = Primitives::new();
        let center = Point3::<f32>::new(0.0, 0.0, 0.0);
        let radius = 5.0;
        let mat = Material::Lambertian(Lambertian::default());
        let s = Primitive::new_sphere(center, radius, mat);
        list.add(s);
        list.remove(0);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_clone() {
        let mut list = Primitives::new();
        let center = Point3::<f32>::new(0.0, 0.0, 0.0);
        let radius = 5.0;
        let mat = Material::Lambertian(Lambertian::default());
        let s = Primitive::new_sphere(center, radius, mat);
        list.add(s);

        let list_clone = list;
        assert_eq!(list_clone.len(), 1);
    }

    #[test]
    fn test_hit(){
        let mut list = Primitives::new();
        let r = Ray::new(Point3::<f32>::new(-10.0, 0.0, 0.0), Vector3::<f32>::new( 1.0, 0.0, 0.0));
        let t_min = 0.0;
        let t_max = 100.0;

        //Case 1: No intersections
        let center = Point3::<f32>::new(0.0, -10.0, 0.0);
        let radius = 5.0;
        let mat = Material::Lambertian(Lambertian::default());
        let s = Primitive::new_sphere(center, radius, mat);
        list.add(s);
        let hit = list.hit(&r, t_min, t_max);
        assert!(hit.is_none());

        //Case 2: One intersection
        let center = Point3::<f32>::new(0.0, 0.0, 0.0);
        let radius = 5.0;
        let mat = Material::Lambertian(Lambertian::default());
        let s = Primitive::new_sphere(center, radius, mat);
        list.add(s);
        let hit = list.hit(&r, t_min, t_max);
        assert!(hit.is_some());
        let rec = hit.unwrap();
        assert_eq!(rec.time, 5.0);  
        
        //Case 3: Two intersections
        let center = Point3::<f32>::new(-2.0, 0.0, 0.0);
        let radius = 5.0;
        let mat = Material::Lambertian(Lambertian::default());
        let s = Primitive::new_sphere(center, radius, mat);
        list.add(s);
        let hit = list.hit(&r, t_min, t_max);
        assert!(hit.is_some());
        let rec = hit.unwrap();
        assert_eq!(rec.time, 3.0); 
    }

    #[test]
    fn test_sort_by(){
        let mut list = Primitives::new();
        for i in 0..101{
            let center = Point3::<f32>::new(500.0 - 5.0*(i as f32), 0.0, 0.0);
            let radius = 1.0;
            let mat = Material::Lambertian(Lambertian::default());
            let s = Primitive::new_sphere(center, radius, mat);
            list.add(s);
        }

        list.sort_by(|a, b| Aabb::box_compare(a, b, 0));

        for i in 0..101{
            match list.get(i) {
                Primitive::GeometricPrimitive(shape) => {
                    if let GeometricPrimitive::Sphere(sphere) = shape{ 
                        let center = sphere.center();
                        assert_eq!(sphere.center(), Point3::<f32>::new(5.0 * (i as f32), 0.0, 0.0));
                    } else {
                        panic!()
                    }
                }
                _ => panic!()
            }
        }
    }

    #[test]
    fn test_largest_extent() {
        let mut list = Primitives::new();
        assert!(list.get_largest_extent().is_none());

        //Sphere 1
        let center = Point3::<f32>::new(0.0, 0.0, 0.0);
        let radius = 5.0;
        let mat = Material::Lambertian(Lambertian::default());
        let s = Primitive::new_sphere(center, radius, mat);
        list.add(s);

        //Sphere 2
        let center = Point3::<f32>::new(-2.0, -10.0, 3.0);
        let radius = 5.0;
        let mat = Material::Lambertian(Lambertian::default());
        let s = Primitive::new_sphere(center, radius, mat);
        list.add(s);

        assert_eq!(list.get_largest_extent().unwrap(), 1 as usize)

    }

}

