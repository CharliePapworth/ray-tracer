use crate::triangle::*;
use crate::sphere::*;
use crate::rect::*;
use crate::traceable::*;
use crate::ray::*;
use crate::material::*;
use crate::vec::*;
use crate::bvh::*;
use crate::enum_dispatch::*;


#[enum_dispatch(Hit)]
#[enum_dispatch(Outline)]
#[derive (Copy, Clone)]
pub enum GeometricPrimitive {
    Triangle(Triangle),
    Sphere(Sphere),
    Rect(Rect),
}

impl GeometricPrimitive {
    pub fn new_triangle(vertices: [Point3; 3], normals: [Vec3;3], mat: Material) -> GeometricPrimitive {
        GeometricPrimitive::Triangle(Triangle::new(vertices, normals, mat))
    }

    pub fn new_sphere(cen: Point3, rad: f64, mat: Material) -> GeometricPrimitive {
        GeometricPrimitive::Sphere(Sphere::new(cen, rad, mat))
    }

    pub fn new_rect(axes: RectAxes, axis1_min: f64, axis1_max: f64, axis2_min: f64, axis2_max: f64, k: f64, mat: Material) -> GeometricPrimitive {
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

    pub fn new_triangle(vertices: [Point3; 3], normals: [Vec3;3], mat: Material) -> Primitive {
        Primitive::new_geometric_primitive(GeometricPrimitive::new_triangle(vertices, normals, mat))
    }

    pub fn new_sphere(cen: Point3, rad: f64, mat: Material) -> Primitive {
        Primitive::new_geometric_primitive(GeometricPrimitive::new_sphere(cen, rad, mat))
    }

    pub fn new_rect(axes: RectAxes, axis1_min: f64, axis1_max: f64, axis2_min: f64, axis2_max: f64, k: f64, mat: Material) -> Primitive {
        Primitive::new_geometric_primitive(GeometricPrimitive::new_rect(axes, axis1_min, axis1_max, axis2_min, axis2_max, k, mat))
    }

    pub fn new_geometric_primitive(geometric_primitive: GeometricPrimitive) -> Primitive{
        Primitive::GeometricPrimitive(geometric_primitive)
    }

    pub fn new_bvh(bvh: BvhNode) -> Primitive {
        Primitive::Bvh(bvh)
    }
}
