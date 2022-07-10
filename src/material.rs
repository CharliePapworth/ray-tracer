use nalgebra::{Vector3, Unit};

use crate::image::{Color};
use crate::spectra::SampledSpectrum;
use crate::vec::VecExtensionMethods;
use crate::{util::*, sampler};
use crate::raytracing::{HitRecord, Ray};
use crate::sampler::*;

pub enum ReflectionModel {
    Diffuse,
    Glossy,
    Specular
}

pub trait Material<'a> {

    ///Returns the reflection model of the material.
    fn reflection_model(&self) -> ReflectionModel;

    ///True if the material transmits light, and false otherwise.
    fn transmits(&self) -> bool;

    ///True if the material reflects light, and false otherwise.
    fn reflects(&self) -> bool;

    ///Returns the radiance (given by a spectrum) reflected from an incident ray of light along a given direction.
    fn scatter(outbound_direction: Vector3<f64>, inbound_direction: Vector3<f64>) -> SampledSpectrum<'a>;

    ///Returns the radiance (given by a spectrum) reflected from an incident ray of light along a given direction. The direction
    /// of the incident ray of light is chosen by the Material. This is useful in cases where the probability of choosing a
    /// direction from which light will be reflected in the desired outbound direction is low (e.g. as would be the case for a mirror).
    fn opinionated_scatter(outbound_direction: Vector3<f64>) -> (SampledSpectrum<'a>, Vector3<f64>);

    ///The hemispherical-directional reflectance is a 2D function that gives the total reflection in a given direction due to constant
    ///  illumination over the hemisphere, or, equivalently, total reflection over the hemisphere due to light from a given direction.
    fn hemispherical_directional_scatter(direction: Vector3<f64>) -> SampledSpectrum<'a>;

    ///The hemispherical-hemispherical reflectance of a surface, denoted by , is a spectral value that gives the fraction of incident 
    /// light reflected by a surface when the incident light is the same from all directions. 
    fn hemispherical_hemispherical_scatter() -> SampledSpectrum<'a>;
}


#[derive(Default, Clone, Copy, PartialEq)]
pub struct Lambertian{
    pub albedo: Color
}

#[derive(Default, Clone, Copy, PartialEq)]
pub struct Metal{
    albedo: Color,
    fuzz: f64
}


#[derive(Default, Clone, Copy, PartialEq)]
pub struct Dielectric{
    index_of_refraction :f64,
}

#[derive(Default, Clone, Copy, PartialEq)]
pub struct DiffuseLights{
    color: Color
}


#[derive(Clone, Copy, PartialEq)]
pub enum Material{
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    DiffuseLights(DiffuseLights)
}

impl Scatter for Material {
    fn scatter(&self, r : &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        match *self {
            Material::Lambertian(material) => material.scatter(r, rec),
            Material::Metal(material) => material.scatter(r, rec),
            Material::Dielectric(material) => material.scatter(r, rec),
            Material::DiffuseLights(material) => material.scatter(r, rec)
        }
    }

    fn emit(&self) -> Color {
        match *self {
            Material::Lambertian(material) => material.emit(),
            Material::Metal(material) => material.emit(),
            Material::Dielectric(material) => material.emit(),
            Material::DiffuseLights(material) => material.emit()
        }
    }
}

impl Material {
    pub fn new_lambertian(alb: Color) -> Material {
        Material::Lambertian(Lambertian::new(alb))
    }

    pub fn new_metal(alb: Vector3<f64>, fuzz: f64) -> Material {
        Material::Metal(Metal::new(alb, fuzz))
    }

    pub fn new_dielectric(ir: f64) -> Material{
        Material::Dielectric(Dielectric::new(ir))
    }

    pub fn new_diffuse_light(color: Color) -> Material{
        Material::DiffuseLights(DiffuseLights::new(color))
    }
}

impl Lambertian{
    pub fn new(alb: Color) -> Lambertian {
        Lambertian{albedo: alb}
    }

