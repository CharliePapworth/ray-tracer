#[rustfmt::skip]
use crate::{
    camera::{Camera, CameraSettings, Film, Color},
    filter::{BoxFilter, Filter},
    primitives::{Primitive, Primitives, Sphere},
    nalgebra::{Point3, Vector3, Matrix4},
    raytracing::{Hit, HitRecord, Ray},
    light::{Light, SpectrumFactory, SpectrumType, PointLight}, 
    material::{Material},
};



/// Contains all information regarding the scene. The raytracing_primitives and
/// the rasterization_primitives contain the same primtitives, but
/// raytracing_primitives may contain acceleration structures designed to
/// improve raytracing performance. The background color is the ambient color of
/// the scene.
#[derive(Clone)]
pub struct Scene<'a> {
    pub raytracing_primitives: Primitives<'a>,
    pub lights: Vec<Light>,
    pub camera: Camera,
}

impl<'a> Scene<'a> {
    pub fn new(
        raytracing_primitives: Primitives,
        lights: Vec<Light>,
        camera: Camera,
    ) -> Scene {
        Scene {
            raytracing_primitives,
            lights,
            camera,
        }
    }
    pub fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.raytracing_primitives.hit(&r, t_min, t_max)
    }
}

pub fn point_light_test<'a>(image_width: usize, aspect_ratio: f32) -> Scene<'a> {
    // Lights
    let lights = Vec::<Light>::new();
    let raytracing_primitives = Primitives::new();
    let rasterization_primitives = Primitives::new();
    let spectrum_factory = SpectrumFactory::new();
    let light_spectrum = spectrum_factory.from_rgb(Color::new(1.0, 1.0, 1.0), SpectrumType::Illuminant);
    let light_position = Point3::<f32>::new(10.0, 10.0, 10.0);

    // Primitives
    let sphere_center = Point3::<f32>::new(12.0, 1.0, 0.0);
    let sphere_radius = 1.0;
    let sphere_color = spectrum_factory.from_rgb(Color::new(1.0, 0.0, 0.0), SpectrumType::Reflectance);
    let sphere_material = Material::new_lambertian(sphere_color);
    let sphere = Primitive::Sphere(Sphere::new(sphere_center, sphere_radius, sphere_material));

    //Camera
    let image_height = ((image_width as f32) / aspect_ratio) as usize;
    let look_from = Point3::<f32>::new(0.0, 0.0, 0.0);
    let look_at = sphere_center;
    let v_up: Vector3<f32> = Vector3::<f32>::new(0.0, 1.0, 0.0);
    let camera_to_world = Matrix4::<f32>::new_observer_frame(&look_from, &look_at, &v_up);
    let focus_dist = 10.0;
    let aperture = 0.0;

    let filter = Filter::from(BoxFilter::new((1.0, 1.0)));
    let film = Film::new(filter, (image_width, image_height));
    let camera = Camera::new(camera_to_world, film, focus_dist);

    let image_height = ((image_width as f32) / aspect_ratio) as usize;
    lights.push(Light::PointLight(PointLight::new(light_spectrum, light_position)));
    raytracing_primitives.add(Primitive::Bvh(rasterization_primitives.to_bvh()));
    return Scene::new(raytracing_primitives, lights, camera);
}
