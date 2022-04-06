use core::f64;
use std::fs::OpenOptions;
use std::ops::Add;
use std::ops::Index;
use std::ops::IndexMut;
use std::fs::File;
use std::io::Write;

use crate::deg_to_rad;
use crate::vec::*;
use crate::ray::*;

#[derive (Copy, Clone, Default)]
pub struct Camera {
    pub origin: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
    pub orientation: Orientation,
    pub lens_radius: f64,
    pub resoloution: (usize, usize)
}


#[derive (Copy, Clone, Default)]
pub struct CameraSettings {
    pub look_from: Point3,
    pub look_at: Point3,
    pub v_up: Vec3, 
    pub v_fov: f64, 
    pub aspect_ratio:f64, 
    pub aperture: f64, 
    pub focus_dist: f64,
    pub image_height: usize,
    pub image_width: usize,
}


#[derive (PartialEq, Debug, Copy, Clone, Default)]
pub struct Orientation{
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3
}

impl Camera {

    pub fn new(settings: CameraSettings) -> Camera {
        let theta = deg_to_rad(settings.v_fov);
        let h = (0.5*theta).tan();
        let viewport_height = 2.0*h;
        let viewport_width = settings.aspect_ratio * viewport_height;

        let w = (settings.look_from - settings.look_at).unit_vector();
        let u = Vec3::cross(settings.v_up, w).unit_vector();
        let v = Vec3::cross(w, u).unit_vector();
        let orientation = Orientation::new(u,v,w);

        let origin = settings.look_from;
        let horizontal = settings.focus_dist * viewport_width * u;
        let vertical = settings.focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - settings.focus_dist * w;

        let resoloution = (settings.image_width, settings.image_height);

        let lens_radius = settings.aperture/2.0;
        Camera{origin, horizontal, vertical, lower_left_corner, orientation, lens_radius, resoloution}
    }

    pub fn get_ray(&self, s: f64, t:f64) -> Ray {
        let rd = self.lens_radius * Vec3::rand_in_unit_disk();
        let offset = self.orientation.u() * rd.x() + self.orientation.v() * rd.y();

        Ray::new(self.origin + offset, (self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset).unit_vector())
    }
}

impl Orientation{

    pub fn new(u: Vec3, v: Vec3, w: Vec3) -> Orientation{
        Orientation{u, v, w}
    }

    pub fn u(&self) -> Vec3{
        self.u
    }

    pub fn v(&self) -> Vec3{
        self.v
    }

    pub fn w(&self) -> Vec3{
        self.w
    }
}

