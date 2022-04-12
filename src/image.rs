use std::{ops::{Index, IndexMut, Add, AddAssign}, fs::OpenOptions};
use std::fs::File;
use std::io::Write;
use std::ops;

use enum_dispatch::enum_dispatch;
use delegate::delegate;

use crate::{vec::Color, threads::DrawMode, util::bound};

#[derive (Clone, PartialEq)]


pub struct Pixel {
    color: Color,
    alpha: f64,
}

impl Pixel{

    pub fn new(color: Color, alpha: f64) -> Pixel {
        Pixel { color, alpha }
    }

    pub fn to_rgb(&self) -> [u8; 3] {
        let r = self.color.x().sqrt();
        let g = self.color.y().sqrt();
        let b = self.color.z().sqrt();

        let ir = (256.0*bound(r, 0.0, 0.999)) as u8;
        let ig = (256.0*bound(g, 0.0, 0.999)) as u8;
        let ib = (256.0*bound(b, 0.0, 0.999)) as u8;

        [ir, ig, ib]
    }

    pub fn write_color<T: std::io::Write>(&self, writer: &mut T)
    {
        let [ir, ig, ib] = self.to_rgb();
        writeln!(writer, "{} {} {}", ir, ig, ib).unwrap();
    }

    pub fn over(&self, under: &Pixel) -> Pixel {
        let alpha = self.alpha + under.alpha * (1.0 - self.alpha);
        let color = (self.color * self.alpha + under.color * under.alpha * (1.0 - self.alpha)) / alpha;
        Pixel::new(color, alpha)
    }
}

#[derive (Clone, PartialEq)]
pub struct Image {
    pub pixels: Vec<Pixel>,
    pub image_width: usize,
    pub image_height: usize
}

impl Image {

    pub fn new(image_width: usize, image_height: usize) -> Image {
        let pixels = vec![Pixel::new(Color::new(0.0, 0.0, 0.0), 0.0); image_width * image_height];
        Image { pixels, image_width, image_height }
    }

    pub fn output_rgba(&self) -> Vec<u8> {
        let mut rgbas = Vec::<u8>::with_capacity(self.pixels.len() * 4);
        for pixel in self.pixels.iter() {
            let rgb = pixel.to_rgb();
            for color in &rgb {
                rgbas.push(*color);
            }
            rgbas.push(255);
        }
        rgbas
    }

    pub fn over(&self, under: &Image) -> Image {
        if self.image_height != under.image_height || self.image_width != under.image_width {
            panic!("Image dimensions do not match");
        }

        let mut image = Image::new(self.image_width, self.image_height);
        image.pixels = self.pixels.iter().zip(under.pixels.iter()).map(|(over_pixel, under_pixel)| over_pixel.over(under_pixel)).collect();
        image
    }

    pub fn save(&self, path: &str) {
        let mut file = OpenOptions::new().create(true)
                                                .write(true)
                                                .open(path)
                                                .unwrap();

        write!(file, "P3\n{} {} \n255\n", self.image_width, self.image_height).unwrap();
        for pixel in &self.pixels {
            pixel.write_color(&mut file);
        }
    }
}

#[derive (Clone, PartialEq)]
pub struct RaytracedImage {
    pub image: Image,
    pub samples: usize
}

impl RaytracedImage {
    pub fn new(image_width: usize, image_height: usize) -> RaytracedImage {
        let image = Image::new(image_width, image_height);
        let samples = 0;
        RaytracedImage{ image, samples }
    }

    pub fn clear(&mut self) {
        self.image = Image::new(self.image.image_width, self.image.image_height);
        self.samples = 0;
    }

    pub fn to_image(&self) -> Image{
        let samples = self.samples;
        let mut image = self.image.clone();
        let scale = 1.0 / (samples as f64);
        for pixel in &mut image.pixels {
            pixel.color[0] *= scale;
            pixel.color[1] *= scale;
            pixel.color[2] *= scale;
        }
        image
    }

    pub fn output_rgba(&self) -> Vec<u8> {
        self.to_image().output_rgba()
    }
}

impl Add for RaytracedImage {
    type Output = RaytracedImage;

    fn add(self, other: RaytracedImage) -> RaytracedImage {
        if self.image.image_height != other.image.image_height || self.image.image_width != other.image.image_width {
            panic!("Image dimensions do not match");
        }
        
        let mut output = RaytracedImage::new(self.image.image_height, self.image.image_width);
        output.samples = self.samples + other.samples;
        output.image.pixels = self.image.pixels.iter().zip(other.image.pixels.iter()).map(|(a,b)| Pixel::new(a.color + b.color, 1.0)).collect();
        output
    }
}

impl AddAssign for RaytracedImage {
    fn add_assign(&mut self, other: RaytracedImage) {
        if self.image.image_height != other.image.image_height || self.image.image_width != other.image.image_width {
            panic!("Image dimensions do not match");
        }

        self.samples += other.samples;
        self.image.pixels = self.image.pixels.iter().zip(other.image.pixels.iter()).map(|(a,b)| Pixel::new(a.color + b.color, 1.0)).collect();
    }
}


#[derive (Clone, PartialEq)]
pub struct Raster {
    pub image: Image,
    pub z_buffer: Vec<f32>,
}

impl Raster {
    pub fn new(image_width: usize, image_height: usize) -> Raster {
        let image = Image::new(image_width, image_height);
        let z_buffer = vec![f32::INFINITY; image_width * image_height];
        Raster { image, z_buffer }
    }

