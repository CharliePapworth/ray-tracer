use crate::util::lerp;
use crate::vec::Color;

use super::coefficients::Coefficients;
use super::constants::*;
use std::iter::zip;

const FIRST_WAVELENGTH: f32 = 400.0;
const LAST_WAVELENGTH: f32 = 700.0;
const SPECTRAL_SAMPLES: usize = 60;


pub enum SpectrumType {
    Reflectance,
    Illuminant
}

pub struct ConstantSpectrum {
    pub coefficients: Coefficients<SPECTRAL_SAMPLES>
}


/// These are referenced by each instance of SampledSpectrum,
/// and must therefore have a longer lifetime than every instance. Initialising them in main() is therefore recommended.
pub struct ConstantSpectra {
    x: ConstantSpectrum,
    y: ConstantSpectrum,
    z: ConstantSpectrum,
    rgb_refl_to_spect_white: ConstantSpectrum,
    rgb_refl_to_spect_cyan: ConstantSpectrum,
    rgb_refl_to_spect_magenta: ConstantSpectrum,
    rgb_refl_to_spect_yellow: ConstantSpectrum,
    rgb_refl_to_spect_red: ConstantSpectrum,
    rgb_refl_to_spect_green: ConstantSpectrum,
    rgb_refl_to_spect_blue: ConstantSpectrum,
    rgb_illum_to_spect_white: ConstantSpectrum,
    rgb_illum_to_spect_cyan: ConstantSpectrum,
    rgb_illum_to_spect_magenta: ConstantSpectrum,
    rgb_illum_to_spect_yellow: ConstantSpectrum,
    rgb_illum_to_spect_red: ConstantSpectrum,
    rgb_illum_to_spect_green: ConstantSpectrum,
    rgb_illum_to_spect_blue: ConstantSpectrum,
}

impl ConstantSpectra {