    fn deterministic_scatter(&self, rec: &HitRecord, rand_unit_vec: Vector3<f64>) -> Option<(Color, Ray)>{
        let mut scatter_direction = rec.normal + rand_unit_vec;

        // Catch degenerate Scatter direction
        if scatter_direction.near_zero(){
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>{

        let reflect_dir = sampler::rand_unit_vec();
        self.deterministic_scatter( rec, reflect_dir.into_inner())

    }
}

impl Metal {
    pub fn new(alb: Vector3<f64>, mut fuzz: f64) -> Metal {
        if fuzz > 1.0 {fuzz = 1.0}
        else if fuzz < 0.0 {fuzz = 0.0}
        Metal{albedo: alb, fuzz}
    }

    fn deterministic_scatter(&self, r_in: &Ray, rec: &HitRecord, rand_in_unit_sphere: Vector3<f64>) -> Option<(Color, Ray)>{
        let reflected = Unit::new_normalize(r_in.direction()).reflect(&rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz*rand_in_unit_sphere);
        let attenuation = self.albedo;
        if scattered.direction().dot(&rec.normal) > 0.0{
            Some((attenuation, scattered))
        }else{
            None
        }
    }
}

impl Scatter for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>{
        let fuzz_dir = sampler::rand_in_unit_sphere();
        self.deterministic_scatter(r_in, rec, fuzz_dir)
    }
}

impl Dielectric {
    pub fn new(ir: f64) -> Dielectric{
        Dielectric{index_of_refraction: ir}
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64{
        //Use Schlick's approximation for reflectance
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }

    fn deterministic_scatter(&self, r_in: &Ray, rec: &HitRecord, reflectance_test: f64) -> Option<(Color, Ray)>{
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let mut refraction_ratio = self.index_of_refraction;
        if rec.front_face{
            refraction_ratio = 1.0/self.index_of_refraction;
        }
        
        let unit_dir = r_in.direction().normalize();
        let cos_theta = - unit_dir.dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract = refraction_ratio*sin_theta > 1.0;
        let direction:Vector3<f64>;

        if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > reflectance_test{
            direction = Vector3::<f64>::reflect(&unit_dir, &rec.normal);
        } else{
            direction = Vector3::<f64>::refract(&unit_dir, &rec.normal, refraction_ratio);
        }
        let scattered = Ray::new(rec.p, direction);
        Some((attenuation, scattered))
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>{
        let rand = sampler::rand_double(0.0, 1.0);
        self.deterministic_scatter(r_in, rec, rand)
    }
}

impl DiffuseLights{
    pub fn new(color: Color) -> DiffuseLights{
        DiffuseLights{color}
    }
}

impl Scatter for DiffuseLights{
    fn scatter(&self, _: &Ray, _: &HitRecord) -> Option<(Color, Ray)>{
        None
    }

    fn emit(&self) -> Color{
        self.color
    }

}

pub trait Scatter: Clone{
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
    fn emit(&self) -> Color{
        Color::new(0.0, 0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::{GeometricPrimitive};
    use crate::raytracing::{Hit, Ray};
    use crate::nalgebra::{Point3};

    #[test]
    fn test_lambertian_deterministic_scatter(){

        //Initialisation
        let albedo = Color::new(0.7, 0.6, 0.5);
        let mat = Material::new_lambertian(albedo);
        let s = GeometricPrimitive::new_sphere(Point3::new(1.0,0.0,0.0), 1.0, mat);
        let r = Ray::new(Point3::new(-10.0, -10.0, 0.0), Vector3::<f64>::new( 1.0, 1.0, 0.0));
        let hit = s.hit(&r, 0.0, 100.0);
        let (rec, _) = hit.unwrap();
        
        //Case 1: Scatter direction is non-degenerate
        let mat = Lambertian::new(albedo);
        let scatter_result = mat.deterministic_scatter(&rec, Vector3::<f64>::new(-1.0, 0.0, 0.0));
        assert!(scatter_result.is_some());
        let (color, reflected_ray) = scatter_result.unwrap();
        assert_eq!(color, Color::new(0.7, 0.6, 0.5));
        assert_eq!(reflected_ray, Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::<f64>::new( -2.0, 0.0, 0.0)));
        

        //Case 2: Scatter direction is degenerate
        let scatter_result = mat.deterministic_scatter(&rec, Vector3::<f64>::new(1.0, 0.0, 0.0));
        assert!(scatter_result.is_some());
        let (color, reflected_ray) = scatter_result.unwrap();
        assert_eq!(color, Color::new(0.7, 0.6, 0.5));
        assert_eq!(reflected_ray, Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::<f64>::new( -1.0, 0.0, 0.0)));
    }

    #[test]
    fn test_lambertian_emit(){
        let albedo = Color::new(0.7, 0.6, 0.5);
        let mat = Lambertian::new(albedo);
        let emission= mat.emit();
        assert_eq!(emission, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_metal_deterministic_scatter(){

        //Initialisation
        let albedo = Color::new(0.7, 0.6, 0.5);
        let mat = Material::new_metal(albedo, 20.0);
        let s = GeometricPrimitive::new_sphere(Point3::new(1.0,0.0,0.0), 1.0, mat);
        let r = Ray::new(Point3::new(-10.0, -10.0, 0.0), Vector3::<f64>::new( 1.0, 1.0, 0.0));
        let hit = s.hit(&r, 0.0, 100.0);
        let (rec, _) = hit.unwrap();
        
        //Case 1: Ray reflects
        let mat = Metal::new(albedo, 20.0);
        let scatter_result = mat.deterministic_scatter(&r, &rec, Vector3::<f64>::new(0.0, 0.0, 0.0));
        assert!(scatter_result.is_some());
        let (color, reflected_ray) = scatter_result.unwrap();
        assert_eq!(color, Color::new(0.7, 0.6, 0.5));
        assert_eq!(reflected_ray, Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::<f64>::new( -1.0, 1.0, 0.0).normalize()));
        

        //Case 2: Ray is absorbed
        let scatter_result = mat.deterministic_scatter(&r, &rec, Vector3::<f64>::new(1.0, 0.0, 0.0));
        assert!(scatter_result.is_none());
    }

    #[test]
    fn test_metal_emit(){
        let albedo = Color::new(0.7, 0.6, 0.5);
        let mat = Metal::new(albedo, 20.0);
        let emission= mat.emit();
        assert_eq!(emission, Color::new(0.0, 0.0, 0.0));
    }


    #[test]
    fn test_diffuse_light_scatter(){
        let mat = Material::new_diffuse_light(Color::new(0.7, 0.6, 0.5));
        let s = GeometricPrimitive::new_sphere(Point3::new(1.0,0.0,0.0), 1.0, mat);
        let r = Ray::new(Point3::new(-10.0, -10.0, 0.0), Vector3::<f64>::new( 1.0, 1.0, 0.0));
        let hit = s.hit(&r, 0.0, 100.0);
        let (rec, _) = hit.unwrap();

        let scatter_result = mat.scatter(&r, &rec);
        assert!(scatter_result.is_none());
    }

    #[test]
    fn test_diffuse_light_emit(){
        let mat = DiffuseLights::new(Color::new(0.7, 0.6, 0.5));
        let emission= mat.emit();
        assert_eq!(emission, Color::new(0.7, 0.6, 0.5));
    }

    #[test]
    fn test_reflectance(){
        let unit_vec = Vector3::<f64>::new(1.0, 2.0, 3.0).normalize();
        let normal = Vector3::<f64>::new(2.0, -1.0, 1.0);
        let cos_theta = unit_vec.dot(&normal).min(1.0);
        let refraction_ratio = 2.0;
        assert_eq!(Dielectric::reflectance(cos_theta,refraction_ratio), 1.0/9.0 + (8.0/9.0)*((1.0-cos_theta).powi(5)));
    }
}