use crate::camera::{Camera, CameraSettings};
use crate::film::Film;
use crate::filter::{BoxFilter, Filter};
use crate::image::Color;
use crate::light::*;
use crate::material::lambertian::Lambertian;
use crate::nalgebra::{Point3, Vector3};
use crate::primitives::rect::*;
use crate::primitives::sphere::Sphere;
use crate::primitives::{GeometricPrimitive, GeometricPrimitives, Primitive, Primitives};
use crate::raytracing::{Hit, HitRecord, Ray};
use crate::spectrum::SpectrumType;
use crate::spectrum::{constant_spectra::ConstantSpectra, spectrum_factory::SpectrumFactory, Spectrum};
use crate::util::*;
use crate::{material::*, sampler};

#[derive(Clone)]

/// Contains all information regarding the scene. The raytracing_primitives and
/// the rasterization_primitives contain the same primtitives, but
/// raytracing_primitives may contain acceleration structures designed to
/// improve raytracing performance. The background color is the ambient color of
/// the scene.
pub struct Scene {
    pub raytracing_primitives: Primitives,
    pub lights: Vec<Light>,
    pub rasterization_primitives: GeometricPrimitives,
    pub camera: Camera,
}

impl Scene {
    pub fn new(
        raytracing_primitives: Primitives,
        lights: Vec<Light>,
        rasterization_primitives: GeometricPrimitives,
        camera: Camera,
    ) -> Scene {
        Scene {
            raytracing_primitives,
            lights,
            rasterization_primitives,
            camera,
        }
    }
    pub fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.raytracing_primitives.hit(&r, t_min, t_max)
    }
}

pub fn point_light_test(image_width: usize, aspect_ratio: f32) -> Scene {
    // Lights
    let lights = Vec::<Light>::new();
    let raytracing_primitives = Primitives::new();
    let rasterization_primitives = GeometricPrimitives::new();
    let spectrum_factory = SpectrumFactory::new();
    let light_spectrum = spectrum_factory.from_rgb(Color::new(1.0, 1.0, 1.0), SpectrumType::Illuminant);
    let light_position = Point3::<f32>::new(10.0, 10.0, 10.0);

    // Primitives
    let sphere_center = Point3::<f32>::new(12.0, 1.0, 0.0);
    let sphere_radius = 1.0;
    let sphere_color = spectrum_factory.from_rgb(Color::new(1.0, 0.0, 0.0), SpectrumType::Reflectance);
    let sphere_material = Material::new_lambertian(sphere_color);
    let sphere = GeometricPrimitive::Sphere(Sphere::new(sphere_center, sphere_radius, sphere_material));

    //Camera
    let image_height = ((image_width as f32) / aspect_ratio) as usize;
    let look_from = Point3::<f32>::new(0.0, 0.0, 0.0);
    let look_at = sphere_center;
    let v_up: Vector3<f32> = Vector3::<f32>::new(0.0, 1.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.0;

    let filter = Filter::from(BoxFilter::new((1.0, 1.0)));
    let film = Film::new(filter, (image_width, image_height));
    let camera_settings = CameraSettings {
        look_from,
        look_at,
        v_up,
        v_fov: 20.0,
        aspect_ratio,
        aperture,
        focus_dist,
        film,
    };
    let camera = Camera::new(camera_settings);

    let image_height = ((image_width as f32) / aspect_ratio) as usize;
    lights.push(Light::PointLight(PointLight::new(light_spectrum, light_position)));
    rasterization_primitives.add(sphere);
    raytracing_primitives.add(Primitive::Bvh(rasterization_primitives.clone().to_bvh()));
    return Scene::new(raytracing_primitives, lights, rasterization_primitives, camera);
}
