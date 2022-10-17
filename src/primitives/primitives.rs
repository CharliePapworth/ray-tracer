use crate::enum_dispatch::*;
use crate::material::*;
use crate::nalgebra::{Point3, Vector3};
use crate::light::Spectrum;
extern crate fastrand;
use super::AxisAlignedBoundingBox;
use super::BvhNode;
use super::Primitive;
use super::{Hit};
use crate::raytracing::{HitRecord, Ray};
use std::convert::TryFrom;

#[derive(Default, Clone)]
pub struct Primitives {
    list: Vec<Primitive>,
    bounding_volume_hierarchy: Option<BvhNode<'static>>
}

impl Hit for Primitives {
    fn hit(&self,r: &Ray,t_min:f32,t_max:f32) -> Option<HitRecord> {
        match &self.bounding_volume_hierarchy {
            Some(bvh) => bvh.hit(r, t_min, t_max),
            None => (&self.list).hit(r, t_min, t_max),
        }
    }

    fn bounding_box(&self) -> Option<AxisAlignedBoundingBox> {
        match &self.bounding_volume_hierarchy {
            Some(bvh)=> bvh.bounding_box(),
            None => (&self.list).bounding_box(),
        }
    }
}

impl Primitives {
    pub fn new() -> Primitives {
        Primitives { list: Vec::new(), bounding_volume_hierarchy: None }
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

    pub fn construct_acceleration_structures(&mut self) {
        todo!()
    }

    
    // pub fn add_obj(&mut self, models: Vec<tobj::Model>, materials_opt: Option<Vec<tobj::Material>>, model_spectrum: Spectrum) {
    //     for m in models.iter() {
    //         //if m.name == "wheel_fr_Circle.050_MAIN"{
    //         let mesh = &m.mesh;
    //         let pos = &mesh.positions;
    //         let norms = &mesh.normals;
    //         let model_color: Color;
    //         match &materials_opt {
    //             Some(mat) => {
    //                 let mat_id = mesh.material_id.unwrap();
    //                 model_color =
    //                     Color::new(mat[mat_id].diffuse[0] as f32, mat[mat_id].diffuse[1] as f32, mat[mat_id].diffuse[2] as f32);
    //             }
    //             None => {
    //                 model_color = Vector3::<f32>::new(0.5, 0.5, 0.5);
    //             }
    //         }
    //         for face_indices in mesh.indices.chunks(3) {
    //             let mut tri_vert = [Point3::<f32>::default(); 3];
    //             let mut tri_norm = [Vector3::<f32>::default(); 3];
    //             for vertex in 0..3 {
    //                 tri_vert[vertex] = Point3::<f32>::new(
    //                     pos[usize::try_from(face_indices[vertex] * 3).unwrap()].into(),
    //                     pos[usize::try_from(face_indices[vertex] * 3 + 1).unwrap()].into(),
    //                     pos[usize::try_from(face_indices[vertex] * 3 + 2).unwrap()].into(),
    //                 );
    //                 tri_norm[vertex] = Vector3::<f32>::new(
    //                     norms[usize::try_from(face_indices[vertex] * 3).unwrap()].into(),
    //                     norms[usize::try_from(face_indices[vertex] * 3 + 1).unwrap()].into(),
    //                     norms[usize::try_from(face_indices[vertex] * 3 + 2).unwrap()].into(),
    //                 );
    //             }

    //             let tri = Primitive::new_triangle(tri_vert, tri_norm, Material::new_lambertian(model_spectrum));
    //             self.add(tri);
    //         }
    //     }
    // }
}



#[cfg(test)]
mod tests {
    use nalgebra::Point3;
    use nalgebra::Unit;
    use nalgebra::Vector3;
    use crate::primitives::*;
    use crate::material::*;
    use crate::raytracing::Ray;
    
    // #[test]
    // fn test_add() {
    //     let mut list = Primitives::new();
    //     let center = Point3::<f32>::new(0.0, 0.0, 0.0);
    //     let radius = 5.0;
    //     let mat = Material::Lambertian(Lambertian::default());
    //     let s = Primitive::new_sphere(center, radius, mat);
    //     list.add(s);
    //     assert_eq!(list.len(), 1);
    // }

