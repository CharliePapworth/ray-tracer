use std::primitive;

use enum_dispatch::enum_dispatch;

use crate::raytracing::{HitRecord, Ray};

use super::AxisAlignedBoundingBox;

#[enum_dispatch]
pub trait Hit {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<AxisAlignedBoundingBox>;
}

impl<C, H, D> Hit for C
where
    D : Deref<Target = H> + ?Sized,
    C: IntoIterator<Item = D>,
    H: Hit + ?Sized,
{
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_out: Option<HitRecord> = None;

        for primitive in self.into_iter() {
            if let Some(hit_temp) = primitive.hit(r, t_min, closest_so_far) {
                hit_out = Some(hit_temp);
                closest_so_far = hit_temp.time;
            }
        }
        hit_out
    }

    fn bounding_box(&self) -> Option<AxisAlignedBoundingBox> {
        let mut output_box = AxisAlignedBoundingBox::default();
        let mut first_box = true;
        for primitive in self.into_iter() {
            match (primitive.bounding_box(), first_box) {
                (None, _) => return None,
                (Some(temp_box), true) => {
                    output_box = temp_box;
                    first_box = false;
                }
                (Some(temp_box), false) => {
                    output_box = AxisAlignedBoundingBox::surrounding_box(output_box, temp_box);
                }
            }
        }

        match first_box {
            true => None,
            false => Some(output_box),
        }
    }
}


// Implements Hit for references to iterable containers yielding immutable references to types implementing Hit
impl<'a, I, H> Hit for &'a I
where
    &'a I: IntoIterator<Item = &'a H>,
    I: ?Sized,
    H: Hit + 'a,
{
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_out: Option<HitRecord> = None;

        for primitive in self.into_iter() {
            if let Some(hit_temp) = primitive.hit(r, t_min, closest_so_far) {
                hit_out = Some(hit_temp);
                closest_so_far = hit_temp.time;
            }
        }
        hit_out
    }

    fn bounding_box(&self) -> Option<AxisAlignedBoundingBox> {
        let mut output_box = AxisAlignedBoundingBox::default();
        let mut first_box = true;
        for primitive in self.into_iter() {
            match (primitive.bounding_box(), first_box) {
                (None, _) => return None,
                (Some(temp_box), true) => {
                    output_box = temp_box;
                    first_box = false;
                }
                (Some(temp_box), false) => {
                    output_box = AxisAlignedBoundingBox::surrounding_box(output_box, temp_box);
                }
            }
        }

        match first_box {
            true => None,
            false => Some(output_box),
        }
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::{Point3, Unit, Vector3};

    use super::*;
    use crate::{
        light::*,
        material::{Lambertian, Material},
        primitives::{Primitive, Primitives, Triangle},
    };

    #[test]
    fn test_new() {
        let mat = Material::Lambertian(Lambertian::default());
        let v0 = Point3::<f32>::new(-2.0, 2.0, 0.0);
        let v1 = Point3::<f32>::new(2.0, 2.0, 0.0);
        let v2 = Point3::<f32>::new(0.0, 4.0, 0.0);
        let norm = [Vector3::<f32>::new(0.0, 0.0, 1.0).normalize(); 3];
        let t = Triangle::new([v0, v1, v2], norm, mat);
        let mut primitives = Vec::new();
        primitives.push(Primitive::from(t));
        (&primitives).bounding_box();
    }
}
