/// Acts like a regular float, using operator overloading 
/// to provide all of the regular arithmetic operations on
///  floats while computing associated error bounds. 
pub struct Ef32 {
    pub value: f32,
    pub error_bound: f32
}

impl Ef32 {
    pub fn new(value: f32, error_bound: f32) -> Ef32 {
        Ef32 {value, error_bound}
    }

    pub fn with_value(value: f32) -> Ef32 {
        let error_bound = 0.0;
        Ef32 {value, error_bound}
    }
}
