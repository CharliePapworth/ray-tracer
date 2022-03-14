use std::ops;
use core::cmp::Ordering;
use std::ops::{Index, IndexMut};
use crate::*;

#[derive (PartialEq, Debug, Copy, Clone, Default)]
pub struct Vec3{
    arr: [f64; 3]
}

#[derive (PartialEq, Debug, Copy, Clone, Default)]
pub struct Vec2{
    arr: [f64; 2]
}

impl Vec2{
    pub fn new(x: f64, y: f64) -> Vec2{
        Vec2{arr: [x, y]}
    }

    pub fn x(&self) -> f64 {
        self.arr[0]
    }

    pub fn y(&self) -> f64 {
        self.arr[1]
    }

    pub fn unit_vector(self) -> Vec2{
        self / (self.length())
    }

    pub fn length_squared(&self) -> f64{
        self.x().powi(2) + self.y().powi(2)
    }

    pub fn length(&self) -> f64{
        self.length_squared().sqrt()
    }

    pub fn round(&self) -> Vec2 {
        Vec2::new(self[0].round(), self[1].round())
    }

}

//Operator overloading using impl_ops
impl_op_ex_commutative!(+ |lhs: f64, rhs: Vec2| -> Vec2 { Vec2::new(rhs.x() + lhs, rhs.y() + lhs)});
impl_op_ex!(+ |lhs: Vec2, rhs: Vec2| -> Vec2 { Vec2::new(lhs.x() + rhs.x(), lhs.y() + rhs.y())});

impl_op_ex!(- |lhs: f64, rhs: Vec2| -> Vec2 { Vec2::new(lhs - rhs.x(), lhs - rhs.y())});
impl_op_ex!(- |lhs: Vec2, rhs: f64| -> Vec2 { Vec2::new(lhs.x() - rhs, lhs.y() - rhs)});
impl_op_ex!(- |lhs: Vec2, rhs: Vec2| -> Vec2 { Vec2::new(lhs.x() - rhs.x(), lhs.y() - rhs.y())});

impl_op_ex_commutative!(* |lhs: f64, rhs: Vec2| -> Vec2 { Vec2::new(rhs.x() * lhs, rhs.y() * lhs)});
impl_op_ex!(/ |lhs: Vec2, rhs: f64| -> Vec2 { Vec2::new(lhs.x() / rhs, lhs.y() / rhs)});

impl Index<usize> for Vec2{
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.arr[index]
    }
}
impl ops::Neg for Vec2{
    type Output = Vec2;
    fn neg(self) -> Vec2{
        Vec2::new(-self.x(), -self.y())
    }
}

