use std::ops::{Index, IndexMut, Neg, Add, Sub, Mul, AddAssign, SubAssign, Div};
use std::iter::zip;
use std::ops;

use crate::util::bound_f32;

/// Infrastructure underpining the spectrum implementations. Each implementation contains an array of coefficients, on
/// which they need to make some common operations. 
pub struct Coefficients<const T: usize> {
    arr: [f32; T],
}

impl<const T: usize> Coefficients<T> {
    pub fn new(constant: f32) -> Coefficients<T> {
        let coefficients = [constant; T];
        Coefficients { arr: coefficients } 
    }

    fn elementwise_binary_operation(&self, rhs: &Coefficients<T>, operation: fn(f32, f32) -> f32) -> Coefficients<T> {
        let mut coefficients = [0f32; T];
        for i in 0..T {
            coefficients[i] = operation(self.arr[i], rhs.arr[i]);
        }
        Coefficients { arr: coefficients } 
    }

    fn elementwise_binary_operation_in_place(&mut self, rhs: &Coefficients<T>, operation: fn(f32, f32) -> f32) {
        for i in 0..T {
            self.arr[i] = operation(self.arr[i], rhs.arr[i]);
        }
    }

    fn unary_operation_in_place<F>(&mut self, operation: F) where F: Fn(f32) -> f32 {
        for i in 0..T {
            self.arr[i] = operation(self.arr[i]);
        }
    }

    pub fn elementwise_multiplication(&self, other: &Coefficients<T>) {
        self.elementwise_binary_operation(&other, |a, b| a * b)
    }

    pub fn elementwise_division(&self, other: &Coefficients<T>) {
        self.elementwise_binary_operation(&other, |a, b| a * b)
    }

    pub fn powi(&self, exponent: i32) -> Coefficients<T> {
        Coefficients { arr: self.arr.map(|a| a.powi(exponent)) }
    }

    pub fn exp(&self) -> Coefficients<T> {
        Coefficients { arr: self.arr.map(|a| a.exp()) }
    }

    pub fn sqrt(&self) -> Coefficients<T> {
        Coefficients { arr: self.arr.map(|a| a.sqrt()) }
    }

    pub fn lerp(&self, other: &Coefficients<T>, t: f32) -> Coefficients<T> {
        (1.0 - t) * self + t * other
    }

    pub fn is_black(&self) -> bool {
        self.arr.iter().any(|a| a != &0f32)
    }

    pub fn has_nan(&self) -> bool {
        self.arr.iter().any(|a| a.is_nan())
    }

    pub fn clamp(&self, min: f32, max: f32) -> Coefficients<T> {
        Coefficients{ arr: self.arr.map(|a| bound_f32(a, min, max)) }
    }
}

impl<const T: usize> Index<usize> for Coefficients<T> {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.arr[index]
    }
}

impl<const T: usize> IndexMut<usize> for Coefficients<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.arr[index]
    }
}

// Vector Operations
impl<const T: usize> Neg for Coefficients<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Coefficients { arr: self.arr.map(|a| - a) }
    }
}

impl<const T: usize> Add for Coefficients<T> {
    type Output = Coefficients<T>;

    fn add(self, other: Coefficients<T>) -> Coefficients<T> {
        self.elementwise_binary_operation(&other, |a, b| a + b)
    }
}

impl<const T: usize> Add for &Coefficients<T> {
    type Output = Coefficients<T>;

    fn add(self, other: &Coefficients<T>) -> Coefficients<T> {
        self.elementwise_binary_operation(&other, |a, b| a + b)
    }
}

impl<const T: usize> Add for Coefficients<T> {
    type Output = Coefficients<T>;

    fn add(&self, other: &Coefficients<T>) -> Coefficients<T> {
        self.elementwise_binary_operation(&other, |a, b| a + b)
    }
}

impl<const T: usize> Add for &Coefficients<T> {
    type Output = Coefficients<T>;

