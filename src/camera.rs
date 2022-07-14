use core::f32;

use nalgebra::{Translation3, Unit, Transform3};

use crate::sampler;
use crate::util::deg_to_rad;
use crate::nalgebra::{Vector3, Point3, Rotation3};
use crate::raytracing::Ray;
#[derive (Copy, Clone)]
pub struct Camera {

    // These settings affect the camera output
    pub origin: Point3<f32>,
    pub horizontal: Vector3<f32>,
    pub vertical: Vector3<f32>,
    pub lower_left_corner: Point3<f32>,
    pub orientation: Orientation,
    pub lens_radius: f32,
    pub resoloution: (usize, usize),
    
    // These settings are used for calculation purposes only
    v_up: Unit<Vector3<f32>>,
    focus_dist: f32,
    viewport_width: f32,
    viewport_height: f32,
    v_fov: f32
}


#[derive (Copy, Clone, Default)]
pub struct CameraSettings {
    pub look_from: Point3<f32>,
    pub look_at: Point3<f32>,
    pub v_up: Vector3<f32>, 
    pub v_fov: f32, 
    pub aspect_ratio:f32, 
    pub aperture: f32, 
    pub focus_dist: f32,
    pub image_height: usize,
    pub image_width: usize,
}


#[derive (PartialEq, Debug, Copy, Clone)]
pub struct Orientation{
    pub u: Unit<Vector3<f32>>,
    pub v: Unit<Vector3<f32>>,
    pub w: Unit<Vector3<f32>>
}

impl Camera {

    pub fn new(settings: CameraSettings) -> Camera {
        let theta = deg_to_rad(settings.v_fov);
        let h = (0.5 * theta).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = settings.aspect_ratio * viewport_height;
        let w: Unit<Vector3<f32>> = Unit::new_normalize(settings.look_from - settings.look_at);
        let u: Unit<Vector3<f32>> = Unit::new_normalize(settings.v_up.cross(&w));
        let v: Unit<Vector3<f32>> = Unit::new_normalize(w.cross(&u));
        let v_up: Unit<Vector3<f32>> = Unit::new_normalize(settings.v_up);
        let v_fov = settings.v_fov;
        let orientation = Orientation::new(u,v,w);

        let focus_dist = settings.focus_dist;
        let origin: Point3<f32> = settings.look_from;
        let horizontal: Vector3<f32> = settings.focus_dist * viewport_width * u.into_inner();
        let vertical: Vector3<f32> = settings.focus_dist * viewport_height * v.into_inner();
        let lower_left_corner: Point3<f32> = origin - horizontal / 2.0 - vertical / 2.0 - settings.focus_dist * w.into_inner();

        let resoloution = (settings.image_width, settings.image_height);

        let lens_radius = settings.aperture/2.0;
        Camera{origin, horizontal, vertical, lower_left_corner, orientation, lens_radius, resoloution, v_up, focus_dist, viewport_width, viewport_height, v_fov}
    }

    pub fn get_ray(&self, s: f32, t:f32) -> Ray {
        let rd = self.lens_radius * sampler::rand_in_unit_disk();
        let offset = self.orientation.u().into_inner() * rd[0] + self.orientation.v().into_inner() * rd[1];

        Ray::new(self.origin + offset, Unit::new_normalize(self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset).into_inner())
    }

    pub fn translate(&mut self, forward: f32, right: f32, up: f32) {
        let delta =  - forward * self.orientation.w.into_inner() + right * self.orientation.u.into_inner() + up * self.v_up.into_inner();
        self.origin = self.origin + delta;
        self.lower_left_corner = self.lower_left_corner + delta;
    }

    pub fn rotate(&mut self, rotation_axis: Unit<Vector3<f32>>, angle: f32) {
        let look_direction = - self.orientation.w;
        let rotation = Rotation3::from_axis_angle(&rotation_axis, angle);
        let rotated_look_vector = rotation.transform_vector(&look_direction);

        self.orientation.w = - Unit::new_normalize(rotated_look_vector);
        self.orientation.u = Unit::new_normalize(self.v_up.cross(&self.orientation.w));
        self.orientation.v = Unit::new_normalize(self.orientation.w.cross(&self.orientation.u));
        self.vertical = self.focus_dist * self.viewport_height * self.orientation.v.into_inner();
        self.horizontal = self.focus_dist * self.viewport_width * self.orientation.u.into_inner();
        self.lower_left_corner = self.origin - self.horizontal/2.0 - self.vertical/2.0 - self.focus_dist * self.orientation.w.into_inner();

    }

    pub fn vertical_field_of_view(&self) ->f32 {
        self.v_fov
    }
}

impl Orientation{

    pub fn new(u: Unit<Vector3<f32>>, v: Unit<Vector3<f32>>, w: Unit<Vector3<f32>>) -> Orientation{
        Orientation{u, v, w}
    }

    pub fn u(&self) -> Unit<Vector3<f32>>{
        self.u
    }

    pub fn v(&self) -> Unit<Vector3<f32>>{
        self.v
    }

    pub fn w(&self) -> Unit<Vector3<f32>>{
        self.w
    }
}

