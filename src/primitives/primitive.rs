use crate::enum_dispatch::*;
use crate::material::*;
use crate::nalgebra::{Point3, Vector3};
extern crate fastrand;
use super::*;

#[enum_dispatch(Hit)]
#[derive(Clone)]
pub enum Primitive<'a> {
    Triangle(Triangle),
    Sphere(Sphere),
    Rect(Rect),
    Bvh(BvhNode<'a>),
}

impl<'a> Primitive<'a> {
    pub fn new_triangle(vertices: [Point3<f32>; 3], normals: [Vector3<f32>; 3], mat: Material) -> Primitive<'a> {
        Primitive::Triangle(Triangle::new(vertices, normals, mat))
    }

    pub fn new_sphere(cen: Point3<f32>, rad: f32, mat: Material) -> Primitive<'a> {
        Primitive::Sphere(Sphere::new(cen, rad, mat))
    }

    pub fn new_rect(
        axes: RectAxes,
        axis1_min: f32,
        axis1_max: f32,
        axis2_min: f32,
        axis2_max: f32,
        k: f32,
        mat: Material,
    ) -> Primitive<'a> {
        Primitive::Rect(Rect::new(axes, axis1_min, axis1_max, axis2_min, axis2_max, k, mat))
    }

    pub fn new_bvh(bvh: BvhNode) -> Primitive {
        Primitive::Bvh(bvh)
    }
}