    fn add(&self, other: &Coefficients<T>) -> Coefficients<T> {
        self.elementwise_binary_operation(&other, |a, b| a + b)
    }
}

impl<const T: usize> AddAssign for Coefficients<T> {
    fn add_assign(&mut self, other: Self) {
        self.elementwise_binary_operation_in_place(&other, |a, b| a + b)
    }
}

impl<const T: usize> AddAssign for &Coefficients<T> {
    fn add_assign(&mut self, other: Self) {
        self.elementwise_binary_operation_in_place(&other, |a, b| a + b)
    }
}

impl<const T: usize> Sub for Coefficients<T> {
    type Output = Coefficients<T>;

    fn sub(self, other: Coefficients<T>) -> Coefficients<T> {
        self.elementwise_binary_operation(&other, |a, b| a - b)
    }
}

impl<const T: usize> Sub for &Coefficients<T> {
    type Output = Coefficients<T>;

    fn sub(self, other: &Coefficients<T>) -> Coefficients<T> {
        self.elementwise_binary_operation(&other, |a, b| a - b)
    }
}

impl<const T: usize> Sub for Coefficients<T> {
    type Output = Coefficients<T>;

    fn sub(&self, other: &Coefficients<T>) -> Coefficients<T> {
        self.elementwise_binary_operation(&other, |a, b| a - b)
    }
}

impl<const T: usize> Sub for &Coefficients<T> {
    type Output = Coefficients<T>;

    fn sub(&self, other: &Coefficients<T>) -> Coefficients<T> {
        self.elementwise_binary_operation(&other, |a, b| a - b)
    }
}

impl<const T: usize> SubAssign for Coefficients<T> {
    fn sub_assign(&mut self, other: Self) {
        self.elementwise_binary_operation_in_place(&other, |a, b| a - b)
    }
}

impl<const T: usize> SubAssign for &Coefficients<T> {
    fn sub_assign(&mut self, other: Self) {
        self.elementwise_binary_operation_in_place(&other, |a, b| a - b)
    }
}


// Scalar Operations
impl<const T: usize> Mul<f32> for Coefficients<T> {
    type Output = Coefficients<T>;

    fn mul(self, scalar: f32) -> Coefficients<T> {
        self.arr.map(|a| a * scalar)
    }
}

impl<const T: usize> Mul<f32> for &Coefficients<T> {
    type Output = Coefficients<T>;

    fn mul(&self, scalar: f32) -> Coefficients<T> {
        self.arr.map(|a| a * scalar)
    }
}

impl<const T: usize> Mul<&f32> for Coefficients<T> {
    type Output = Coefficients<T>;

    fn mul(self, scalar: &f32) -> Coefficients<T> {
        self.arr.map(|a| a * scalar)
    }
}

impl<const T: usize> Mul<&f32> for &Coefficients<T> {
    type Output = Coefficients<T>;

    fn mul(&self, scalar: &f32) -> Coefficients<T> {
        self.arr.map(|a| a * scalar)
    }
}

impl<const T: usize> Div<f32> for Coefficients<T> {
    type Output = Coefficients<T>;

    fn div(self, scalar: f32) -> Coefficients<T> {
        self.arr.map(|a| a / scalar)
    }
}

impl<const T: usize> Div<f32> for &Coefficients<T> {
    type Output = Coefficients<T>;

    fn div(&self, scalar: f32) -> Coefficients<T> {
        self.arr.map(|a| a / scalar)
    }
}

impl<const T: usize> Div<&f32> for Coefficients<T> {
    type Output = Coefficients<T>;

    fn div(self, scalar: &f32) -> Coefficients<T> {
        self.arr.map(|a| a * scalar)
    }
}

impl<const T: usize> Div<&f32> for &Coefficients<T> {
    type Output = Coefficients<T>;

    fn div(&self, scalar: &f32) -> Coefficients<T> {
        self.arr.map(|a| a * scalar)
    }
}