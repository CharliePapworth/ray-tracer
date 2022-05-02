use super::coefficients::Coefficients;
use std::iter::zip;

/// SampledSpectrum uses the Coefficients infrastructure to represent an SPD with uniformly spaced samples
/// between a starting and an ending wavelength. The wavelength range covers from 400 nm to 700 nm—the range of the
///  visual spectrum where the human visual system is most sensitive. 
pub struct SampledSpectrum {
    pub coefficients: Coefficients
}

impl SampledSpectrum {

    /// Initialises a SampledSpectrum with coefficients of a constant value.
    pub fn new(constant: f32) -> SampledSpectrum {
        let coefficients = Coefficients::new(constant);
        SampledSpectrum { coefficients } 
    }
    
    /// Takes arrays of SPD sample values at given wavelengths lambda and uses them to 
    /// define a piecewise linear function to represent the SPD.
    pub fn from_sampled(sample_values: Vec<f32>, sample_wavelengths: Vec<f32>, ) {
        if sample_values.len() == 0 || sample_wavelengths.len() == 0 {
            panic!("One of the inputted vectors has length zero.")
        }

        if sample_values.len() != sample_wavelengths.len() {
            panic!("Vector dimensions do not match.");
        }

        if sample_values.iter().any(|a| !a.is_finite()) || sample_wavelengths.iter().any(|a| !a.is_finite()) {
            panic!("Vectors contain non-finite values.")
        }
        

        // Zip the sample values up into a single vector.
        let mut sample_dictionary: Vec<(f32, f32)> = sample_values.iter().zip(sample_wavelengths.iter())
                                                                         .map(|(a,b)| (*a, *b))
                                                                         .collect();
        //Sort the sample values from lowest wavelength to highest.
        sample_dictionary.sort_by(|a,b| a.0.partial_cmp(&b.0).unwrap());
    }
}

/// Compute the average of the piecewise linear function over the range of wavelengths that each
/// SPD sample is responsible for. The samples (submitted as two seperate vectors, containing the values for each wavelength)
/// must be sorted.
fn average_samples(values: Vec<f32>, wavelengths: Vec<f32>, from_wavelength: f32, to_wavelength: f32) -> f32 {
    if to_wavelength <= wavelengths[0] {
        return values[0];
    }

    if from_wavelength >= *wavelengths.last().unwrap() {
        return values[values.len() - 1];
    }

    if wavelengths.len() == 1 {
        return values[0];
    }

    let mut sum = 0f32;

    //Add contributions of constant segments before/after samples
    if from_wavelength < wavelengths[0] {
        sum += values[0] * (wavelengths[0] - from_wavelength)
    }

    if to_wavelength > *wavelengths.last().unwrap() {
        sum += values.last().unwrap() * (from_wavelength - wavelengths.last().unwrap())
    }


    todo!()

}
