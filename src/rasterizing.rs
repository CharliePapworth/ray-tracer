use enum_dispatch::enum_dispatch;
use crate::camera::Camera;
use crate::image::{Raster, Pixel};
use crate::primitives::{GeometricPrimitives};
use crate::image::Color;

#[enum_dispatch] 
pub trait Rasterize: Send + Sync{
    fn outline(&self, cam: &Camera) -> Option<Vec<[usize; 2]>>;
}

impl<W> Rasterize for Vec<W> where W: Rasterize {
    fn outline(&self, cam: &Camera) -> Option<Vec<[usize; 2]>> {
        let mut pixels: Vec<[usize; 2]> = Default::default();
        for object in self {
            if let Some(mut new_pixels) = object.outline(cam) {
                pixels.append(&mut new_pixels);
            }
        }

        if pixels.len() == 0 {
            None
        } else {
            Some(pixels)
        }
    }
}

pub fn rasterize(mut image: Raster, cam: Camera, geometric_primitives: &GeometricPrimitives)  -> Raster {
    
    let image_width = image.image.image_width;
    let image_height = image.image.image_height;
    
    if let Some(pixels) = geometric_primitives.outline(&cam) {
        for pixel in pixels {
            let pixel_index = (image_height - pixel[1] - 1) * image_width + pixel[0];
            image.image.pixels[pixel_index] = Pixel::new(Color::new(1.0, 1.0, 1.0), 1.0);
            image.z_buffer[pixel_index] = 1.0;
        }
    }
    image
}