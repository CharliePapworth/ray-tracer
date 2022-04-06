use std::{ops::{Index, IndexMut, Add, AddAssign}, fs::OpenOptions};
use std::fs::File;
use std::io::Write;
use std::ops;

use enum_dispatch::enum_dispatch;
use delegate::delegate;

use crate::{vec::Color, threads::DrawMode};

#[derive (Clone, PartialEq)]
pub struct ImageLayer {
    pub pixels: Vec<Option<Color>>,
    pub image_width: usize,
    pub image_height: usize
}

impl ImageLayer {
    pub fn new(image_width: usize, image_height: usize, background: Color) -> ImageLayer {
        let pixels = vec![None; image_width * image_height];
        ImageLayer { pixels, image_width, image_height }
    }

    pub fn clear(&mut self) {
        self.pixels = vec![None; self.image_width * self.image_height];
    }

    pub fn output_rgba(&self, background: Color) -> Vec<u8> {
        let mut rgbas = Vec::<u8>::with_capacity(self.pixels.len() * 4);
        for pixel in self.pixels.iter() {
            let mut color = background;
            if let Some(pixel_color) = pixel{
                color = *pixel_color;
            }
            let rgb = color.scale_colors(1);
            for color in &rgb {
                rgbas.push(*color);
            }
            rgbas.push(255);
        }
        rgbas
    }
}


#[derive (Clone, PartialEq)]
pub struct RaytracedImage {
    pub pixels: Vec<Color>,
    pub samples: usize,
    pub image_width: usize,
    pub image_height: usize
}



impl RaytracedImage {
    pub fn new(image_width: usize, image_height: usize) -> RaytracedImage {
        let pixels = vec![Color::default(); image_width * image_height];
        let samples = 0;
        RaytracedImage{ pixels, samples, image_width, image_height }
    }

    pub fn clear(&mut self) {
        self.pixels = vec![Color::default(); self.image_width * self.image_height];
        self.samples = 0;
    }

    pub fn output_rgba(&self) -> Vec<u8> {
        let mut rgbas = Vec::<u8>::with_capacity(self.pixels.len() * 4);
        for pixel in self.pixels.iter() {
            let rgb = pixel.scale_colors(self.samples);
            for color in &rgb {
                rgbas.push(*color);
            }
            rgbas.push(255);
        }
        rgbas
    }
}

impl Add for RaytracedImage {
    type Output = RaytracedImage;

    fn add(self, other: RaytracedImage) -> RaytracedImage {
        if self.image_height != other.image_height || self.image_width != other.image_width {
            panic!("Image dimensions do not match");
        }
        
        let mut output = RaytracedImage::new(self.image_height, self.image_width);
        output.samples = self.samples + other.samples;
        output.pixels = self.pixels.iter().zip(other.pixels.iter()).map(|(a,b)| a + b).collect();
        output
    }
}

impl AddAssign for RaytracedImage {
    fn add_assign(&mut self, other: RaytracedImage) {
        if self.image_height != other.image_height || self.image_width != other.image_width {
            panic!("Image dimensions do not match");
        }
        self.pixels = self.pixels.iter().zip(other.pixels.iter()).map(|(a,b)| a + b).collect();
        self.samples += other.samples;
    }
}

#[derive (Clone, PartialEq)]
pub struct OutlinedImage {
    pub pixels: Vec<bool>,
    pub image_width: usize,
    pub image_height: usize,

}

impl OutlinedImage {
    pub fn new(image_width: usize, image_height: usize) -> OutlinedImage {
        let pixels = vec![false; image_width * image_height];
        OutlinedImage{ pixels, image_width, image_height }
    }

    pub fn clear(&mut self) {
        self.pixels = vec![false; self.image_width * self.image_height];
    }

    pub fn output_rgba(&self) -> Vec<u8> {
        let mut rgbas = Vec::<u8>::with_capacity(self.pixels.len() * 4);
        for outlined in self.pixels.iter() {
            let mut pixel = Color::new(0.0, 0.0, 0.0);
            if *outlined == true {
                pixel = Color::new(1.0, 1.0, 1.0);
            }
            let rgb = pixel.scale_colors(1);
            for color in &rgb {
                rgbas.push(*color);
            }
            rgbas.push(255);
        }
        rgbas
    }
}

impl Add for OutlinedImage {
    type Output = OutlinedImage;

