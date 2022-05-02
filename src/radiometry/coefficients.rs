use std::ops::{Index, IndexMut, Neg};
use std::iter::zip;
use std::ops;

use crate::util::bound_f32;

const COEFFICIENTCOUNT: usize = 60;

/// Infrastructure underpining the spectrum implementations. Each implementation contains an array of coefficients, on
/// which they need to make some common operations. 
pub struct Coefficients {
    arr: [f32; COEFFICIENTCOUNT],
}

impl Coefficients {
    pub fn new(constant: f32) -> Coefficients {
        let coefficients = [constant; COEFFICIENTCOUNT];
        Coefficients { arr: coefficients } 
    }

    fn elementwise_binary_operation(&self, rhs: &Coefficients, operation: fn(f32, f32) -> f32) -> Coefficients {
        let mut coefficients = [0f32; COEFFICIENTCOUNT];
        for i in 0..COEFFICIENTCOUNT {
            coefficients[i] = operation(self.arr[i], rhs.arr[i]);
        }
        Coefficients { arr: coefficients } 
    }

    fn elementwise_binary_operation_in_place(&mut self, rhs: &Coefficients, operation: fn(f32, f32) -> f32) {
        for i in 0..COEFFICIENTCOUNT {
            self.arr[i] = operation(self.arr[i], rhs.arr[i]);
        }
    }

    fn unary_operation_in_place<F>(&mut self, operation: F) where F: Fn(f32) -> f32 {
        for i in 0..COEFFICIENTCOUNT {
            self.arr[i] = operation(self.arr[i]);
        }
    }

    pub fn powi(&self, exponent: i32) -> Coefficients {
        Coefficients { arr: self.arr.map(|a| a.powi(exponent)) }
    }

    pub fn exp(&self) -> Coefficients {
        Coefficients { arr: self.arr.map(|a| a.exp()) }
    }

    pub fn sqrt(&self) -> Coefficients {
        Coefficients { arr: self.arr.map(|a| a.sqrt()) }
    }

    pub fn lerp(&self, other: &Coefficients, t: f32) -> Coefficients {
        (1.0 - t) * self + t * other
    }

    pub fn is_black(&self) -> bool {
        self.arr.iter().any(|a| a != &0f32)
    }

    pub fn has_nan(&self) -> bool {
        self.arr.iter().any(|a| a.is_nan())
    }

    pub fn clamp(&self, min: f32, max: f32) -> Coefficients {
        Coefficients{ arr: self.arr.map(|a| bound_f32(a, min, max)) }
    }
}

impl Index<usize> for Coefficients {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.arr[index]
    }
}

impl IndexMut<usize> for Coefficients {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.arr[index]
    }
}

impl Neg for Coefficients {
    type Output = Self;

    fn neg(self) -> Self {
        Coefficients { arr: self.arr.map(|a| - a) }
    }
}

impl_op_ex!(+ |lhs: &Coefficients, rhs: &Coefficients| -> Coefficients {
        lhs.elementwise_binary_operation(&rhs, |a, b| a + b)
    }
);

impl_op_ex!(+= |lhs: &mut Coefficients, rhs: &Coefficients| {
        lhs.elementwise_binary_operation_in_place(&rhs, |a, b| a + b);
    }
);

impl_op_ex!(- |lhs: &Coefficients, rhs: &Coefficients| -> Coefficients {
        lhs.elementwise_binary_operation(&rhs, |a, b| a - b)
    }
);

impl_op_ex!(-= |lhs: &mut Coefficients, rhs: Coefficients| {
        lhs.elementwise_binary_operation_in_place(&rhs, |a, b| a - b);
    }
);

impl_op_ex!(* |lhs: &Coefficients, rhs: &Coefficients| -> Coefficients {
        lhs.elementwise_binary_operation(&rhs, |a, b| a * b)
    }
);

impl_op_ex!(*= |lhs: &mut Coefficients, rhs: Coefficients| {
        lhs.elementwise_binary_operation_in_place(&rhs, |a, b| a * b);
    }
);

impl_op_ex_commutative!(* |lhs: &Coefficients, rhs: &f32| -> Coefficients {
        Coefficients { arr: lhs.arr.map(|a| a * rhs) }
    }
);

impl_op_ex!(*= |lhs: &mut Coefficients, rhs: &f32| {
        lhs.unary_operation_in_place(|a| a * rhs);
    }
);

impl_op_ex!(/ |lhs: &Coefficients, rhs: &Coefficients| -> Coefficients {
        lhs.elementwise_binary_operation(&rhs, |a, b| a / b)
    }
);

impl_op_ex!(/= |lhs: &mut Coefficients, rhs: &Coefficients| {
        lhs.elementwise_binary_operation_in_place(&rhs, |a, b| a / b);
    }
);