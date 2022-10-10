use crate::film::Film;
use crate::nalgebra::{Point3, Rotation3, Vector3};
use crate::raytracing::Ray;
use crate::sampler::{self, concentrically_sample_from_disk};
use crate::util::deg_to_rad;
use core::f32;
use nalgebra::{Matrix4, Perspective3, Point2, Translation3, Unit, UnitVector3};
use std::f32::consts::PI;

use super::camera_sample::CameraSample;

/// Type representing a perspective camera. Generates rays for use in the integrator. 
/// # Further Reading
/// https://pbr-book.org/3ed-2018/Camera_Models/Camera_Model#Camera
#[derive(Clone)]
pub struct Camera {
    /// The camera_to_world transformation matrix projects points in camera space into world space. In camera space, the camera
    /// reside at the origin. The `z`-axis of this coordinate system is mapped to the viewing direction, and the `y`-axis points
    /// vertically upwards.
    camera_to_world: Matrix4<f32>,
    /// The perspective transformation matrix projects points in camera space into screen space. The `x` and `y` coordinates are
    ///  first  divided by the `z` coordinate.
    ///
    /// If the image is square, they're then rescaled such that the `x'` and `y'` coordinates for all points within the field of
    /// view lie between `[-1, 1]`. Otherwise, the direction in which the image is narrower maps to `[-1, 1]`, and the wider
    /// direction maps to a proportionally larger range of screen space values.
    ///
    /// The projected `z'` coordinate is computed so that points on the near plane map to `z' = 0` and points on the far plane
    /// map to `z' = 1`.
    camera_to_screen: Matrix4<f32>,
    /// A screen_to_raster transformation matrix projects points in camera space into raster space. In raster space, depth values
    ///  are the same as in screen space, but the origin is in the upper-left hand corner of the image. The bottom right hand 
    /// corner is defined as `(image_width, image_height)`, measured in pixels.
    camera_to_raster: Matrix4<f32>,
    /// The inverse transformation of camera_to_world.
    world_to_camera: Matrix4<f32>,
    /// The inverse transformation of camera_to_screen.
    screen_to_camera: Matrix4<f32>,
    /// The inverse transformation of camera_to_raster.
    raster_to_camera: Matrix4<f32>,
    /// The radius of the lens. The larger this is, the greater the defocus
    /// blur.
    pub lens_radius: f32,
    /// How far away along the `z`-axis the focal plane is (i.e. the plane on which all objects are perfectly in focus). The
    /// focal plane cuts across the `x` and `y` axis.
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
    pub fn new(camera_to_world: Matrix4<f32>, film: Film, focus_dist: f32) {}

    pub fn calculate_camera_to_screen(field_of_view: f32, near_z_plane: f32, far_z_plane: f32) -> Matrix4<f32> {
        let m33 = far_z_plane / (far_z_plane - near_z_plane);
        let m34 = -far_z_plane * near_z_plane / (far_z_plane - near_z_plane);

        #[rustfmt::skip]
        let perspective_transformation = Matrix4::new(1.0, 0.0, 0.0, 0.0,
                                                      0.0, 1.0, 0.0, 0.0,
                                                      0.0, 0.0, m33,  m34,
                                                      0.0, 0.0, 1.0, 0.0);

        let x_y_scale_factor = 1.0 / (field_of_view * PI / 180.0).tan() / 2.0;
        let scale_vector = Vector3::new(x_y_scale_factor, x_y_scale_factor, 1.0);
        let scale = Matrix4::new_nonuniform_scaling(&scale_vector);
        perspective_transformation * scale
    }

    pub fn calculate_screen_to_raster(
        &self,
        screen_bottom_left_corner: Point2<f32>,
        screen_top_right_corner: Point2<f32>,
    ) -> Matrix4<f32> {
        let screen_width = screen_top_right_corner.x - screen_bottom_left_corner.x;
        let screen_height = screen_top_right_corner.y - screen_bottom_left_corner.y;
        let image_width = self.film.image_width as f32;
        let image_height = self.film.image_height as f32;

        let scale_to_raster = Matrix4::new_nonuniform_scaling(&Vector3::new(image_width, image_height, 1.0));
        let rescale_screen = Matrix4::new_nonuniform_scaling(&Vector3::new(1.0 / screen_width, 1.0 / screen_height, 1.0));
        let translate = Matrix4::new_translation(&Vector3::new(-screen_bottom_left_corner.x, -screen_top_right_corner.y, 0.0));

        scale_to_raster * rescale_screen * translate
    }

    /// Returns a ray emitted from the camera, as well as a floating-point value that affects how much the radiance arriving at
    /// the film plane along the generated ray will contribute to the final image.
    /// # Further Reading
    /// [Thin Lens Model][https://pbr-book.org/3ed-2018/Camera_Models/Projective_Camera_Models#TheThinLensModelandDepthofField]
    pub fn get_ray(&self, camera_sample: CameraSample) -> (Ray, f32) {
        // Elevate the ray-film intersection into 3 dimensions
        let ray_film_intersection =
            Point3::new(camera_sample.ray_film_intersection.x, camera_sample.ray_film_intersection.y, 0.0);

        // Transform the intersection into camera space
        let ray_film_intersection = self.raster_to_camera.transform_vector(ray_film_intersection);

        // In camera space, all rays originate from the origin
        let ray_position = Point3::<f32>::new(0.0, 0.0, 0.0);
        let ray_direction = ray_film_intersection.norm();
        let mut ray = Ray::new(ray_position, ray_direction);

        // Modify ray for depth of field
        if self.lens_radius > 0.0 {
            //Sample points on lens
            let lens_sample = self.lens_radius * concentrically_sample_from_disk(camera_sample.ray_lens_intersection);

            //Compute the intersection point of the ray with the plane of focus
            let ray_focal_plane_intersection = ray.dir * self.focus_dist / ray.dir.z;

            // Update ray for effect of lens
            ray.orig = Point3::new(lens_sample.x, lens_sample.y, 0.0);
            ray.dir = UnitVector3::new_normalize(ray_focal_plane_intersection - ray.orig);
        }
        (ray, 1.0)
    }

    pub fn translate(&mut self, forward: f32, right: f32, up: f32) {
        let translation = Matrix4::new_translation(&Vector3::new(forward, right, up));
        self.camera_to_world = translation * self.camera_to_world;
        // Unwrap the inverse - translations are always invertible.
        self.world_to_camera = self.world_to_camera * translation.try_inverse().unwrap();
    }

    pub fn rotate(&mut self, rotation_axis: Unit<Vector3<f32>>, angle: f32) {
        let rotation = Matrix4::new_rotation(rotation_axis.into_inner() * angle);
        self.camera_to_world = rotation * self.camera_to_world;
        // Unwrap the inverse - rotations are always invertible.
        self.world_to_camera = self.world_to_camera * rotation.try_inverse().unwrap();
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
