use crate::camera::Camera;
use crate::geometry::lines::*;
use crate::rasterizing::Rasterize;
use crate::nalgebra::{Vector3, Point3};
use crate::primitives::bvh::*;
use crate::material::*;
use crate::raytracing::{HitRecord, Hit, Ray};
use crate::image::Color;


#[derive (Copy, Clone)]
pub enum RectAxes {
    XY,
    XZ,
    YZ
}

#[derive (Copy, Clone)]
pub struct Rect {
    mat: Material,
    axes: RectAxes,
    corners: [f64; 4],
    k: f64
}

impl Rect {
    pub fn new(axes: RectAxes, axis1_min: f64, axis1_max: f64, axis2_min: f64, axis2_max: f64, k: f64, mat: Material) -> Rect {
        Rect{axes, corners: [axis1_min, axis1_max, axis2_min, axis2_max], k, mat}
    }   

    /// Returns the indices corresponding to the dimensions in which the rectangle has non-zero width
    pub fn axes_indices(&self) -> (usize, usize) {
        match self.axes{
            RectAxes::XY => (0,1),
            RectAxes::XZ => (0,2),
            RectAxes::YZ => (1,2)
        }
    }

    /// Returns the index of the dimension in which the rectangle has zero-width
    pub fn unused_axis_index(&self) -> usize {
        match self.axes{
            RectAxes::XY => 2,
            RectAxes::XZ => 1,
            RectAxes::YZ => 0,
        }
    }


    /// Returns the lines bounding the rectangle
    pub fn get_lines(&self) -> [Line3; 4] {
        let mut lines: [Line3; 4] = Default::default();
        let mut corners: [Point3<f64>; 4] = Default::default();
        let indices = self.axes_indices();
        let normal_axis = self.unused_axis_index();

        //(min, min)
        corners[0][indices.0] = self.corner(0);
        corners[0][indices.1] = self.corner(2);
        corners[0][normal_axis] = self.k;

        //(max, min)
        corners[2][indices.0] = self.corner(1);
        corners[2][indices.1] = self.corner(2);
        corners[2][normal_axis] = self.k;

        //(min, max)
        corners[1][indices.0] = self.corner(0);
        corners[1][indices.1] = self.corner(3);
        corners[1][normal_axis] = self.k;

        //(max, max)
        corners[3][indices.0] = self.corner(1);
        corners[3][indices.1] = self.corner(3);
        corners[3][normal_axis] = self.k;

        lines[0].points[0] = corners[0];
        lines[0].points[1] = corners[1];

        lines[1].points[0] = corners[0];
        lines[1].points[1] = corners[2];

        lines[2].points[0] = corners[3];
        lines[2].points[1] = corners[1];

        lines[3].points[0] = corners[3];
        lines[3].points[1] = corners[2];

        lines
    }

    /// Returns the normal to the rectangle
    pub fn outward_normal(&self) -> Vector3<f64> {
        match self.axes{
            RectAxes::XY => Vector3::<f64>::new(0.0, 0.0, 1.0),
            RectAxes::XZ => Vector3::<f64>::new(0.0, 1.0, 0.0),
            RectAxes::YZ => Vector3::<f64>::new(1.0, 0.0, 0.0),
        }
    }

    pub fn corner(&self, index: usize) -> f64{
        self.corners[index]
    }
}

impl Hit for Rect {
    fn hit(&self, r:&Ray, t_min: f64, t_max: f64) -> Option<(HitRecord, &Material)> {
        let indices = self.axes_indices();
        let unused = self.unused_axis_index();

        let t = (self.k-r.origin()[unused]) / r.direction()[unused];
        if t.is_nan() || t < t_min || t > t_max{
            return None;
        }
        let x = r.origin()[indices.0] + t*r.direction().index(indices.0);
        let y = r.origin()[indices.1] + t*r.direction().index(indices.1);
        if x < self.corner(0) || x > self.corner(1) || y < self.corner(2) || y > self.corner(3){
            return None;
        }
        let rec = HitRecord::new(r.at(t), self.outward_normal(), t, *r, Vector3::<f64>::default());
        Some((rec, &self.mat))
    }

    fn bounding_box(&self) -> Option<Aabb>{
        //The bounding box must have a non-zero width in each dimension,
        //so pad the missing dimension a small amount
        match self.axes{
            RectAxes::XY => Some(Aabb::new(Point3::<f64>::new(self.corner(0),self.corner(2), self.k-0.0001), Point3::<f64>::new(self.corner(1), self.corner(3), self.k+0.0001))),
            RectAxes::XZ => Some(Aabb::new(Point3::<f64>::new(self.corner(0),self.k-0.0001, self.corner(2)) , Point3::<f64>::new(self.corner(1),self.k+0.0001, self.corner(3)))),
            RectAxes::YZ => Some(Aabb::new(Point3::<f64>::new(self.k-0.0001, self.corner(0),self.corner(2)), Point3::<f64>::new(self.k+0.0001, self.corner(1), self.corner(3))))
        }
    }
}

