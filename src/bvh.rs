use crate::vec::*;
use crate::ray::*;
use crate::traceable::*;
use crate::material::*;
use crate::primitive::*;
use std::cmp::Ordering;

#[derive (Debug, Copy, Clone, Default, PartialEq)]

pub struct Aabb{
    min: Point3,
    max: Point3
}

#[derive(Clone)]
pub enum BvhNode {
    Branch(BvhBranch),
    Root(BvhRoot),
}

#[derive(Clone)]
pub struct BvhBranch{
    children: (Box<BvhNode>, Box<BvhNode>),
    bb: Aabb
}

#[derive(Clone)]
pub struct BvhRoot{
    traceable: Primitive,
    bb: Aabb
}

impl Aabb{
    pub fn new(min: Point3, max: Point3) -> Aabb{
        Aabb{min, max}
    }

    pub fn min(&self) -> Point3{
        self.min
    }

    pub fn max(&self) -> Point3{
        self.max
    }

    pub fn centroid(&self) -> Point3{
        self.min() + (self.max() - self.min()) / 2.0
    }

    pub fn hit(&self, r: &Ray, mut t_min: f64, mut t_max: f64) -> bool{
        for a in 0..3{
            let tx0 = (self.min()[a] - r.origin()[a]) / r.direction()[a];
            let tx1 = (self.max()[a] - r.origin()[a]) / r.direction()[a];
            
            let t0 = tx0.min(tx1);
            let t1 = tx0.max(tx1);

            t_min = t0.max(t_min);
            t_max = t1.min(t_max);

            if t_max <= t_min{
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(box_0: Aabb, box_1: Aabb) -> Aabb{
        let small = Point3::new(box_0.min().x().min(box_1.min().x()),
                                box_0.min().y().min(box_1.min().y()),
                                box_0.min().z().min(box_1.min().z()));

        let big = Point3::new(box_0.max().x().max(box_1.max().x()),
                              box_0.max().y().max(box_1.max().y()),
                              box_0.max().z().max(box_1.max().z()));

        Aabb::new(small, big)
    }

    pub fn box_compare<H>(a: &H, b: &H, axis: i8) -> Ordering where H: Hit {
        match (a.bounding_box(), b.bounding_box()){
            (Some(box_a), Some(box_b)) => box_a.min().index(axis as usize).partial_cmp(&box_b.min().index(axis as usize)).unwrap(),
            (_, _) => panic!("One of a, b cannot be bound")
        }
    }
}

impl BvhBranch {
    pub fn new(left: TraceableList, right: TraceableList, bb: Aabb) -> BvhNode{
        BvhNode::Branch(BvhBranch{children: (Box::new(BvhNode::new(left)), Box::new(BvhNode::new(right))), bb})
    }

    fn left(&self) -> &BvhNode{
        &*(self.children.0)
    }

    fn right(&self) -> &BvhNode{
        &*(self.children.1)
    }

    fn children(&self) -> (&BvhNode, &BvhNode){
        (&*(self.children.0), &*(self.children.1))
    }
}

impl BvhRoot{
    pub fn new(traceable: Primitive, bb: Aabb) -> BvhNode{
        BvhNode::Root(BvhRoot{traceable, bb})
    }
}

impl BvhNode{
    pub fn new(mut objects: TraceableList) -> BvhNode{
        let object_span = objects.len();
        match object_span {
            1 => {
                let traceable = objects.remove(0);
                let bb = traceable.bounding_box().expect("A primitive within the TraceableList cannot be bound");
                BvhRoot::new(traceable, bb)

            } 
            
            _ => {
                let axis = objects.get_largest_extent().expect("The TraceableList is empty") as i8;
                objects.sort_by(|a, b| Aabb::box_compare(a, b, axis));
                let mid = object_span/2;
                let right_objs = objects.split_off(mid);
                let left_objs = objects;
                let bb_left = left_objs.bounding_box().expect("A primitive within the TraceableList cannot be bound");
                let bb_right = right_objs.bounding_box().expect("A primitive within the TraceableList cannot be bound");
                let bb_surrounding = Aabb::surrounding_box(bb_left, bb_right);
                BvhBranch::new(left_objs, right_objs, bb_surrounding)
            }
        }
    }

    fn hit_debug(&self ,r: &Ray, t_min: f64, t_max: f64) -> (i32, Option<(HitRecord, &Material)>) {
        match self{
            BvhNode::Branch(x) => x.hit_debug(r, t_min, t_max),
            BvhNode::Root(x) => x.hit_debug(r, t_min, t_max)
        }
    }
}

impl BvhBranch{
    pub fn hit_debug(&self, r: &Ray, t_min: f64, t_max: f64) -> (i32, Option<(HitRecord, &Material)>) {
        if !self.bb.hit(r, t_min, t_max){
            return (0, None)
        } 

        let mut hit_left = self.left().hit_debug(r, t_min, t_max);
        let mut hit_right = self.right().hit_debug(r, t_min, t_max);
        hit_left.0 += hit_right.0;
        hit_right.0 = hit_left.0;
        match(hit_left.1, hit_right.1){
            (None, None) => (hit_left.0, None),
            (Some(_), None) => hit_left,
            (None, Some(_)) => hit_right,
            (Some(left), Some(right)) =>  {
                if left.0.t <= right.0.t{
                    hit_left
                } else {
                    hit_right
                }
            }
        }
    }
}

impl BvhRoot{
    pub fn hit_debug(&self, r: &Ray, t_min: f64, t_max: f64) -> (i32, Option<(HitRecord, &Material)>) {
        match self.traceable.hit(r, t_min, t_max) {
            Some((rec, mat)) => {
                (1, Some((rec, mat)))
            }
            None => (1, (None))
        }
    }
}
impl Hit for BvhBranch {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<(HitRecord, &Material)> {
        if !self.bb.hit(r, t_min, t_max){
            return None
        } 

        let hit_left = self.left().hit(r, t_min, t_max);
        let hit_right = self.right().hit(r, t_min, t_max);
        match(hit_left, hit_right){
            (None, None) => None,
            (Some(_), None) => hit_left,
            (None, Some(_)) => hit_right,
            (Some(left), Some(right)) =>  {
                if left.0.t <= right.0.t{
                    hit_left
                } else {
                    hit_right
                }
            }
        }
        
    }
    
    fn bounding_box(&self) -> Option<Aabb> {
        Some(self.bb)
    }
}

impl Hit for BvhRoot {
    fn hit(&self ,r: &Ray, t_min: f64, t_max: f64) -> Option<(HitRecord, &Material)> {
        self.traceable.hit(r, t_min, t_max)
    }
    fn bounding_box(&self) -> Option<Aabb>{
        Some(self.bb)
    }
}

impl Hit for BvhNode{
    fn hit(&self ,r: &Ray, t_min: f64, t_max: f64) -> Option<(HitRecord, &Material)> {
        match self{
            BvhNode::Branch(x) => x.hit(r, t_min, t_max),
            BvhNode::Root(x) => x.hit(r, t_min, t_max)
        }
    }
    fn bounding_box(&self) -> Option<Aabb>{
        match self{
            BvhNode::Branch(x) => x.bounding_box(),
            BvhNode::Root(x) => x.bounding_box()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sphere::*;
    use crate::material::*;
    
    use crate::triangle::*;
        
    use super::*;

    #[test]
    fn min(){
        let min = Point3::new(0.0, -2.0, 1.0);
        let max = Point3::new(10.0, 3.0, 4.0);
        let aabb = Aabb::new(min, max);
        assert_eq!(aabb.min(), Point3::new(0.0, -2.0, 1.0));
    }

    #[test]
    fn max(){
        let min = Point3::new(0.0, -2.0, 1.0);
        let max = Point3::new(10.0, 3.0, 4.0);
        let aabb = Aabb::new(min, max);
        assert_eq!(aabb.max(), Point3::new(10.0, 3.0, 4.0));
    }

    #[test]
    fn test_aabb_hit(){

        let min = Point3::new(-10.0, -10.0, -10.0);
        let max = Point3::new(10.0, 10.0, 10.0);
        let aabb = Aabb::new(min, max);

        //Case 1: Hit
        let r = Ray::new(Vec3::new(20.0, 5.0, 5.0), Vec3::new( -1.0, 0.4, -0.2));
        let rec = aabb.hit(&r, 0.0, 100.0);
        assert_eq!(rec, true);

        //Case 2: Miss (due to timeout)
        let rec = aabb.hit(&r, 0.0, 9.99);
        assert_eq!(rec, false);

        //Case 3: Miss (due to geometry)
        let r = Ray::new(Vec3::new(-10.0, 10.01, 5.0), Vec3::new( 1.0, 0.0, 0.0));
        let rec = aabb.hit(&r, 0.0, 100.0);
        assert_eq!(rec, false);
    }

    #[test]
    fn test_surrounding_box(){
        
        let min = Point3::new(0.0, 0.0, 0.0);
        let max = Point3::new(10.0, 10.0, 10.0);
        let aabb_a = Aabb::new(min, max);

        let min = Point3::new(-1.0, 3.0, -2.0);
        let max = Point3::new(9.0, 16.0, 10.0);
        let aabb_b = Aabb::new(min, max);

        let sb = Aabb::surrounding_box(aabb_a, aabb_b);

        assert_eq!(sb.min(), Point3::new(-1.0, 0.0, -2.0));
        assert_eq!(sb.max(), Point3::new(10.0, 16.0, 10.0));


    }

    #[test]
    fn test_box_compare(){
        
        let center = Vec3::new(0.0, -10.0, 0.0);
        let radius = 5.0;
        let mat = Material::Lambertian(Lambertian::default());
        let s1 = Primitive::Sphere(Sphere::new(center, radius, mat));

        let center = Vec3::new(-20.0, -10.0, 0.0);
        let radius = 5.0;
        let mat = Material::Lambertian(Lambertian::default());
        let s2 = Primitive::Sphere(Sphere::new(center, radius, mat));

        let sb = Aabb::box_compare(&s1, &s2, 0);
        assert_eq!(sb, Ordering::Greater);
    }

    #[test]
    fn test_bvhnode_hit(){

        let mut list = TraceableList::new();
        let r = Ray::new(Vec3::new(-10.0, 0.0, 0.0), Vec3::new( 1.0, 0.0, 0.0));
        let t_min = 0.0;
        let t_max = 100.0;

        
        //Case 1: No intersections
        let center = Vec3::new(0.0, -10.0, 0.0);
        let radius = 5.0;
        let mat = Material::Lambertian(Lambertian::default());
        let s = Primitive::Sphere(Sphere::new(center, radius, mat));
        for _ in 1..100{
            list.add(s.clone());
        }
        let list_clone = list.clone();
        let bvh = list_clone.to_bvh();
        let hit = bvh.hit(&r, t_min, t_max);
        assert!(hit.is_none());

        //Case 2: Single intersection
        let center = Vec3::new(0.0, 0.0, 0.0);
        let radius = 5.0;
        let mat = Material::Lambertian(Lambertian::default());
        let s = Primitive::Sphere(Sphere::new(center, radius, mat));
        list.add(s);
        let list_clone = list.clone();
        let bvh = list_clone.to_bvh();
        let hit = bvh.hit(&r, t_min, t_max);
        assert!(hit.is_some());
        let (rec, _) = hit.unwrap();
        assert_eq!(rec.t, 5.0); 
        
        //Case 3: Two intersections
        let center = Vec3::new(-2.0, 0.0, 0.0);
        let radius = 5.0;
        let mat = Material::Lambertian(Lambertian::default());
        let s = Primitive::Sphere(Sphere::new(center, radius, mat));
        list.add(s);
        let list_clone = list.clone();
        let bvh = list_clone.to_bvh();
        let hit = bvh.hit(&r, t_min, t_max);
        assert!(hit.is_some());
        let (rec, _) = hit.unwrap();
        assert_eq!(rec.t, 3.0); 
        
    }

    #[test]
    
    fn test_bvhnode_hit_debug(){

        let t_min = 0.0;
        let t_max = 1000.0;

        //Case 1: Ray misses bounding box entirely
        let r = Ray::new(Vec3::new(-10.0, -10.0, 0.0), Vec3::new( 1.0, 0.0, 0.0));
        let mut list = TraceableList::new();
        let radius = 5.0;
        let mat = Material::Lambertian(Lambertian::default());
        for i in 1..100{
            let center = Vec3::new(i as f64, 0.0, 0.0);
            let s = Primitive::Sphere(Sphere::new(center, radius, mat));
            list.add(s);
        }
        let bvh = list.to_bvh();
        let hit = bvh.hit_debug(&r, t_min, t_max);
        assert_eq!(hit.0, 0);
        assert!(hit.1.is_none());

        //Case 2: Ray hits one bounding box, but misses the sphere
        let r = Ray::new(Vec3::new(1.0, -10.0, 6.0), Vec3::new( 0.0, 1.0, 0.0));
        let mut list = TraceableList::new();
        let radius = 5.0;
        let mat = Material::Lambertian(Lambertian::default());
        for i in 1..100{
            let center = Vec3::new(i as f64, 0.0, 0.0);
            let s = Primitive::Sphere(Sphere::new(center, radius, mat));
            list.add(s);
        }
        let bvh = list.to_bvh();
        let hit = bvh.hit_debug(&r, t_min, t_max);
        assert_eq!(hit.0, 0);
        assert!(hit.1.is_none());


         //Case 3: Ray hits all bounding boxes but misses the spheres
         let r = Ray::new(Vec3::new(-10.0, 6.0, 0.0), Vec3::new( 1.0, 0.0, 0.0));
         let mut list = TraceableList::new();
         let radius = 5.0;
         let mat = Material::Lambertian(Lambertian::default());
         for i in 1..100{
             let center = Vec3::new(i as f64, 0.0, 0.0);
             let s = Primitive::Sphere(Sphere::new(center, radius, mat));
             list.add(s);
         }
         let bvh = list.to_bvh();
         let hit = bvh.hit_debug(&r, t_min, t_max);
         assert_eq!(hit.0, 0);
         assert!(hit.1.is_none());

         //Case 4: Ray hits all spheres
         let r = Ray::new(Vec3::new(-10.0, 0.0, 0.0), Vec3::new( 1.0, 0.0, 0.0));
         let mut list = TraceableList::new();
         let radius = 5.0;
         let mat = Material::Lambertian(Lambertian::default());
         for i in 1..100{
             let center = Vec3::new(i as f64, 0.0, 0.0);
             let s = Primitive::Sphere(Sphere::new(center, radius, mat));
             list.add(s);
         }
         let bvh = list.to_bvh();
         let hit = bvh.hit_debug(&r, t_min, t_max);
         assert_eq!(hit.0, 99);
         assert!(hit.1.is_some()); 
         
         //Case 5: Ray hits 10 spheres and 10 bounding boxes
         let r = Ray::new(Vec3::new(20.0, -10.0, 4.0), Vec3::new( 0.0, 1.0, 0.0));
         let mut list = TraceableList::new();
         let radius = 5.0;
         let mat = Material::Lambertian(Lambertian::default());
         for i in 1..100{
             let center = Vec3::new(i as f64, 0.0, 0.0);
             let s = Primitive::Sphere(Sphere::new(center, radius, mat));
             list.add(s);
         }
         let bvh = list.to_bvh();
         let hit = bvh.hit_debug(&r, t_min, t_max);
         let rec = hit.1.unwrap();
         assert_eq!(hit.0, 9);
         assert!(hit.1.is_some());

         //Case 6: Ray hits 1 triangles
         let r = Ray::new(Vec3::new(1.5, -10.0, 4.0), Vec3::new( 0.0, 1.0, 0.0));
         let mut list = TraceableList::new();
         let mat = Material::Lambertian(Lambertian::default());
         for i in 1..100{
             let v1 = Vec3::new(i as f64, 0.0, 0.0);
             let v2 = Vec3::new((i + 1) as f64, 0.0, 0.0);
             let v3 = Vec3::new(i as f64 + 0.5, 0.0, 10.0);
             let norm = Vec3::new(0.0, 1.0, 0.0);
             let s = Primitive::Triangle(Triangle::new([v1, v2, v3], [norm; 3], mat ));
             list.add(s);
         }
         let bvh = list.to_bvh();
         let hit = bvh.hit_debug(&r, t_min, t_max);
         let rec = hit.1;
         assert_eq!(hit.0, 1);
        assert!(rec.is_some());

    }

    
}