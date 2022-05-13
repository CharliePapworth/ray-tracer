use crate::util::lerp;

use super::coefficients::Coefficients;
use super::constants::*;
use std::iter::zip;

const FIRST_WAVELENGTH: f32 = 400.0;
const LAST_WAVELENGTH: f32 = 700.0;
const SPECTRAL_SAMPLES: usize = 60;

pub struct MatchingCurve {
    pub coefficients: Coefficients<SPECTRAL_SAMPLES>
}

impl MatchingCurve {
        /// Initialises a MatchingCurve with coefficients of a constant value. Not for public consumption.
        fn new(constant: f32) -> MatchingCurve {
            let coefficients = Coefficients::<SPECTRAL_SAMPLES>::new(constant);
            MatchingCurve { coefficients } 
        }

        /// Initialises the three matching curves: X, Y & Z. These are referenced by each instance of SampledSpectrum,
        /// and must therefore have a longer lifetime than every instance. Initialising them in main() is therefore recommended.
        pub fn init_xyz<'a>() -> [MatchingCurve; 3] {
            let mut i = 0;
            let spectral_samples_f32 = SPECTRAL_SAMPLES as f32;
            let mut x = MatchingCurve::new(0.0);
            let mut y = MatchingCurve::new(0.0);
            let mut z = MatchingCurve::new(0.0);

            while i < SPECTRAL_SAMPLES {
                let i_f32 = i as f32;
                let next_i_f32 = (i + 1) as f32;
                let from_wavelength = lerp(FIRST_WAVELENGTH, LAST_WAVELENGTH, i_f32 / spectral_samples_f32);
                let to_wavelength =  lerp(FIRST_WAVELENGTH, LAST_WAVELENGTH, next_i_f32 / spectral_samples_f32);
                x.coefficients[i] = average_samples(&CIE_X.to_vec(), &CIE_LAMBDA.to_vec(), from_wavelength, to_wavelength);
                y.coefficients[i] = average_samples(&CIE_Y.to_vec(), &CIE_LAMBDA.to_vec(), from_wavelength, to_wavelength);
                z.coefficients[i] = average_samples(&CIE_Z.to_vec(), &CIE_LAMBDA.to_vec(), from_wavelength, to_wavelength);
                i += 1;
            }
            [x, y, z]
        }
}

/// SampledSpectrum<'a> uses the Coefficients infrastructure to represent an SPD with uniformly spaced samples
/// between a starting and an ending wavelength. The wavelength range covers from 400 nm to 700 nm—the range of the
///  visual spectrum where the human visual system is most sensitive. 
pub struct SampledSpectrum<'a> {
    pub coefficients: Coefficients<SPECTRAL_SAMPLES>,
    matching_curves: &'a [SampledSpectrum<'a>; 3]
}

impl<'a> SampledSpectrum<'a> {

    /// Initialises a SampledSpectrum with coefficients of a constant value.
    ///     
    /// A reference to the matching curves, calculated by MatchingCurve::init_xyz, must be passed into the constructor.
    pub fn new(constant: f32, matching_curves: &'a[SampledSpectrum<'a>; 3]) -> SampledSpectrum<'a> {
        let coefficients = Coefficients::<SPECTRAL_SAMPLES>::new(constant);
        SampledSpectrum { coefficients, matching_curves } 
    }
    
    /// Takes arrays of SPD sample values at given wavelengths lambda and uses them to 
    /// define a piecewise linear function to represent the SPD.
    /// 
    /// A reference to the matching curves, calculated by MatchingCurve::init_xyz, must be passed into the constructor.
    pub fn from_sampled(sample_values: Vec<f32>, sample_wavelengths: Vec<f32>, matching_curves: &'a [SampledSpectrum<'a>; 3]) -> SampledSpectrum<'a> {
        let mut spectrum = SampledSpectrum::<'a>::new(0.0, matching_curves);
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
        for i in 0..SPECTRAL_SAMPLES {
            let from_wavelength = lerp((i as f32) / (SPECTRAL_SAMPLES as f32), FIRST_WAVELENGTH, LAST_WAVELENGTH);
            let to_wavelength = lerp(((i as f32) + 1.0) / (SPECTRAL_SAMPLES as f32), FIRST_WAVELENGTH, LAST_WAVELENGTH);
            spectrum.coefficients[i] = average_samples(&sample_values, &sample_wavelengths, from_wavelength, to_wavelength);
        }
        spectrum
    }

    pub fn from_coefficients(coefficients: Coefficients<SPECTRAL_SAMPLES>, matching_curves: &'a [SampledSpectrum<'a>; 3]) -> SampledSpectrum<'a> {
        SampledSpectrum { coefficients, matching_curves }
    }

    /// Computes X, Y & Z coefficients. This is calculated by integrating the product of the matching curves
    /// with the sampled spectrum.
    pub fn to_xyz(&self) -> [f32; 3] {
        let mut xyz = [0.0; 3];
        for i in 0..SPECTRAL_SAMPLES {
            xyz[0] += self.matching_curves[0].coefficients[i] * self.coefficients[i];
            xyz[1] += self.matching_curves[1].coefficients[i] * self.coefficients[i];
            xyz[2] += self.matching_curves[2].coefficients[i] * self.coefficients[i];
        }
        let scale = (LAST_WAVELENGTH - FIRST_WAVELENGTH as f32) / (CIE_Y_INTEGRAL * SPECTRAL_SAMPLES as f32);
        xyz[0] *= scale;
        xyz[1] *= scale;
        xyz[2] *= scale;
        xyz        
    }
}

/// Compute the average of the piecewise linear function over the range of wavelengths that each
/// SPD sample is responsible for. The samples (submitted as two seperate vectors, containing the values for each wavelength)
/// must be sorted.
fn average_samples(values: &Vec<f32>, wavelengths: &Vec<f32>, from_wavelength: f32, to_wavelength: f32) -> f32 {
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

    // Advance to first relevant wavelength segment
    let mut i = 0;
    while from_wavelength > values[i + 1] {
        i += 1;
    } 

    let interp = |wavelength: f32, i: usize| -> f32 {
        lerp(values[i], values[i + 1], (wavelength - wavelengths[i]) / (wavelengths[i + 1] - wavelengths[i]))
    };

    // Loop over wavelength segments and add contributions
    while i + 1 <= wavelengths.len()  && to_wavelength >= wavelengths[i] {
        let segment_start = from_wavelength.max(wavelengths[i]);
        let segment_end = to_wavelength.min(wavelengths[i + 1]);
        sum += 0.5 * (interp(segment_start, i) + interp(segment_end, i)) * (segment_end - segment_start);
        i += 1;
    }

    sum / (to_wavelength - from_wavelength)

}
