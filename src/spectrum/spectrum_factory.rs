pub struct SpectrumFactory {
    pub constant_spectra: ConstantSpectra
}

impl SpectrumFactory {
    pub fn new() -> SpectrumFactory {
        SpectrumFactory { constant_spectra: ConstantSpectra::new() }
    }

    pub fn new_spectrum(&self, constant: f32) -> Spectrum {
        Spectrum::new(constant)
    }

    /// Converts from the given RGB values to a full SPD. In addition to the RGB values, it takes an enumeration value that
    ///  denotes whether the RGB value represents surface reflectance or an illuminant; the corresponding rgb_illum_to_spect_ 
    /// or rgb_refl_to_spect_ values are used for the conversion. 
    /// 
    /// This approach is based on the observation that a good start is to compute individual SPDs for red, green, and blue that
    ///  are smooth and such that computing the weighted sum of them with the given RGB coefficients and then converting back
    ///  to RGB give a result that is close to the original RGB coefficients. These spectra were found through a numerical 
    /// optimization procedure. 
    pub fn from_rgb(rgb: Color, spectrum_type: SpectrumType, constant_spectra: &ConstantSpectra) -> Spectrum{
        match spectrum_type {
            SpectrumType::Reflectance => {
                return SpectrumFactory::from_rgb_reflectance(rgb, constant_spectra);
            }
            SpectrumType::Illuminant => {
                return SpectrumFactory::from_rgb_illuminant(rgb, constant_spectra);
            }
        }
    }

    fn from_rgb_reflectance(rgb: Color, constant_spectra: &ConstantSpectra) -> Spectrum {
        let mut output = Spectrum::new(0.0);
        let rgb = [rgb[0] as f32, rgb[1] as f32, rgb[2] as f32];
        if rgb[0] <= rgb[1] && rgb[0] <= rgb[2] {
            //Compute reflectance SampledSpectrum with rgb[0] as minimum
            output.coefficients += rgb[0] * &constant_spectra.rgb_refl_to_spect_white.coefficients;
            if rgb[1] <= rgb[2] {
            output.coefficients += (rgb[1] - rgb[0]) * &constant_spectra.rgb_refl_to_spect_cyan.coefficients;
            output.coefficients += (rgb[2] - rgb[1]) * &constant_spectra.rgb_refl_to_spect_blue.coefficients;
            } else {
            output.coefficients += (rgb[2] - rgb[0]) * &constant_spectra.rgb_refl_to_spect_cyan.coefficients;
            output.coefficients += (rgb[1] - rgb[2]) * &constant_spectra.rgb_refl_to_spect_green.coefficients;
            }
        } else if rgb[1] <= rgb[0] && rgb[1] <= rgb[2] { 
            output.coefficients += rgb[1] * &constant_spectra.rgb_refl_to_spect_white.coefficients;
            if rgb[0] <= rgb[2] {
                output.coefficients += (rgb[0] - rgb[1]) * &constant_spectra.rgb_refl_to_spect_magenta.coefficients;
                output.coefficients += (rgb[2] - rgb[0]) * &constant_spectra.rgb_refl_to_spect_blue.coefficients;
            } else {
                output.coefficients += (rgb[2] - rgb[1]) * &constant_spectra.rgb_refl_to_spect_magenta.coefficients;
                output.coefficients += (rgb[0] - rgb[2]) * &constant_spectra.rgb_refl_to_spect_red.coefficients;
            }
        } else {
            output.coefficients += rgb[2] * &constant_spectra.rgb_refl_to_spect_white.coefficients;
            if rgb[0] <= rgb[1] {
            output.coefficients += (rgb[0] - rgb[2]) * &constant_spectra.rgb_refl_to_spect_yellow.coefficients;
            output.coefficients += (rgb[1] - rgb[0]) * &constant_spectra.rgb_refl_to_spect_green.coefficients;
            } else {
            output.coefficients += (rgb[1] - rgb[2]) * &constant_spectra.rgb_refl_to_spect_yellow.coefficients;
                output.coefficients += (rgb[0] - rgb[1]) * &constant_spectra.rgb_refl_to_spect_red.coefficients;
            }
        }
        output.clamp(0.0, f32::INFINITY);
        output
    }


    fn from_rgb_illuminant(rgb: Color, constant_spectra: &ConstantSpectra) -> Spectrum {
        let mut output = Spectrum::new(0.0);
        let rgb = [rgb[0] as f32, rgb[1] as f32, rgb[2] as f32];
        if rgb[0] <= rgb[1] && rgb[0] <= rgb[2] {
            //Compute reflectance SampledSpectrum with rgb[0] as minimum
            output.coefficients += rgb[0] * &constant_spectra.rgb_illum_to_spect_white.coefficients;
            if rgb[1] <= rgb[2] {
            output.coefficients += (rgb[1] - rgb[0]) * &constant_spectra.rgb_illum_to_spect_cyan.coefficients;
            output.coefficients += (rgb[2] - rgb[1]) * &constant_spectra.rgb_illum_to_spect_blue.coefficients;
            } else {
            output.coefficients += (rgb[2] - rgb[0]) * &constant_spectra.rgb_illum_to_spect_cyan.coefficients;
            output.coefficients += (rgb[1] - rgb[2]) * &constant_spectra.rgb_illum_to_spect_green.coefficients;
            }
        } else if rgb[1] <= rgb[0] && rgb[1] <= rgb[2] { 
            output.coefficients += rgb[1] * &constant_spectra.rgb_illum_to_spect_white.coefficients;
            if rgb[0] <= rgb[2] {
                output.coefficients += (rgb[0] - rgb[1]) * &constant_spectra.rgb_illum_to_spect_magenta.coefficients;
                output.coefficients += (rgb[2] - rgb[0]) * &constant_spectra.rgb_illum_to_spect_blue.coefficients;
            } else {
                output.coefficients += (rgb[2] - rgb[1]) * &constant_spectra.rgb_illum_to_spect_magenta.coefficients;
                output.coefficients += (rgb[0] - rgb[2]) * &constant_spectra.rgb_illum_to_spect_red.coefficients;
            }
        } else {
            output.coefficients += rgb[2] * &constant_spectra.rgb_illum_to_spect_white.coefficients;
            if rgb[0] <= rgb[1] {
            output.coefficients += (rgb[0] - rgb[2]) * &constant_spectra.rgb_illum_to_spect_yellow.coefficients;
            output.coefficients += (rgb[1] - rgb[0]) * &constant_spectra.rgb_illum_to_spect_green.coefficients;
            } else {
            output.coefficients += (rgb[1] - rgb[2]) * &constant_spectra.rgb_illum_to_spect_yellow.coefficients;
                output.coefficients += (rgb[0] - rgb[1]) * &constant_spectra.rgb_illum_to_spect_red.coefficients;
            }
        }
        output.clamp(0.0, f32::INFINITY);
        output
    }
}

