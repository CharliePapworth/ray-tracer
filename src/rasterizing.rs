use enum_dispatch::enum_dispatch;
use crate::camera::Camera;
use crate::GeometricPrimitive;

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