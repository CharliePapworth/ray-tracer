pub mod constant_spectra;
pub mod spectrum_factory;

use nalgebra::{Vector, SVector, Const};

use crate::util::{lerp, bound_f32};
use crate::image::Color;

use constant_spectra::*;
use std::iter::zip;
use std::ops::{Mul, Div};

const FIRST_WAVELENGTH: f32 = 400.0;
const LAST_WAVELENGTH: f32 = 700.0;
const SPECTRAL_SAMPLES: usize = 60;


pub enum SpectrumType {
    Reflectance,
    Illuminant
}



/// SampledSpectrum uses the Coefficients infrastructure to represent an SPD with uniformly spaced samples
/// between a starting and an ending wavelength. The wavelength range covers from 400 nm to 700 nm—the range of the
///  visual spectrum where the human visual system is most sensitive. 
#[derive (Copy, Clone)]
pub struct Spectrum {
    pub coefficients: SVector<f32, SPECTRAL_SAMPLES>
}

impl Default for Spectrum {
    fn default() -> Self { Spectrum::new(0.0) }
}

impl Mul<f32> for Spectrum {
    type Output = Spectrum;

    fn mul(self, rhs: f32) -> Self::Output {
        Spectrum::from_coefficients(self.coefficients * rhs)
    }
}

impl Div<f32> for Spectrum {
    type Output = Spectrum;

    fn div(self, rhs: f32) -> Self::Output {
        Spectrum::from_coefficients(self.coefficients / rhs)
    }
}

impl Mul<&Spectrum> for f32 {
    type Output = Spectrum;

    fn mul(self, rhs: &Spectrum) -> Self::Output {
        Spectrum::from_coefficients(rhs.coefficients * self)
    }
}

impl Mul<&Spectrum> for &Spectrum {
    type Output = Spectrum;

    fn mul(self, rhs: &Spectrum) -> Self::Output {
        Spectrum::from_coefficients(rhs.coefficients.component_mul(&self.coefficients))
    }
}

impl Spectrum {

    /// Initialises a SampledSpectrum with coefficients of a constant value.
    pub fn new(constant: f32) -> Spectrum {
        let coefficients = SVector::<f32, SPECTRAL_SAMPLES>::repeat(0.0);
        Spectrum { coefficients } 
    }
    
    /// Takes arrays of SPD sample values at given wavelengths lambda and uses them to 
    /// define a piecewise linear function to represent the SPD.
    pub fn from_sampled(sample_values: Vec<f32>, sample_wavelengths: Vec<f32>) -> Spectrum {
        let mut spectrum = Spectrum::new(0.0);
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

    pub fn from_coefficients(coefficients: SVector<f32, SPECTRAL_SAMPLES>) -> Spectrum {
        Spectrum { coefficients }
    }

    /// Computes X, Y & Z coefficients. This is calculated by integrating the product of the matching curves
    /// with the sampled spectrum.
    pub fn get_xyz(&self, constant_spectra: &ConstantSpectra) -> [f32; 3] {
        let mut xyz = [0.0; 3];
        for i in 0..SPECTRAL_SAMPLES {
            xyz[0] += constant_spectra.x.coefficients[i] * self.coefficients[i];
            xyz[1] += constant_spectra.y.coefficients[i] * self.coefficients[i];
            xyz[2] += constant_spectra.z.coefficients[i] * self.coefficients[i];
        }
        let scale = (LAST_WAVELENGTH - FIRST_WAVELENGTH as f32) / (CIE_Y_INTEGRAL * SPECTRAL_SAMPLES as f32);
        xyz[0] *= scale;
        xyz[1] *= scale;
        xyz[2] *= scale;
        xyz        
    }

    

    ///The Y coefficient of XYZ color is closely related to luminance, which measures the perceived brightness of a color. 
    /// This method calculates the Y coefficient alone.
    pub fn get_y(&self, constant_spectra: &ConstantSpectra) -> f32 {
        let mut y =0.0;
        for i in 0..SPECTRAL_SAMPLES {
            y += constant_spectra.y.coefficients[i] * self.coefficients[i];
        }
        let scale = (LAST_WAVELENGTH - FIRST_WAVELENGTH as f32) / (CIE_Y_INTEGRAL * SPECTRAL_SAMPLES as f32);
        y *= scale;
        y
    }

    /// Converts from XYZ values to RGB values, based on a standard set of RGB spectra
    /// that has been defined for high-definition television.
    pub fn xyz_to_rgb(xyz: [f32; 3]) -> [f32; 3] {
        let mut rgb = [0.0; 3];
        rgb[0] =  3.240479f32 * xyz[0] - 1.537150f32 * xyz[1] - 0.498535f32 * xyz[2];
        rgb[1] = -0.969256f32 * xyz[0] + 1.875991f32 * xyz[1] + 0.041556f32 * xyz[2];
        rgb[2] =  0.055648f32 * xyz[0] - 0.204043f32 * xyz[1] + 1.057311f32 * xyz[2];
        rgb
    }

    /// Converts from RGB values to XYZ values, based on a standard set of RGB spectra
    /// that has been defined for high-definition television.
    pub fn rgb_to_xyz(rgb: [f32; 3]) -> [f32; 3] {
        let mut xyz = [0.0; 3];
        xyz[0] = 0.412453f32 * rgb[0] + 0.357580f32 * rgb[1] + 0.180423f32 * rgb[2];
        xyz[1] = 0.212671f32 * rgb[0] + 0.715160f32 * rgb[1] + 0.072169f32 * rgb[2];
        xyz[2] = 0.019334f32 * rgb[0] + 0.119193f32 * rgb[1] + 0.950227f32 * rgb[2];
        xyz
    }

    /// Gets the RGB coefficients for the SPD.
    pub fn get_rgb(&self, constant_spectra: &ConstantSpectra) -> [f32; 3] {
        Spectrum::xyz_to_rgb(self.get_xyz(constant_spectra))
    }

    
    pub fn clamp(&mut self, min: f32, max: f32) {
        self.coefficients = self.coefficients.map(|a| bound_f32(a, min, max));
    }

    pub fn is_black(&mut self) -> bool {
        !self.coefficients.iter().any(|x| *x != 0.0f32)
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
