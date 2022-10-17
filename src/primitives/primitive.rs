use crate::enum_dispatch::*;
use crate::material::*;
use crate::nalgebra::{Point3, Vector3};
use crate::raytracing::Ray;
use crate::raytracing::HitRecord;
extern crate fastrand;
use super::*;

#[enum_dispatch(Hit)]
#[derive(Clone)]
pub enum Primitive {
    Triangle(Triangle),
    Sphere(Sphere),
    Rect(Rect),
}