    // #[test]
    // fn test_remove() {
    //     let mut list = Primitives::new();
    //     let center = Point3::<f32>::new(0.0, 0.0, 0.0);
    //     let radius = 5.0;
    //     let mat = Material::Lambertian(Lambertian::default());
    //     let s = Primitive::new_sphere(center, radius, mat);
    //     list.add(s);
    //     list.remove(0);
    //     assert_eq!(list.len(), 0);
    // }

    // #[test]
    // fn test_clone() {
    //     let mut list = Primitives::new();
    //     let center = Point3::<f32>::new(0.0, 0.0, 0.0);
    //     let radius = 5.0;
    //     let mat = Material::Lambertian(Lambertian::default());
    //     let s = Primitive::new_sphere(center, radius, mat);
    //     list.add(s);

    //     let list_clone = list;
    //     assert_eq!(list_clone.len(), 1);
    // }

    // #[test]
    // fn test_hit() {
    //     let mut list = Primitives::new();
    //     let r = Ray::new(Point3::<f32>::new(-10.0, 0.0, 0.0), Unit::new_normalize(Vector3::<f32>::new(1.0, 0.0, 0.0)));
    //     let t_min = 0.0;
    //     let t_max = 100.0;

    //     //Case 1: No intersections
    //     let center = Point3::<f32>::new(0.0, -10.0, 0.0);
    //     let radius = 5.0;
    //     let mat = Material::Lambertian(Lambertian::default());
    //     let s = Primitive::new_sphere(center, radius, mat);
    //     list.add(s);
    //     let hit = list.hit(&r, t_min, t_max);
    //     assert!(hit.is_none());

    //     //Case 2: One intersection
    //     let center = Point3::<f32>::new(0.0, 0.0, 0.0);
    //     let radius = 5.0;
    //     let mat = Material::Lambertian(Lambertian::default());
    //     let s = Primitive::new_sphere(center, radius, mat);
    //     list.add(s);
    //     let hit = list.hit(&r, t_min, t_max);
    //     assert!(hit.is_some());
    //     let rec = hit.unwrap();
    //     assert_eq!(rec.time, 5.0);

    //     //Case 3: Two intersections
    //     let center = Point3::<f32>::new(-2.0, 0.0, 0.0);
    //     let radius = 5.0;
    //     let mat = Material::Lambertian(Lambertian::default());
    //     let s = Primitive::new_sphere(center, radius, mat);
    //     list.add(s);
    //     let hit = list.hit(&r, t_min, t_max);
    //     assert!(hit.is_some());
    //     let rec = hit.unwrap();
    //     assert_eq!(rec.time, 3.0);
    // }

    // #[test]
    // fn test_sort_by() {
    //     let mut list = Primitives::new();
    //     for i in 0..101 {
    //         let center = Point3::<f32>::new(500.0 - 5.0 * (i as f32), 0.0, 0.0);
    //         let radius = 1.0;
    //         let mat = Material::Lambertian(Lambertian::default());
    //         let s = Primitive::new_sphere(center, radius, mat);
    //         list.add(s);
    //     }

    //     list.sort_by(|a, b| AxisAlignedBoundingBox::box_compare(a, b, 0));

    //     for i in 0..101 {
    //         match list.get(i) {
    //             Primitive::Sphere(sphere) => {
    //                 let center = sphere.center();
    //                 assert_eq!(sphere.center(), Point3::<f32>::new(5.0 * (i as f32), 0.0, 0.0));
    //             } 
    //             _ => panic!(),
    //         }
    //     }
    // }

    // #[test]
    // fn test_largest_extent() {
    //     let mut list = Primitives::new();
    //     assert!(list.get_largest_extent().is_none());

    //     //Sphere 1
    //     let center = Point3::<f32>::new(0.0, 0.0, 0.0);
    //     let radius = 5.0;
    //     let mat = Material::Lambertian(Lambertian::default());
    //     let s = Primitive::new_sphere(center, radius, mat);
    //     list.add(s);

    //     //Sphere 2
    //     let center = Point3::<f32>::new(-2.0, -10.0, 3.0);
    //     let radius = 5.0;
    //     let mat = Material::Lambertian(Lambertian::default());
    //     let s = Primitive::new_sphere(center, radius, mat);
    //     list.add(s);

    //     assert_eq!(list.get_largest_extent().unwrap(), 1 as usize)
    // }
}
