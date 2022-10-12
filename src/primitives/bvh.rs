use super::{Primitive, Primitives};
use crate::nalgebra::Point3;
use crate::raytracing::{Hit, HitRecord, Ray};
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Default, PartialEq)]

pub struct Aabb {
    min: Point3<f32>,
    max: Point3<f32>,
}

#[derive(Clone)]
pub enum BvhNode<'a> {
    Branch(BvhBranch<'a>),
    Root(BvhRoot<'a>),
}

#[derive(Clone)]
pub struct BvhBranch<'a> {
    children: (Box<BvhNode<'a>>, Box<BvhNode<'a>>),
    bb: Aabb,
}

#[derive(Clone)]
pub struct BvhRoot<'a> {
    primitive: &'a Primitive<'a>,
    bb: Aabb,
}

impl Aabb {
    pub fn new(min: Point3<f32>, max: Point3<f32>) -> Aabb {
        Aabb { min, max }
    }

    pub fn min(&self) -> Point3<f32> {
        self.min
    }

    pub fn max(&self) -> Point3<f32> {
        self.max
    }

    pub fn centroid(&self) -> Point3<f32> {
        self.min() + (self.max() - self.min()) / 2.0
    }

    pub fn hit(&self, r: &Ray, mut t_min: f32, mut t_max: f32) -> bool {
        for a in 0..3 {
            let tx0 = (self.min()[a] - r.origin()[a]) / r.direction()[a];
            let tx1 = (self.max()[a] - r.origin()[a]) / r.direction()[a];

            let t0 = tx0.min(tx1);
            let t1 = tx0.max(tx1);

            t_min = t0.max(t_min);
            t_max = t1.min(t_max);

            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(box_0: Aabb, box_1: Aabb) -> Aabb {
        let small = Point3::<f32>::new(
            box_0.min()[0].min(box_1.min()[0]),
            box_0.min()[1].min(box_1.min()[1]),
            box_0.min()[2].min(box_1.min()[2]),
        );

        let big = Point3::<f32>::new(
            box_0.max()[0].max(box_1.max()[0]),
            box_0.max()[1].max(box_1.max()[1]),
            box_0.max()[2].max(box_1.max()[2]),
        );

        Aabb::new(small, big)
    }

    pub fn box_compare<H>(a: &H, b: &H, axis: i8) -> Ordering
    where
        H: Hit,
    {
        match (a.bounding_box(), b.bounding_box()) {
            (Some(box_a), Some(box_b)) => box_a.min()[axis as usize].partial_cmp(&box_b.min()[axis as usize]).unwrap(),
            (_, _) => panic!("One of a, b cannot be bound"),
        }
    }
}

impl<'a> BvhBranch<'a> {
    pub fn new(left: Primitives, right: Primitives, bb: Aabb) -> BvhNode<'a> {
        BvhNode::Branch(BvhBranch {
            children: (Box::new(BvhNode::new(left)), Box::new(BvhNode::new(right))),
            bb,
        })
    }

    fn left(&self) -> &BvhNode {
        &*(self.children.0)
    }

    fn right(&self) -> &BvhNode {
        &*(self.children.1)
    }

    fn children(&self) -> (&BvhNode, &BvhNode) {
        (&*(self.children.0), &*(self.children.1))
    }
}

impl<'a> BvhRoot<'a> {
    pub fn new(primitive: &Primitive, bb: Aabb) -> BvhNode<'a> {
        BvhNode::Root(BvhRoot { primitive, bb })
    }
}

impl<'a> BvhNode<'a> {
    pub fn new(mut objects: Primitives) -> BvhNode<'a> {
        let object_span = objects.len();
        match object_span {
            1 => {
                let primitive = objects.remove(0);
                let bb = primitive
                    .bounding_box()
                    .expect("A Primitive within the TraceableList cannot be bound");
                BvhRoot::new(&primitive, bb)
            }

            _ => {
                let axis = objects.get_largest_extent().expect("The TraceableList is empty") as i8;
                objects.sort_by(|a, b| Aabb::box_compare(a, b, axis));
                let mid = object_span / 2;
                let right_objs = objects.split_off(mid);
                let left_objs = objects;
                let bb_left = left_objs
                    .bounding_box()
                    .expect("A Primitive within the TraceableList cannot be bound");
                let bb_right = right_objs
                    .bounding_box()
                    .expect("A Primitive within the TraceableList cannot be bound");
                let bb_surrounding = Aabb::surrounding_box(bb_left, bb_right);
                BvhBranch::new(left_objs, right_objs, bb_surrounding)
            }
        }
    }
}