    pub fn init() -> ConstantSpectra {
        let spectral_samples_f32 = SPECTRAL_SAMPLES as f32;
        let mut x = ConstantSpectrum::new(0.0);
        let mut y = ConstantSpectrum::new(0.0);
        let mut z = ConstantSpectrum::new(0.0);
        let mut rgb_refl_to_spect_white = ConstantSpectrum::new(0.0);
        let mut rgb_refl_to_spect_cyan = ConstantSpectrum::new(0.0);
        let mut rgb_refl_to_spect_magenta = ConstantSpectrum::new(0.0);
        let mut rgb_refl_to_spect_yellow = ConstantSpectrum::new(0.0);
        let mut rgb_refl_to_spect_red = ConstantSpectrum::new(0.0);
        let mut rgb_refl_to_spect_green = ConstantSpectrum::new(0.0);
        let mut rgb_refl_to_spect_blue = ConstantSpectrum::new(0.0);
        let mut rgb_illum_to_spect_white = ConstantSpectrum::new(0.0);
        let mut rgb_illum_to_spect_cyan = ConstantSpectrum::new(0.0);
        let mut rgb_illum_to_spect_magenta = ConstantSpectrum::new(0.0);
        let mut rgb_illum_to_spect_yellow = ConstantSpectrum::new(0.0);
        let mut rgb_illum_to_spect_red = ConstantSpectrum::new(0.0);
        let mut rgb_illum_to_spect_green = ConstantSpectrum::new(0.0);
        let mut rgb_illum_to_spect_blue = ConstantSpectrum::new(0.0);

        for i in 0..N_SPECTRAL_SAMPLES {
            let i_f32 = i as f32;
            let next_i_f32 = (i + 1) as f32;
            let from_wavelength = lerp(FIRST_WAVELENGTH, LAST_WAVELENGTH, i_f32 / spectral_samples_f32);
            let to_wavelength =  lerp(FIRST_WAVELENGTH, LAST_WAVELENGTH, next_i_f32 / spectral_samples_f32);

            x.coefficients[i] = average_samples(&CIE_X.to_vec(), &CIE_LAMBDA.to_vec(), from_wavelength, to_wavelength);
            y.coefficients[i] = average_samples(&CIE_Y.to_vec(), &CIE_LAMBDA.to_vec(), from_wavelength, to_wavelength);
            z.coefficients[i] = average_samples(&CIE_Z.to_vec(), &CIE_LAMBDA.to_vec(), from_wavelength, to_wavelength);

            rgb_refl_to_spect_white.coefficients[i] = average_samples(&RGB_2_SPECT_LAMBDA.to_vec(), &RGB_REFL2_SPECT_WHITE.to_vec(), from_wavelength, to_wavelength);
            rgb_refl_to_spect_cyan.coefficients[i] = average_samples(&RGB_2_SPECT_LAMBDA.to_vec(), &RGB_REFL2_SPECT_CYAN.to_vec(), from_wavelength, to_wavelength);
            rgb_refl_to_spect_magenta.coefficients[i] = average_samples(&RGB_2_SPECT_LAMBDA.to_vec(), &RGB_REFL2_SPECT_MAGENTA.to_vec(), from_wavelength, to_wavelength);
            rgb_refl_to_spect_yellow.coefficients[i] = average_samples(&RGB_2_SPECT_LAMBDA.to_vec(), &RGB_REFL2_SPECT_YELLOW.to_vec(), from_wavelength, to_wavelength);
            rgb_refl_to_spect_red.coefficients[i] = average_samples(&RGB_2_SPECT_LAMBDA.to_vec(), &RGB_REFL2_SPECT_RED.to_vec(), from_wavelength, to_wavelength);
            rgb_refl_to_spect_green.coefficients[i] = average_samples(&RGB_2_SPECT_LAMBDA.to_vec(), &RGB_REFL2_SPECT_GREEN.to_vec(), from_wavelength, to_wavelength);
            rgb_refl_to_spect_blue.coefficients[i] = average_samples(&RGB_2_SPECT_LAMBDA.to_vec(), &RGB_REFL2_SPECT_BLUE.to_vec(), from_wavelength, to_wavelength);
        
            rgb_illum_to_spect_white.coefficients[i] = average_samples(&RGB_2_SPECT_LAMBDA.to_vec(), &RGB_ILLUM2_SPECT_WHITE.to_vec(), from_wavelength, to_wavelength);
            rgb_illum_to_spect_cyan.coefficients[i] = average_samples(&RGB_2_SPECT_LAMBDA.to_vec(), &RGB_ILLUM2_SPECT_CYAN.to_vec(), from_wavelength, to_wavelength);
            rgb_illum_to_spect_magenta.coefficients[i] = average_samples(&RGB_2_SPECT_LAMBDA.to_vec(), &RGB_ILLUM2_SPECT_MAGENTA.to_vec(), from_wavelength, to_wavelength);
            rgb_illum_to_spect_yellow.coefficients[i] = average_samples(&RGB_2_SPECT_LAMBDA.to_vec(), &RGB_ILLUM2_SPECT_YELLOW.to_vec(), from_wavelength, to_wavelength);
            rgb_illum_to_spect_red.coefficients[i] = average_samples(&RGB_2_SPECT_LAMBDA.to_vec(), &RGB_ILLUM2_SPECT_RED.to_vec(), from_wavelength, to_wavelength);
            rgb_illum_to_spect_green.coefficients[i] = average_samples(&RGB_2_SPECT_LAMBDA.to_vec(), &RGB_ILLUM2_SPECT_GREEN.to_vec(), from_wavelength, to_wavelength);
            rgb_illum_to_spect_blue.coefficients[i] = average_samples(&RGB_2_SPECT_LAMBDA.to_vec(), &RGB_ILLUM2_SPECT_BLUE.to_vec(), from_wavelength, to_wavelength);
        }    

        ConstantSpectra { x, y, z, rgb_refl_to_spect_white, rgb_refl_to_spect_cyan, rgb_refl_to_spect_magenta, rgb_refl_to_spect_yellow, rgb_refl_to_spect_red, rgb_refl_to_spect_green, rgb_refl_to_spect_blue, rgb_illum_to_spect_white, rgb_illum_to_spect_cyan, rgb_illum_to_spect_magenta, rgb_illum_to_spect_yellow, rgb_illum_to_spect_red, rgb_illum_to_spect_green, rgb_illum_to_spect_blue }
    }
}



