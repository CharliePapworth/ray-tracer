use crate::film::Film;
use crate::nalgebra::{Point3, Rotation3, Vector3};
use crate::raytracing::Ray;
use crate::sampler;
use crate::util::deg_to_rad;
use core::f32;
use nalgebra::{Matrix4, Perspective3, Translation3, Unit};
use std::f32::consts::PI;

#[derive(Clone)]
pub struct Camera {
    /// The camera_to_world transformation matrix projects points in camera space into world space.
    /// In camera space, the camera reside at the origin. The `z`-axis of this coordinate system is mapped to the viewing
    /// direction, and the `y`-axis points vertically upwards.
    camera_to_world: Matrix4<f32>,
    /// The perspective transformation matrix projects points in camera space into screen space.
    /// The `x'` and `y'` coordinates of the projected points are equal to the unprojected `x` and `y` coordinates divided by the `z` coordinate.
    /// The projected `z'` coordinate is computed so that points on the near plane map to `z' = 0` and points on the far plane map to `z = 1`.
    camera_to_screen: Matrix4<f32>,
    /// A screen_to_raster transformation matrix projects points in camera space into raster space. In raster space, depth values are the same as in screen
    /// space, but the origin is in the upper-left hand corner of the image. The bottom right hand corner is defined as `(image_width, image_height)`,
    /// measured in pixels.
    camera_to_raster: Matrix4<f32>,
    /// The radius of the lens. The larger this is, the greater the defocus blur.
    pub lens_radius: f32,
    /// How far away along the `z`-axis the focal plane is (i.e. the plane on which all objects are perfectly in focus).
    /// The focal plane cuts across the `x` and `y` axis.
    pub focus_dist: f32,
    pub film: Film,
    pub field_of_view: f32,
}

#[derive(Clone)]
pub struct CameraSettings {
    pub look_from: Point3<f32>,
    pub look_at: Point3<f32>,
    pub v_up: Vector3<f32>,
    pub v_fov: f32,
    pub aspect_ratio: f32,
    pub aperture: f32,
    pub focus_dist: f32,
    pub film: Film,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Orientation {
    pub u: Unit<Vector3<f32>>,
    pub v: Unit<Vector3<f32>>,
    pub w: Unit<Vector3<f32>>,
}

impl Camera {
    /// # Arguments
    ///
    /// * `camera_to_world` - A transformation from camera space to world space.
    /// * `film` - The film the camera writes to.
    ///  * `focus_dist` - The focus distance of the camera.
    /// # Further Reading
    /// Camera space definitions: https://pbr-book.org/3ed-2018/Camera_Models/Camera_Model#Camera
    pub fn new(camera_to_world: Matrix4<f32>, film: Film, focus_dist: f32) {}

    pub fn calculate_camera_to_screen(
        field_of_view: f32,
        near_z_plane: f32,
        far_z_plane: f32,
    ) -> Matrix4<f32> {
        let m33 = far_z_plane / (far_z_plane - near_z_plane);
        let m34 = -far_z_plane * near_z_plane / (far_z_plane - near_z_plane);

        #[rustfmt::skip]
        let perspective_transformation = Matrix4::new(1.0, 0.0, 0.0, 0.0,
                                                      0.0, 1.0, 0.0, 0.0,
                                                      0.0, 0.0, m33,  m34,
                                                      0.0, 0.0, 1.0, 0.0);

        let inverse_tan_angle = 1.0 / (field_of_view * PI / 180.0).tan() / 2.0;
        let scale_vector = Vector3::new(inverse_tan_angle, inverse_tan_angle, 1.0);
        let scale = Matrix4::new_nonuniform_scaling(&scale_vector);
        perspective_transformation * scale
    }

    pub fn calculate_screen_to_raster() {
        // let translation = Translation3::new()
        // let translation = Matrix4::new_translation(translation)
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        todo!()
    }

    pub fn translate(&mut self, forward: f32, right: f32, up: f32) {}

    pub fn rotate(&mut self, rotation_axis: Unit<Vector3<f32>>, angle: f32) {}

    pub fn vertical_field_of_view(&self) -> f32 {
        todo!()
    }
}

impl Orientation {
    pub fn new(u: Unit<Vector3<f32>>, v: Unit<Vector3<f32>>, w: Unit<Vector3<f32>>) -> Orientation {
        Orientation { u, v, w }
    }

    pub fn u(&self) -> Unit<Vector3<f32>> {
        self.u
    }

    pub fn v(&self) -> Unit<Vector3<f32>> {
        self.v
    }

    pub fn w(&self) -> Unit<Vector3<f32>> {
        self.w
    }
}