    pub fn clear(&mut self) {
        self.image = Image::new(self.image.image_width, self.image.image_height);
        self.z_buffer = vec![f32::INFINITY; self.image.image_width * self.image.image_height];
    }

    pub fn output_rgba(&self) -> Vec<u8> {
        self.image.output_rgba()
    }

    pub fn to_image(&self) -> Image {
        self.image.clone()
    }
}

impl Add for Raster {
    type Output = Raster;

    fn add(self, other: Raster) -> Raster {
        if self.image.image_height != other.image.image_height || self.image.image_width != other.image.image_width {
            panic!("Image dimensions do not match");
        }
        
        let mut output = Raster::new(self.image.image_height, self.image.image_width);
        for index in 0..self.image.pixels.len() {
            if self.z_buffer[index] < other.z_buffer[index] {
                output.image.pixels[index] = self.image.pixels[index].over(&other.image.pixels[index]);
            } else if self.z_buffer[index] > other.z_buffer[index] {
                output.image.pixels[index] = other.image.pixels[index].over(&self.image.pixels[index]);
            }
        }
        output
    }
}

impl AddAssign for Raster {
    fn add_assign(&mut self, other: Raster) {
        if self.image.image_height != other.image.image_height || self.image.image_width != other.image.image_width {
            panic!("Image dimensions do not match");
        }

        for index in 0..self.image.pixels.len() {
            if self.z_buffer[index] < other.z_buffer[index] {
                self.image.pixels[index] = self.image.pixels[index].over(&other.image.pixels[index]);
            } else if self.z_buffer[index] > other.z_buffer[index] {
                self.image.pixels[index] = other.image.pixels[index].over(&self.image.pixels[index]);
            }
        }
    }
}


#[derive (Clone)]
pub struct CompositeImage {
    raster: Raster,
    outline: Raster,
    raytrace: RaytracedImage,
    pub image_width: usize,
    pub image_height: usize
}

#[derive (Copy, Clone, PartialEq)]
pub enum PrimaryImageType {
    Raster,
    Raytrace
}

#[derive (Clone, PartialEq)]
pub enum PrimaryImage {
    Raster(Raster),
    Raytrace(RaytracedImage)
}

impl CompositeImage {
    pub fn new(image_width: usize, image_height: usize) -> CompositeImage {
        let raster = Raster::new(image_width, image_height);
        let outline = Raster::new(image_width, image_height);
        let raytraced_image = RaytracedImage::new(image_width, image_height);

        CompositeImage { raster, outline, raytrace: raytraced_image, image_width, image_height}
    }

    pub fn clear(&mut self) {
        self.raster.clear();
        self.outline.clear();
        self.raytrace.clear();
    }

    pub fn output(&self, primary_image: PrimaryImageType, outlining_on: bool) -> Image {
        let mut image: Image;
        match primary_image {
            PrimaryImageType::Raster => image = self.raster.to_image(),
            PrimaryImageType::Raytrace => {
                if self.raytrace.samples == 0 {
                    image = self.outline.to_image();
                 }
                 else {
                     image = self.raytrace.to_image();
                 }
            }
        }
        
        if outlining_on {
            image = self.outline.image.over(&image);
        }

        image
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
        output.raytrace = self.raytrace + other.raytrace;
        output
    }
}

impl_op_ex_commutative!(+ |lhs: CompositeImage, rhs: Raster| -> CompositeImage { 
    CompositeImage {raster: lhs.raster + rhs, outline: lhs.outline, raytrace: lhs.raytrace, image_height: lhs.image_height, image_width: lhs.image_width }
});

impl_op_ex_commutative!(+ |lhs: CompositeImage, rhs: RaytracedImage| -> CompositeImage { 
    CompositeImage {raster: lhs.raster, outline: lhs.outline, raytrace: lhs.raytrace + rhs, image_height: lhs.image_height, image_width: lhs.image_width }
});

impl_op_ex_commutative!(+ |lhs: CompositeImage, rhs: CompositeImageContribution| -> CompositeImage {
    match rhs {
        CompositeImageContribution::Outline(outline) => lhs + outline,
        CompositeImageContribution::Raster(raster) => lhs + raster,
        CompositeImageContribution::RaytracedImage(raytrace) => lhs + raytrace
    } 
});

impl_op_ex!(+= |lhs: &mut CompositeImage, rhs: Raster| { lhs.raster += rhs});
impl_op_ex!(+= |lhs: &mut CompositeImage, rhs: RaytracedImage| { lhs.raytrace += rhs});
impl_op_ex!(+= |lhs: &mut CompositeImage, rhs: CompositeImage| { 
    lhs.raytrace += rhs.raytrace;
    lhs.outline += rhs.outline;
    lhs.raster += rhs.raster;
});

impl_op_ex!(+= |lhs: &mut CompositeImage, rhs: CompositeImageContribution| {
    match rhs {
        CompositeImageContribution::Outline(outline) => lhs.outline += outline,
        CompositeImageContribution::Raster(raster) => lhs.raster += raster,
        CompositeImageContribution::RaytracedImage(raytrace) => lhs.raytrace += raytrace
    }
});

#[derive (Clone)]
pub enum CompositeImageContribution {
    Outline(Raster),
    Raster(Raster),
    RaytracedImage(RaytracedImage)
}