    fn add(self, other: OutlinedImage) -> OutlinedImage {
        if self.image_height != other.image_height || self.image_width != other.image_width {
            panic!("Image dimensions do not match");
        }
        
        let mut output = OutlinedImage::new(self.image_height, self.image_width);
        output.pixels = self.pixels.iter().zip(other.pixels.iter()).map(|(a,b)| a | b).collect();
        output
    }
}

impl AddAssign for OutlinedImage {
    fn add_assign(&mut self, other: OutlinedImage) {
        if self.image_height != other.image_height || self.image_width != other.image_width {
            panic!("Image dimensions do not match");
        }
        self.pixels = self.pixels.iter().zip(other.pixels.iter()).map(|(a,b)| a | b).collect();
    }
}

#[derive (Clone, PartialEq)]
pub struct Raster {
    pub pixels: Vec<Color>,
    pub z_buffer: Vec<f32>,
    pub image_width: usize,
    pub image_height: usize
}

impl Raster {
    pub fn new(image_width: usize, image_height: usize) -> Raster {
        let pixels = vec![Color::default(); image_width * image_height];
        let z_buffer = vec![f32::INFINITY; image_width * image_height];
        Raster { pixels, z_buffer, image_width, image_height }
    }

    pub fn clear(&mut self) {
        self.pixels = vec![Color::default(); self.image_width * self.image_height];
        self.z_buffer = vec![f32::INFINITY; self.image_width * self.image_height];
    }

    pub fn output_rgba(&self) -> Vec<u8> {
        let mut rgbas = Vec::<u8>::with_capacity(self.pixels.len() * 4);
        for pixel in self.pixels.iter() {
            let rgb = pixel.scale_colors(1);
            for color in &rgb {
                rgbas.push(*color);
            }
            rgbas.push(255);
        }
        rgbas
    }
    
}

impl Add for Raster {
    type Output = Raster;

    fn add(self, other: Raster) -> Raster {
        if self.image_height != other.image_height || self.image_width != other.image_width {
            panic!("Image dimensions do not match");
        }
        
        let mut output = Raster::new(self.image_height, self.image_width);
        for index in 0..self.pixels.len() {
            if self.z_buffer[index] < other.z_buffer[index] {
                output.pixels[index] = self.pixels[index];
            } else {
                output.pixels[index] = other.pixels[index];
            }
        }
        output
    }
}

impl AddAssign for Raster {
    fn add_assign(&mut self, other: Raster) {
        if self.image_height != other.image_height || self.image_width != other.image_width {
            panic!("Image dimensions do not match");
        }
        for index in 0..self.pixels.len() {
            if self.z_buffer[index] < other.z_buffer[index] {
                self.pixels[index] = self.pixels[index];
            } else {
                self.pixels[index] = other.pixels[index];
            }
        }
    }
}


#[derive (Clone)]
pub struct CompositeImage {
    raster: Raster,
    outline: OutlinedImage,
    raytraced_image: RaytracedImage,
    pub image_width: usize,
    pub image_height: usize
}

#[derive (Copy, Clone, PartialEq)]
pub enum PrimaryImage {
    Raster,
    Raytrace
}

impl CompositeImage {
    pub fn new(image_width: usize, image_height: usize) -> CompositeImage {
        let raster = Raster::new(image_width, image_height);
        let outline = OutlinedImage::new(image_width, image_height);
        let raytraced_image = RaytracedImage::new(image_width, image_height);

        CompositeImage { raster, outline, raytraced_image, image_width, image_height}
    }

    pub fn clear(&mut self) {
        self.raster.clear();
        self.outline.clear();
        self.raytraced_image.clear();
    }

    pub fn output_rgba(&self, primary_image: PrimaryImage, outlining_on: bool) -> Vec<u8> {
        match (primary_image, outlining_on) {
            (PrimaryImage::Raster, false) => self.raster.output_rgba(),
            (PrimaryImage::Raytrace, false) => self.raytraced_image.output_rgba(),
            (PrimaryImage::Raster, true) => {
                let mut rgbas = Vec::<u8>::with_capacity(self.raster.pixels.len() * 4);
                for (pixel, outline) in self.raster.pixels.iter().zip(self.outline.pixels.iter()) {
                    let mut rgb: [u8; 3] = [255, 255, 255]; 
                    if *outline == false {
                        rgb = pixel.scale_colors(1);  
                    } 
                    for color in &rgb {
                        rgbas.push(*color);
                    }
                    rgbas.push(255);
                }
                rgbas
            }
            (PrimaryImage::Raytrace, true) => {
                let mut rgbas = Vec::<u8>::with_capacity(self.raytraced_image.pixels.len() * 4);
                for (pixel, outline) in self.raytraced_image.pixels.iter().zip(self.outline.pixels.iter()) {
                    let mut rgb: [u8; 3] = [255, 255, 255]; 
                    if *outline == false {
                        rgb = pixel.scale_colors(self.raytraced_image.samples);  
                    } 
                    for color in &rgb {
                        rgbas.push(*color);
                    }
                    rgbas.push(255);
                }
                rgbas
            }
        }
    }