impl ops::Neg for &Vec2{
    type Output = Vec2;
    fn neg(self) -> Vec2{
        Vec2::new(-self.x(), -self.y())
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3{
    pub fn new(x: f64, y: f64, z:f64) -> Vec3{
        Vec3{arr: [x,y,z]}
    }

    pub fn rand(min: f64, max:f64) -> Vec3{
        Vec3{arr:[rand_double(min, max), rand_double(min, max), rand_double(min, max)]}
    }

    pub fn rand_in_unit_sphere() -> Vec3{
        loop{
            let p = Vec3::rand(-1.0, 1.0);
            if p.length_squared() < 1.0{
                break(p)
            }
        }
    }

    pub fn rand_in_unit_disk() -> Vec3{
        loop{
            let p = Vec3::new(rand_double(-1.0, 1.0), rand_double(-1.0, 1.0), 0.0);
            if p.length_squared() < 1.0{
                break(p)
            }
        }
    }

    pub fn rand_unit_vec() -> Vec3{
        Vec3::rand_in_unit_sphere().unit_vector()
    }

    pub fn x(&self) -> f64{
        self.arr[0]
    }

    pub fn y(&self) -> f64{
        self.arr[1]
    }

    pub fn z(&self) -> f64{
        self.arr[2]
    }

    pub fn index(&self, index: usize) -> f64{
        self.arr[index]
    }

    pub fn length(self) -> f64{
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64{
        self.x().powi(2) + self.y().powi(2) + self.z().powi(2)
    }

    pub fn dot(self, rhs: Vec3) -> f64{
        self.x()*rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    pub fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&f64, &f64) -> Ordering,
    {
        self.arr.sort_by(compare);
    }

    pub fn max_dim(self) -> usize{
        self.arr.iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.abs().partial_cmp(&b.abs())
                .expect("Tried to compare a NaN"))
                .map(|(index, _)| index)
                .unwrap()
    }

    pub fn swap(&mut self, i: usize, j: usize){
        self.arr.swap(i,j);
    }

    pub fn unit_vector(self) -> Vec3{
        self/(self.length())
    }

    pub fn cross(self, rhs: Vec3) -> Vec3{
        Vec3::new(self.y()*rhs.z() - self.z()*rhs.y(), 
                  self.z()*rhs.x() - self.x()*rhs.z(),
                  self.x()*rhs.y() - self.y()*rhs.x())
    }

    pub fn near_zero(self) -> bool{
        let min = 1e-8;
        self.x().abs() < min && self.y().abs() < min && self.z().abs() < min
    }

    pub fn reflect(self, normal: Vec3) -> Vec3{
        self - 2.0 * self.dot(normal)*normal
    }

    pub fn elementwise_mult(&self, rhs: &Vec3) -> Vec3{
        Vec3::new(self.x()*rhs.x(), self.y()*rhs.y(), self.z()*rhs.z())
    }

    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3{
        let cos_theta = -uv.dot(n).min(1.0);
        let r_out_perp = etai_over_etat*(uv + cos_theta*n);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
        r_out_perp + r_out_parallel
    }

    //Applies the absolute value function to vector components
    pub fn abs(&self) -> Vec3{
        Vec3::new(self.x().abs(), self.y().abs(), self.z().abs())
    }

    pub fn offset_origin(&self, dir: Point3,  p_err: Vec3, norm: Vec3) -> Point3{
        let d = norm.abs().dot(p_err);
        let mut offset = d*p_err;
        if dir.dot(norm) < 0.0{
            offset = -offset;
        }
        self + offset
    }
}

//Operator overloading using impl_ops
impl_op_ex_commutative!(+ |lhs: &f64, rhs: &Vec3| -> Vec3 { Vec3::new(rhs.x() + lhs, rhs.y() + lhs, rhs.z() + lhs)});
impl_op_ex!(+ |lhs: &Vec3, rhs: &Vec3| -> Vec3 { Vec3::new(lhs.x() + rhs.x(), lhs.y() + rhs.y(), lhs.z() + rhs.z())});

impl_op_ex!(- |lhs: &f64, rhs: &Vec3| -> Vec3 { Vec3::new(lhs - rhs.x(), lhs - rhs.y(), lhs - rhs.z())});
impl_op_ex!(- |lhs: &Vec3, rhs: &f64| -> Vec3 { Vec3::new(lhs.x() - rhs, lhs.y() - rhs, lhs.z() - rhs)});
impl_op_ex!(- |lhs: &Vec3, rhs: &Vec3| -> Vec3 { Vec3::new(lhs.x() - rhs.x(), lhs.y() - rhs.y(), lhs.z() - rhs.z())});

impl_op_ex_commutative!(* |lhs: &f64, rhs: &Vec3| -> Vec3 { Vec3::new(rhs.x() * lhs, rhs.y() * lhs, rhs.z() * lhs)});
impl_op_ex!(/ |lhs: &Vec3, rhs: &f64| -> Vec3 { Vec3::new(lhs.x() / rhs, lhs.y() / rhs, lhs.z() / rhs)});

impl ops::Neg for Vec3{
    type Output = Vec3;
    fn neg(self) -> Vec3{
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl ops::Neg for &Vec3{
    type Output = Vec3;
    fn neg(self) -> Vec3{
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl Index<usize> for Vec3{
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.arr[index]
    }
}

impl IndexMut<usize> for Vec3{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.arr[index]
    }
}


impl Color{

    pub fn write_color<T: std::io::Write>(self, writer: &mut T, samples: usize)
    {
        let mut r = self.x();
        let mut g = self.y();
        let mut b = self.z();

        let scale = 1.0/(samples as f64);
        r = (scale*r).sqrt();
        g = (scale*g).sqrt();
        b = (scale*b).sqrt();
        

        let ir = (256.0*bound(r, 0.0, 0.999)) as i64;
        let ig = (256.0*bound(g, 0.0, 0.999)) as i64;
        let ib = (256.0*bound(b, 0.0, 0.999)) as i64;
        writeln!(writer, "{} {} {}", ir, ig, ib).unwrap();
    }

    pub fn scale_colors(&self, samples: usize) -> [u8; 3]{
        let mut r = self.x();
        let mut g = self.y();
        let mut b = self.z();

        let scale = 1.0/(samples as f64);
        r = (scale*r).sqrt();
        g = (scale*g).sqrt();
        b = (scale*b).sqrt();
        

        let ir = (256.0*bound(r, 0.0, 0.999)) as u8;
        let ig = (256.0*bound(g, 0.0, 0.999)) as u8;
        let ib = (256.0*bound(b, 0.0, 0.999)) as u8;
        [ir, ig, ib]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new(){
        let result = Vec3{arr:[1.0, 2.0, 3.0]};
        assert_eq!(Vec3::new(1.0,2.0,3.0), result);
    }

    #[test]
    fn test_x(){
        let vec =  Vec3{arr:[1.0, 2.0, 3.0]};
        assert_eq!(vec.x(), 1.0);
    }

    #[test]
    fn test_y(){
        let vec = Vec3{arr:[1.0, 2.0, 3.0]};
        assert_eq!(vec.y(), 2.0);
    }

    #[test]
    fn test_z(){
        let vec =Vec3{arr:[1.0, 2.0, 3.0]};
        assert_eq!(vec.z(), 3.0);
    }

    #[test]
    fn test_vec_add(){
        let lhs = Vec3::new(1.0,2.0,3.0);
        let rhs = Vec3::new(4.0,2.0,-1.0);
        let result = Vec3::new(5.0,4.0,2.0);
        assert_eq!(lhs + rhs, result);
    }

    #[test]
    fn test_scalar_add(){
        let lhs = Vec3::new(1.0,2.0,3.0);
        let rhs = 2.0;
        let result = Vec3::new(3.0,4.0,5.0);
        assert_eq!(lhs + rhs, result);
    }

    #[test]
    fn test_vec_sub(){
        let lhs = Vec3::new(1.0,2.0,3.0);
        let rhs = Vec3::new(4.0,2.0,-1.0);
        let result = Vec3::new(-3.0,0.0,4.0);
        assert_eq!(lhs - rhs, result);
    }

    #[test]
    fn test_scalar_sub(){
        let lhs = Vec3::new(1.0,2.0,3.0);
        let rhs = 2.0;
        let result = Vec3::new(-1.0,0.0,1.0);
        assert_eq!(lhs - rhs, result);
    }

    #[test]
    fn test_scalar_mul(){
        let lhs = Vec3::new(1.0,2.0,3.0);
        let rhs = 5.0;
        let result = Vec3::new(5.0,10.0,15.0);
        assert_eq!(lhs * rhs, result);
    }

    #[test]
    fn test_scalar_div(){
        let lhs = Vec3::new(1.0,2.0,3.0);
        let rhs = 5.0;
        let result = Vec3::new(0.2,0.4,0.6);
        assert_eq!(lhs / rhs, result);
    }

    #[test]
    fn test_dot(){
        let lhs = Vec3::new(1.0,2.0,3.0);
        let rhs = Vec3::new(4.0,2.0,-1.0);
        let result = 5.0;
        assert_eq!(lhs.dot(rhs), result);
    }

    #[test]
    fn test_length(){
        let lhs = Vec3::new(4.0,3.0,0.0);
        let result = 5.0;
        assert_eq!(lhs.length(), result);
    }

    #[test]
    fn test_length_squared(){
        let lhs = Vec3::new(1.0,2.0,3.0);
        let result = 14.0;
        assert_eq!(lhs.length_squared(), result);
    }

    #[test]
    fn test_unit_vector(){
        let vec = Vec3::new(2.0,0.0,0.0);
        let result = Vec3::new(1.0,0.0,0.0);
        assert_eq!(vec.unit_vector(), result);
    }

    #[test]
    fn test_cross(){
        let lhs = Vec3::new(3.0,-3.0,1.0);
        let rhs = Vec3::new(4.0,9.0,2.0);
        let result = Vec3::new(-15.0, -2.0, 39.0);
        assert_eq!(lhs.cross(rhs), result);
    }

    #[test]
    fn test_neg(){
        let vec = Vec3::new(3.0,-3.0,1.0);
        let result = Vec3::new(-3.0, 3.0, -1.0);
        assert_eq!(-vec, result);
    }

    #[test]
    fn test_rand_unit_sphere(){
        let vec_1 = Vec3::rand_unit_vec();
        assert!(vec_1.length() <= 1.0);
        let vec_2 = Vec3::rand_unit_vec();
        assert!(vec_1 != vec_2);
    }

    #[test]
    fn test_elementwise_mult(){
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(-1.0, 2.0, -3.0);
        assert_eq!(Vec3::new(-1.0, 4.0, -9.0), vec1.elementwise_mult(&vec2));
    }

    #[test]
    fn test_near_zero(){
        let vec1 = Vec3::new(1e-8 - 1e-9, 1e-8 - 1e-9, 1e-8 - 1e-9);
        let vec2 = Vec3::new(1e-8, 1e-8, 1e-8);
        assert_eq!(vec1.near_zero(), true);
        assert_eq!(vec2.near_zero(), false);
    }

    #[test]
    fn test_reflect(){
        let vec = Vec3::new(1.0, 0.0, 0.0);
        let normal = Vec3::new(-1.0, 0.0, 0.0);
        assert_eq!(vec.reflect(normal), Vec3::new(-1.0, 0.0, 0.0));
    }

    
    #[test]
    fn test_refract(){
        let vec = Vec3::new(0.5, 0.5, 0.0);
        let normal = Vec3::new(1.0, 0.0, 0.0);
        let reflective_index = 1.5;
        assert_eq!(Vec3::refract(vec, normal, reflective_index), Vec3::new(-(1.0-0.75f64.powi(2)).sqrt(), 0.75, 0.0));
    }

    #[test]
    fn test_swap(){
        let mut vec = Vec3::new(0.1, 0.2, 0.3);
        vec.swap(0, 2);
        assert_eq!(vec, Vec3::new(0.3, 0.2, 0.1));

        vec.swap(1, 2);
        assert_eq!(vec, Vec3::new(0.3, 0.1, 0.2));

        vec.swap(1, 1);
        assert_eq!(vec, Vec3::new(0.3, 0.1, 0.2));
    }

    #[test]
    fn test_max_dim(){
        let vec = Vec3::new(0.4, 0.3, 0.2);
        assert_eq!(vec.max_dim(), 0);

        let vec = Vec3::new(8.0, -9.0, 0.0);
        assert_eq!(vec.max_dim(), 1);
    }
}
