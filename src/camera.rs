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

#[derive (Clone, Copy, PartialEq)]
pub enum Pixel {
    Color(Color),
    Transparent,
    Outline
}

impl Pixel {

    pub fn write_color<T: std::io::Write>(self, writer: &mut T, samples: usize) {
        match self {
            Pixel::Color(color) => color.write_color(writer, samples),
            Pixel::Transparent => Color::default().write_color(writer, samples),
            Pixel::Outline => Color::new(1.0, 1.0, 1.0).write_color(writer, samples)
        }
    }

    pub fn get_color(&self) -> Color {
        match self {
            Pixel::Color(color) => *color,
            Pixel::Transparent => Color::new(0.0, 0.0, 0.0),
            Pixel::Outline => Color::new(1.0, 1.0, 1.0)
        }
    }
}

#[derive (Clone, Default)]
pub struct Image {
    pub pixels: Vec<Pixel>,
    pub samples_added: Vec<usize>,
    pub image_height: usize,
    pub image_width: usize
}

impl Image {
    pub fn new(image_width: usize, image_height: usize) -> Image {
        let samples_added = vec![0; image_width * image_height];
        Image{ pixels: vec![Pixel::Transparent; image_width * image_height], samples_added, image_height, image_width }
    }

    pub fn save(&self, path: &str) {
        let mut file = OpenOptions::new().create(true)
                                               .write(true)
                                               .open(path)
                                               .unwrap();

        write!(file, "P3\n{} {} \n255\n", self.image_width, self.image_height).unwrap();
        for (pixel, samples) in self.pixels.iter().zip(self.samples_added.iter()) {
            pixel.write_color(&mut file, *samples);
         }
    }

    pub fn output_rgba(&self) -> Vec<u8> {
        let mut rgbas = Vec::<u8>::with_capacity(self.pixels.len() * 4);
        for (pixel, samples) in self.pixels.iter().zip(self.samples_added.iter()){
            let rgb = pixel.get_color().scale_colors(*samples);
            if *samples > 1 {
                let a = 1;
            }
            for color in &rgb{
                rgbas.push(*color);
            }
            rgbas.push(255);
        }
        rgbas
    }
}

impl Index<usize> for Image{
    type Output = Pixel;
    fn index(&self, index: usize) -> &Self::Output {
        &self.pixels[index]
    }
}

impl IndexMut<usize> for Image{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.pixels[index]
    }
}

impl Add for Image {
    type Output = Image;

    fn add(self, other: Image) -> Image {
        let mut output = Image::new(self.image_width, self.image_height);
        for i in 0..self.pixels.len(){
            match (self[i], other.pixels[i])
            {
                (Pixel::Color(color_1), Pixel::Color(color_2)) =>  {
                    output[i] = Pixel::Color(color_1 + color_2);
                    output.samples_added[i] = self.samples_added[i] + other.samples_added[i];
                }
                (Pixel::Transparent, Pixel::Color(color_2)) => {
                    output[i] = Pixel::Color(color_2) ;
                    output.samples_added[i] = self.samples_added[i] + other.samples_added[i];
                }

                (Pixel::Color(color_1), Pixel::Transparent) => {
                    output[i] = Pixel::Color(color_1);
                    output.samples_added[i] = self.samples_added[i] + other.samples_added[i];
                }
                (Pixel::Transparent, Pixel::Transparent) => output[i] = Pixel::Transparent,
                (Pixel::Outline, _) => output[i] = Pixel::Outline,
                (_, Pixel::Outline) => output[i] = Pixel::Outline
            }
        }
        output
    }
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