impl<'a> BvhRoot<'a> {
    pub fn hit_debug(&self, r: &Ray, t_min: f32, t_max: f32) -> (i32, Option<HitRecord>) {
        match self.primitive.hit(r, t_min, t_max) {
            Some(rec) => {
                let mat = rec.surface_material;
                (1, Some(rec))
            }
            None => (1, (None)),
        }
    }
}
impl<'a> Hit for BvhBranch<'a> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !self.bb.hit(r, t_min, t_max) {
            return None;
        }

        let hit_left = self.left().hit(r, t_min, t_max);
        let hit_right = self.right().hit(r, t_min, t_max);
        match (hit_left, hit_right) {
            (None, None) => None,
            (Some(_), None) => hit_left,
            (None, Some(_)) => hit_right,
            (Some(left), Some(right)) => {
                if left.time <= right.time {
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

impl<'a> Hit for BvhRoot<'a> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.primitive.hit(r, t_min, t_max)
    }
    fn bounding_box(&self) -> Option<Aabb> {
        Some(self.bb)
    }
}

impl<'a> Hit for BvhNode<'a> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self {
            BvhNode::Branch(x) => x.hit(r, t_min, t_max),
            BvhNode::Root(x) => x.hit(r, t_min, t_max),
        }
    }
    fn bounding_box(&self) -> Option<Aabb> {
        match self {
            BvhNode::Branch(x) => x.bounding_box(),
            BvhNode::Root(x) => x.bounding_box(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::{lambertian::Lambertian, Material};
    use nalgebra::{Unit, Vector3};

    #[test]
    fn min() {
        let min = Point3::<f32>::new(0.0, -2.0, 1.0);
        let max = Point3::<f32>::new(10.0, 3.0, 4.0);
        let aabb = Aabb::new(min, max);
        assert_eq!(aabb.min(), Point3::<f32>::new(0.0, -2.0, 1.0));
    }

    #[test]
    fn max() {
        let min = Point3::<f32>::new(0.0, -2.0, 1.0);
        let max = Point3::<f32>::new(10.0, 3.0, 4.0);
        let aabb = Aabb::new(min, max);
        assert_eq!(aabb.max(), Point3::<f32>::new(10.0, 3.0, 4.0));
    }

    #[test]
    fn test_aabb_hit() {
        let min = Point3::<f32>::new(-10.0, -10.0, -10.0);
        let max = Point3::<f32>::new(10.0, 10.0, 10.0);
        let aabb = Aabb::new(min, max);

        //Case 1: Hit
        let r = Ray::new(Point3::<f32>::new(20.0, 5.0, 5.0), Unit::new_normalize(Vector3::new(-1.0, 0.4, -0.2)));
        let rec = aabb.hit(&r, 0.0, 100.0);
        assert_eq!(rec, true);

        //Case 2: Miss (due to timeout)
        let rec = aabb.hit(&r, 0.0, 9.99);
        assert_eq!(rec, false);

        //Case 3: Miss (due to geometry)
        let r = Ray::new(Point3::<f32>::new(-10.0, 10.01, 5.0), Unit::new_normalize(Vector3::<f32>::new(1.0, 0.0, 0.0)));
        let rec = aabb.hit(&r, 0.0, 100.0);
        assert_eq!(rec, false);
    }

    #[test]
    fn test_surrounding_box() {
        let min = Point3::<f32>::new(0.0, 0.0, 0.0);
        let max = Point3::<f32>::new(10.0, 10.0, 10.0);
        let aabb_a = Aabb::new(min, max);

        let min = Point3::<f32>::new(-1.0, 3.0, -2.0);
        let max = Point3::<f32>::new(9.0, 16.0, 10.0);
        let aabb_b = Aabb::new(min, max);

        let sb = Aabb::surrounding_box(aabb_a, aabb_b);

        assert_eq!(sb.min(), Point3::<f32>::new(-1.0, 0.0, -2.0));
        assert_eq!(sb.max(), Point3::<f32>::new(10.0, 16.0, 10.0));
    }

    #[test]
    fn test_box_compare() {
        let center = Point3::<f32>::new(0.0, -10.0, 0.0);
        let radius = 5.0;
        let mat = Material::Lambertian(Lambertian::default());
        let s1 = Primitive::new_sphere(center, radius, mat);

        let center = Point3::<f32>::new(-20.0, -10.0, 0.0);
        let radius = 5.0;
        let mat = Material::Lambertian(Lambertian::default());
        let s2 = Primitive::new_sphere(center, radius, mat);

        let sb = Aabb::box_compare(&s1, &s2, 0);
        assert_eq!(sb, Ordering::Greater);
    }

    #[test]
    fn test_bvhnode_hit() {
        let mut list = Primitives::new();
        let r = Ray::new(Point3::<f32>::new(-10.0, 0.0, 0.0), Unit::new_normalize(Vector3::<f32>::new(1.0, 0.0, 0.0)));
        let t_min = 0.0;
        let t_max = 100.0;

        //Case 1: No intersections
        let center = Point3::<f32>::new(0.0, -10.0, 0.0);
        let radius = 5.0;
        let mat = Material::Lambertian(Lambertian::default());
        let s = Primitive::new_sphere(center, radius, mat);
        for _ in 1..100 {
            list.add(s.clone());
        }
        let bvh = list.to_bvh();
        let hit = bvh.hit(&r, t_min, t_max);
        assert!(hit.is_none());

        //Case 2: Single intersection
        let center = Point3::<f32>::new(0.0, 0.0, 0.0);
        let radius = 5.0;
        let mat = Material::Lambertian(Lambertian::default());
        let s = Primitive::new_sphere(center, radius, mat);
        list.add(s);
        let bvh = list.to_bvh();
        let hit = bvh.hit(&r, t_min, t_max);
        assert!(hit.is_some());
        let rec = hit.unwrap();
        assert_eq!(rec.time, 5.0);

        //Case 3: Two intersections
        let center = Point3::<f32>::new(-2.0, 0.0, 0.0);
        let radius = 5.0;
        let mat = Material::Lambertian(Lambertian::default());
        let s = Primitive::new_sphere(center, radius, mat);
        list.add(s);
        let bvh = list.to_bvh();
        let hit = bvh.hit(&r, t_min, t_max);
        assert!(hit.is_some());
        let rec = hit.unwrap();
        assert_eq!(rec.time, 3.0);
    }
}