impl Rasterize for Rect {
    fn outline(&self, cam: &Camera) -> Option<Vec<[usize; 2]>>{
        let lines = self.get_lines().to_vec();
        lines.outline(cam)
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_new(){
    }

    #[test]
    fn test_hit(){

        //XY
        let diff_light = Material::new_diffuse_light(Color::new(4.0,4.0,4.0));
        let rect = Box::new(Rect::new(RectAxes::XY, 3.0, 5.0, 1.0, 3.0, 0.0, diff_light));

        //Case 1: Collision
        let r = Ray::new(Point3::<f64>::new(4.0, 2.0, -10.0), Vector3::<f64>::new( 0.0, 0.0, 1.0));
        let rec_option = rect.hit(&r, 0.0, 100.0);
        assert!(rec_option.is_some());
        let (rec, _) = rec_option.unwrap();
        assert_eq!(rec.t, 10.0);

        //Case 2: Miss face of rectangle
        let r = Ray::new(Point3::<f64>::new(5.01, 2.0, -10.0), Vector3::<f64>::new( 0.0, 0.0, 1.0));
        let rec_option = rect.hit(&r, 0.0, 100.0);
        assert!(rec_option.is_none());

        //Case 3: Miss (due to timeout)
        let r = Ray::new(Point3::<f64>::new(4.0, 2.0, -10.0), Vector3::<f64>::new( 0.0, 0.0, 1.0));
        let rec_option = rect.hit(&r, 0.0, 9.99);
        assert!(rec_option.is_none());

        //Case 4: Miss on infinitely thin edge
        let r = Ray::new(Point3::<f64>::new(0.0, 2.0, 0.0), Vector3::<f64>::new( 1.0, 0.0, 1.0));
        let rec_option = rect.hit(&r, 0.0, 100.0);
        assert!(rec_option.is_none());

        //XZ
        let rect = Box::new(Rect::new(RectAxes::XZ, 3.0, 5.0, 1.0, 3.0, 0.0, diff_light));

        //Case 1: Collision
        let r = Ray::new(Point3::<f64>::new(4.0, -10.0, 2.0), Vector3::<f64>::new( 0.0, 1.0, 0.0));
        let rec_option = rect.hit(&r, 0.0, 100.0);
        assert!(rec_option.is_some());
        let (rec, _) = rec_option.unwrap();
        assert_eq!(rec.t, 10.0);

        //Case 2: Miss face of rectangle
        let r = Ray::new(Point3::<f64>::new(5.01, -10.0, 2.0), Vector3::<f64>::new( 0.0, 1.0, 0.0));
        let rec_option = rect.hit(&r, 0.0, 100.0);
        assert!(rec_option.is_none());

        //Case 3: Miss (due to timeout)
        let r = Ray::new(Point3::<f64>::new(4.0, -10.0, 2.0), Vector3::<f64>::new( 0.0, 1.0, 0.0));
        let rec_option = rect.hit(&r, 0.0, 9.99);
        assert!(rec_option.is_none());

        //Case 4: Miss on infinitely thin edge
        let r = Ray::new(Point3::<f64>::new(0.0, 0.0, 2.0), Vector3::<f64>::new( 1.0, 0.0, 0.0));
        let rec_option = rect.hit(&r, 0.0, 100.0);
        assert!(rec_option.is_none());

        //YZ
        let rect = Box::new(Rect::new(RectAxes::YZ, 3.0, 5.0, 1.0, 3.0, 0.0, diff_light));

        //Case 1: Collision
        let r = Ray::new(Point3::<f64>::new(-10.0, 4.0, 2.0), Vector3::<f64>::new( 1.0, 0.0, 0.0));
        let rec_option = rect.hit(&r, 0.0, 100.0);
        assert!(rec_option.is_some());
        let (rec, _) = rec_option.unwrap();
        assert_eq!(rec.t, 10.0);

        //Case 2: Miss face of rectangle
        let r = Ray::new(Point3::<f64>::new(-10.0, 5.01, 2.0), Vector3::<f64>::new( 1.0, 0.0, 0.0));
        let rec_option = rect.hit(&r, 0.0, 100.0);
        assert!(rec_option.is_none());

        //Case 3: Miss (due to timeout)
        let r = Ray::new(Point3::<f64>::new(4.0, -10.0, 2.0), Vector3::<f64>::new( 0.0, 1.0, 0.0));
        let rec_option = rect.hit(&r, 0.0, 9.99);
        assert!(rec_option.is_none());

        //Case 4: Miss on infinitely thin edge
        let r = Ray::new(Point3::<f64>::new(0.0, 0.0, 2.0), Vector3::<f64>::new( 0.0, 1.0, 0.0));
        let rec_option = rect.hit(&r, 0.0, 100.0);
        assert!(rec_option.is_none());


    }

    #[test]
    fn test_bounding_box(){
        //XY
        let diff_light = Material::new_diffuse_light(Color::new(4.0,4.0,4.0));
        let rect = Box::new(Rect::new(RectAxes::XY, -5.0, -3.0, 1.0, 3.0, 0.0, diff_light));
        let bb = rect.bounding_box();
        assert!(bb.is_some());
        let bb = bb.unwrap();
        assert_eq!(bb.min(), Point3::<f64>::new(-5.0, 1.0, -0.0001));
        assert_eq!(bb.max(), Point3::<f64>::new(-3.0, 3.0, 0.0001));

        //XZ
        let rect = Box::new(Rect::new(RectAxes::XZ, -5.0, -3.0, 1.0, 3.0, 0.0, diff_light));
        let bb = rect.bounding_box();
        assert!(bb.is_some());
        let bb = bb.unwrap();
        assert_eq!(bb.min(), Point3::<f64>::new(-5.0, -0.0001, 1.0));
        assert_eq!(bb.max(), Point3::<f64>::new(-3.0, 0.0001, 3.0));

        //YZ
        let rect = Box::new(Rect::new(RectAxes::YZ, -5.0, -3.0, 1.0, 3.0, 0.0, diff_light));
        let bb = rect.bounding_box();
        assert!(bb.is_some());
        let bb = bb.unwrap();
        assert_eq!(bb.min(), Point3::<f64>::new(-0.0001, -5.0, 1.0));
        assert_eq!(bb.max(), Point3::<f64>::new(0.0001, -3.0, 3.0));
    }
}