impl ConstantSpectrum {
    /// Initialises a spectrum with coefficients of a constant value. Not for public consumption.
    fn new(constant: f32) -> ConstantSpectrum {
        let coefficients = Coefficients::<SPECTRAL_SAMPLES>::new(constant);
        ConstantSpectrum { coefficients } 
    }
}

/// SampledSpectrum<'a> uses the Coefficients infrastructure to represent an SPD with uniformly spaced samples
/// between a starting and an ending wavelength. The wavelength range covers from 400 nm to 700 nm—the range of the
///  visual spectrum where the human visual system is most sensitive. 
pub struct SampledSpectrum<'a> {
    pub coefficients: Coefficients<SPECTRAL_SAMPLES>,
    pub constant_spectra: &'a ConstantSpectra
}

impl<'a> SampledSpectrum<'a> {

    /// Initialises a SampledSpectrum with coefficients of a constant value.
    ///     
    /// A reference to the matching curves, calculated by MatchingCurve::init_xyz, must be passed into the constructor.
    pub fn new(constant: f32, constant_spectra: &'a ConstantSpectra) -> SampledSpectrum<'a> {
        let coefficients = Coefficients::<SPECTRAL_SAMPLES>::new(constant);
        SampledSpectrum { coefficients, constant_spectra } 
    }
    
    /// Takes arrays of SPD sample values at given wavelengths lambda and uses them to 
    /// define a piecewise linear function to represent the SPD.
    /// 
    /// A reference to the matching curves, calculated by MatchingCurve::init_xyz, must be passed into the constructor.
    pub fn from_sampled(sample_values: Vec<f32>, sample_wavelengths: Vec<f32>, constant_spectra: &'a ConstantSpectra) -> SampledSpectrum<'a> {
        let mut spectrum = SampledSpectrum::<'a>::new(0.0, constant_spectra);
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

    pub fn from_coefficients(coefficients: Coefficients<SPECTRAL_SAMPLES>, constant_spectra: &'a ConstantSpectra) -> SampledSpectrum<'a> {
        SampledSpectrum { coefficients, constant_spectra }
    }

    /// Computes X, Y & Z coefficients. This is calculated by integrating the product of the matching curves
    /// with the sampled spectrum.
    pub fn get_xyz(&self) -> [f32; 3] {
        let mut xyz = [0.0; 3];
        for i in 0..SPECTRAL_SAMPLES {
            xyz[0] += self.constant_spectra.x.coefficients[i] * self.coefficients[i];
            xyz[1] += self.constant_spectra.y.coefficients[i] * self.coefficients[i];
            xyz[2] += self.constant_spectra.z.coefficients[i] * self.coefficients[i];
        }
        let scale = (LAST_WAVELENGTH - FIRST_WAVELENGTH as f32) / (CIE_Y_INTEGRAL * SPECTRAL_SAMPLES as f32);
        xyz[0] *= scale;
        xyz[1] *= scale;
        xyz[2] *= scale;
        xyz        
    }

    /// Converts from the given RGB values to a full SPD. In addition to the RGB values, it takes an enumeration value that
    ///  denotes whether the RGB value represents surface reflectance or an illuminant; the corresponding rgb_illum_to_spect_ 
    /// or rgb_refl_to_spect_ values are used for the conversion. 
    /// 
    /// This approach is based on the observation that a good start is to compute individual SPDs for red, green, and blue that
    ///  are smooth and such that computing the weighted sum of them with the given RGB coefficients and then converting back
    ///  to RGB give a result that is close to the original RGB coefficients. These spectra were found through a numerical 
    /// optimization procedure. 
    pub fn from_rgb(rgb: Color, spectrum_type: SpectrumType) -> SampledSpectrum<'a>{
        match spectrum_type {
            SpectrumType::Reflectance => {

            }
            SpectrumType::Illuminant => {

            }
        }

        todo!()
    }


    ///The Y coefficient of XYZ color is closely related to luminance, which measures the perceived brightness of a color. 
    /// This method calculates the Y coefficient alone.
    pub fn get_y(&self) -> f32 {
        let mut y =0.0;
        for i in 0..SPECTRAL_SAMPLES {
            y += self.constant_spectra.y.coefficients[i] * self.coefficients[i];
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
    pub fn get_rgb(&self) -> [f32; 3] {
        SampledSpectrum::xyz_to_rgb(self.get_xyz())
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