    pub fn save(&self, path: &str, draw_mode: DrawMode, outlining_on: bool) {
        let mut file = OpenOptions::new().create(true)
                                                .write(true)
                                                .open(path)
                                                .unwrap();

        write!(file, "P3\n{} {} \n255\n", self.image_width, self.image_height).unwrap();
        match (draw_mode, outlining_on) {
            (DrawMode::Outline, false) => {
                for pixel in self.raster.clone().pixels.iter() {
                    pixel.write_color(&mut file, 1);
                }
            }
            (DrawMode::Raytrace, false) => {
                for pixel in self.raytraced_image.clone().pixels.iter() {
                    pixel.write_color(&mut file, self.raytraced_image.samples);
                }
            }
            (DrawMode::Outline, true) => {
                for pixel in self.raster.clone().pixels.iter() {
                    pixel.write_color(&mut file, 1);
                }
            }
            (DrawMode::Raytrace, true) => {
                for pixel in self.raytraced_image.clone().pixels.iter() {
                    pixel.write_color(&mut file, self.raytraced_image.samples);
                }
            }
        }
    }     
}

impl Add for CompositeImage {
    type Output = CompositeImage;

    fn add(self, other: CompositeImage) -> CompositeImage {
        if self.image_height != other.image_height || self.image_width != other.image_width {
            panic!("Image dimensions do not match");
        }
        
        let mut output = CompositeImage::new(self.image_height, self.image_width);
        output.outline = self.outline + other.outline;
        output.raster = self.raster + other.raster;
        output.raytraced_image = self.raytraced_image + other.raytraced_image;
        output
    }
}


impl_op_ex_commutative!(+ |lhs: CompositeImage, rhs: OutlinedImage| -> CompositeImage { 
    CompositeImage {raster: lhs.raster, outline: lhs.outline + rhs, raytraced_image: lhs.raytraced_image, image_height: lhs.image_height, image_width: lhs.image_width }
});

impl_op_ex_commutative!(+ |lhs: CompositeImage, rhs: Raster| -> CompositeImage { 
    CompositeImage {raster: lhs.raster + rhs, outline: lhs.outline, raytraced_image: lhs.raytraced_image, image_height: lhs.image_height, image_width: lhs.image_width }
});

impl_op_ex_commutative!(+ |lhs: CompositeImage, rhs: RaytracedImage| -> CompositeImage { 
    CompositeImage {raster: lhs.raster, outline: lhs.outline, raytraced_image: lhs.raytraced_image + rhs, image_height: lhs.image_height, image_width: lhs.image_width }
});

impl_op_ex_commutative!(+ |lhs: CompositeImage, rhs: Image| -> CompositeImage {
    match rhs {
        Image::Outline(outline) => lhs + outline,
        Image::Raster(raster) => lhs + raster,
        Image::RaytracedImage(raytrace) => lhs + raytrace
    } 
});

impl_op_ex!(+= |lhs: &mut CompositeImage, rhs: OutlinedImage| { lhs.outline += rhs});
impl_op_ex!(+= |lhs: &mut CompositeImage, rhs: Raster| { lhs.raster += rhs});
impl_op_ex!(+= |lhs: &mut CompositeImage, rhs: RaytracedImage| { lhs.raytraced_image += rhs});
impl_op_ex!(+= |lhs: &mut CompositeImage, rhs: CompositeImage| { 
    lhs.raytraced_image += rhs.raytraced_image;
    lhs.outline += rhs.outline;
    lhs.raster += rhs.raster;
});

impl_op_ex!(+= |lhs: &mut CompositeImage, rhs: Image| {
    match rhs {
        Image::Outline(outline) => lhs.outline += outline,
        Image::Raster(raster) => lhs.raster += raster,
        Image::RaytracedImage(raytrace) => lhs.raytraced_image += raytrace
    }
});





#[derive (Clone)]
pub enum Image {
    Outline(OutlinedImage),
    Raster(Raster),
    RaytracedImage(RaytracedImage)
}
