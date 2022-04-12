use core::time;
use std::ops::{Index, IndexMut, RangeBounds};

use crate::enum_dispatch::*;
use crate::raytracing::{Ray, RayPlaneIntersection};
use crate::plane::*;
use line_drawing::Bresenham;

use crate::vec::{Vec2, Vec3};
use crate::camera::{Camera, CameraSettings, Orientation};
use crate::primitives::{GeometricPrimitive, Primitive};
use crate::rasterizing::*;
use crate::points::{Point2, Point3};

pub type OutCode = i8;

#[derive (PartialEq, Debug, Copy, Clone, Default)]
pub struct Line2 {
    pub points: [Vec2; 2]
}

impl Line2 {
    pub fn new(start: Vec2, end: Vec2) -> Line2 {
        Line2 {points: [start, end]}
    }

    pub fn start(&self) -> Vec2 {
        self[0]
    }

    pub fn end(&self) -> Vec2 {
        self[1]
    }

    pub fn length(&self) -> f64 {
        (self[1] - self[0]).length()
    }

    pub fn scale(&self, scale: f64) -> Line2 {
        Line2::new(self.points[0] * scale, self.points[1] * scale)
    }
        
    // Cohen–Sutherland clipping algorithm clips a line from against a rectangle with 
    // diagonal from (min_x, min_y) to (max_x, max_y).
    pub fn clip(&self, min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> Option<Line2> {

        let left = 1;   // 0001
        let right = 2;  // 0010
        let bottom = 4; // 0100
        let top = 8;    // 1000

        let mut point_0 = self[0];
        let mut point_1 = self[1];
        // compute outcodes for P0, P1, and whatever point lies outside the clip rectangle
        let mut outcode_0 = point_0.compute_outcode(min_x, max_x, min_y, max_y);
        let mut outcode_1 = point_1.compute_outcode(min_x, max_x, min_y, max_y);

        loop {
            if outcode_0 == 0 && outcode_1 == 0 {
                // bitwise OR is 0: both points inside window; trivially accept and exit loop
                return Some(Line2::new(point_0, point_1));
            } else if (outcode_0 & outcode_1) != 0 {
                // bitwise AND is not 0: both points share an outside zone (LEFT, RIGHT, TOP,
                // or BOTTOM), so both must be outside window; exit loop (accept is false)
                return None;
            } else {
                // failed both tests, so calculate the line segment to clip
                // from an outside point to an intersection with clip edge
                let mut x = 0f64;
                let mut y = 0f64;

                // At least one endpoint is outside the clip rectangle; pick it.
                let outcode_out: OutCode;
                if outcode_1 > outcode_0 {
                    outcode_out = outcode_1;
                }
                else {
                    outcode_out = outcode_0;
                }

                // Now find the intersection point;
                // use formulas:
                //   slope = (y1 - y0) / (x1 - x0)
                //   x = x0 + (1 / slope) * (ym - y0), where ym is min_y or max_y
                //   y = y0 + slope * (xm - x0), where xm is min_x or max_x
                // No need to worry about divide-by-zero because, in each case, the
                // outcode bit being tested guarantees the denominator is non-zero
                if (outcode_out & top) != 0 {           // point is above the clip window
                    x = point_0.x() + (point_1.x() - point_0.x()) * (max_y - point_0.y()) / (point_1.y() - point_0.y());
                    y = max_y;
                } else if (outcode_out & bottom) != 0 { // point is below the clip window
                    x = point_0.x() + (point_1.x() - self[0].x()) * (min_y - point_0.y()) / (point_1.y() - point_0.y());
                    y = min_y;
                } else if (outcode_out & right) != 0 {  // point is to the right of clip window
                    y = point_0.y() + (point_1.y() - point_0.y()) * (max_x - self[0].x()) / (point_1.x() - self[0].x());
                    x = max_x;
                } else if (outcode_out & left) != 0 {   // point is to the left of clip window
                    y = point_0.y() + (point_1.y() - point_0.y()) * (min_x - self[0].x()) / (point_1.x() - self[0].x());
                    x = min_x;
                }

                // Now we move outside point to intersection point to clip
                // and get ready for next pass.
                if outcode_out == outcode_0 {
                    point_0[0] = x;
                    point_0[1] = y;
                    outcode_0 = point_0.compute_outcode(min_x, max_x, min_y, max_y);
                } else {
                    point_1[0] = x;
                    point_1[1] = y;
                    outcode_1 = point_1.compute_outcode(min_x, max_x, min_y, max_y);
                }
            }
        }
    }

    pub fn bresenham(&self) -> Vec<[usize; 2]> {
        let line_start = (self.start().x().round() as isize, self.start().y().round() as isize);
        let line_end = (self.end().x().round() as isize, self.end().y().round() as isize);
        Bresenham::new(line_start, line_end).map(|(x,y)|[x as usize, y as usize]).collect()
    }
}

impl Index<usize> for Line2 {
    type Output = Vec2;
    fn index(&self, index: usize) -> &Self::Output {
        &self.points[index]
    }
}

impl IndexMut<usize> for Line2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.points[index]
    }
}

#[derive (PartialEq, Debug, Copy, Clone)]

pub enum LinePlaneIntersection {
    Line(Line3),
    Point(Point3),
    None
}

#[derive (PartialEq, Debug, Copy, Clone, Default)]
pub struct Line3{
    pub points: [Vec3; 2]
}

impl Line3{
    pub fn new(start: Vec3, end: Vec3) -> Line3 {
        Line3 {points: [start, end]}
    }

    /// Returns the length of the line.
    pub fn length(&self) -> f64 {
        (self[1] - self[0]).length()
    }

    /// Returns the direction of the line as a vector.
    pub fn dir(&self) -> Vec3 {
        self[1] - self[0]
    }

    /// Scales the magnitude of the line by a scalar. Both ends of the
    /// line translate.
    pub fn scale(&self, scale: f64) -> Line3 {
        Line3::new(self.points[0] * scale, self.points[1] * scale)
    }

    pub fn unbounded_projection(&self, plane: Plane, camera_origin: Point3) -> Line2 {
        let mut projected_line = Line2::default();
        for i in 0..2 as usize{
            let ray = Ray::new(camera_origin, self.points[i] - camera_origin);
            if let RayPlaneIntersection::Point(projection_3d) = ray.plane_intersection(plane) {
                let relative_point = projection_3d - plane.origin;
                projected_line[i] = Vec2::new(relative_point.dot(plane.orientation.u), relative_point.dot(plane.orientation.v));
            } 
        }       
        projected_line
    }

    pub fn plane_intersection(&self, plane: Plane) -> LinePlaneIntersection {
        let dir = self[1] - self[0];
        let plane_normal = plane.orientation.w;

        //Check if line is parallel to plane
        if dir.dot(plane_normal) == 0.0 {
            //If so, check if the line lies in the plane.
            if (plane.origin - self[0]).dot(plane_normal) == 0.0 {
                return LinePlaneIntersection::Line(*self)
            } else {
                return LinePlaneIntersection::None
            }
        } 

        //Check if the intersection point lies within the bounds of the line
        let time_of_intersection = (plane.origin - self[0]).dot(plane_normal) / (dir.dot(plane_normal));
        if time_of_intersection < 0.0 || time_of_intersection > self.length()  {
            return LinePlaneIntersection::None
        }
        let intersection_point = self[0] + time_of_intersection * dir;
        LinePlaneIntersection::Point(intersection_point)
    }

    pub fn visible(&self, plane: Plane, camera_origin: Point3) -> Option<Line3> {
        let mut visible_line = self.clone();

        //Check if the line lies behind the camera
        let dividing_plane = Plane::new(plane.orientation, camera_origin);
        if !visible_line[0].is_in_front(dividing_plane) && !visible_line[1].is_in_front(dividing_plane) {
            return None
        }

        //If the line partially lies behind the camera, reduce it to to the portion which is visible.
        let intersection = self.plane_intersection(dividing_plane);
        match intersection {
            LinePlaneIntersection::Point(intersection_point) => {
                //Find the point which is out of view
                    for i in 0..2 {
                    if !self[i].is_in_front(dividing_plane) {
                        visible_line[i] = intersection_point;
                        break;
                    }
                }
            }
            _ => return Some(*self)
        }
        Some(visible_line)
    }


    pub fn project(&self, plane: Plane, camera_origin: Point3) -> Option<Line2> {
        
        let mut points: [Vec2; 2] = Default::default();
        let mut visible_line = self.clone();

        //Check if the line lies behind the camera
        let dividing_plane = Plane::new(plane.orientation, camera_origin);
        if !visible_line[0].is_in_front(dividing_plane) && !visible_line[1].is_in_front(dividing_plane) {
            return None
        }

        //If the line partially lies behind the camera, reduce it to to the portion which is visible.
        let intersection = self.plane_intersection(dividing_plane);
        match intersection {
            LinePlaneIntersection::Point(intersection_point) => {
                //Find the point which is out of view
                    for i in 0..2 {
                    if !self[i].is_in_front(dividing_plane) {
                        visible_line[i] = intersection_point;
                        break;
                    }
                }
            }
            _ => {}
        }
        
        //Project the remaining line onto the viewing plane
        for i in 0..2 as usize{
            let ray = Ray::new(camera_origin, self.points[i] - camera_origin);
            if let RayPlaneIntersection::Point(projection_3d) = ray.plane_intersection(plane) {
                let relative_point = projection_3d - plane.origin;
                points[i] = Vec2::new(relative_point.dot(plane.orientation.u), relative_point.dot(plane.orientation.v));
            } 
        }        

    Some(Line2::new(points[0], points[1]))
    }
}

impl Index<usize> for Line3 {
    type Output = Vec3;
    fn index(&self, index: usize) -> &Self::Output {
        &self.points[index]
    }
}
impl IndexMut<usize> for Line3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.points[index]
    }
}

impl Rasterize for Line3 {
    fn outline(&self, cam: &Camera) -> Option<Vec<[usize; 2]>>
    {
        if let Some(projected_line) = self.project(Plane::new(cam.orientation, cam.lower_left_corner), cam.origin) {
            let scale = cam.resoloution.1 as f64/ cam.vertical.length() as f64;
            if let Some(clipped_line) = projected_line.clip(0.0, cam.horizontal.length() - 1.0 / scale, 0.0, cam.vertical.length() - 1.0 / scale) {
                return Some(clipped_line.scale(scale).bresenham());
            }
        }
        None
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use core::f64;
    use std::f64::consts::PI;


    #[test]
    fn test_project() {

        //Case 1: Axis-aligned camera
        let v_up = Vec3::new(0.0, 1.0, 0.0);
        let look_from = Point3::new(0.0, 0.0, 0.0);
        let look_at = Point3::new(10.0, 0.0, 0.0);
        let v_fov = 2.0 * 10f64.atan() * 180.0 / PI;
        let focus_dist = 1.0;
        let aperture = 0.0;
        let aspect_ratio = 1.0;
        let image_height = 100;
        let image_width = 100;
        let camera_settings = CameraSettings { look_from, look_at, v_up, v_fov, aspect_ratio, aperture, focus_dist, image_height, image_width };
        let cam = Camera::new(camera_settings);
        
        let line = Line3::new(Point3::new(1.0, 1.0, 1.0), Point3::new(1.0, 5.0, 4.0));
        let projected_line = line.project(Plane::new(cam.orientation, cam.lower_left_corner), cam.origin).unwrap();
        assert!((projected_line[0] - Vec2::new(11.0, 11.0)).length() < 0.00001);
        assert!((projected_line[1] - Vec2::new(15.0, 14.0)).length() < 0.00001);
    }

    #[test]
    fn test_bresenham() {
        let line = Line2::new(Point2::new(0.0, 0.0), Point2::new(10.0, 0.0));
        let pixels = line.bresenham();
        
        assert_eq!(pixels.len(), 11);
        for i in 0..11 as usize {
            assert_eq!(pixels[i], [i, 0]);
        }
    }

    #[test]
    fn test_clip() {

        //Case 1: Bottom left corner
        let start = Point2::new(-1.0, -1.0);
        let end = Point2::new(5.0, 5.0);
        let line = Line2::new(start, end);
        let clipped_line =line.clip(0.0, 10.0, 0.0, 10.0).unwrap();
        assert_eq!(clipped_line.points[0], Point2::new(0.0, 0.0));

        
        //Case 2: Left-hand side
        let start = Point2::new(-1.0, 5.0);
        let end = Point2::new(5.0, 5.0);
        let line = Line2::new(start, end);
        let clipped_line =line.clip(0.0, 10.0, 0.0, 10.0).unwrap();
        assert_eq!(clipped_line.points[0], Point2::new(0.0, 5.0));

                
        //Case 3: Top
        let start = Point2::new(5.0, 15.0);
        let end = Point2::new(5.0, 5.0);
        let line = Line2::new(start, end);
        let clipped_line =line.clip(0.0, 10.0, 0.0, 10.0).unwrap();
        assert_eq!(clipped_line.points[0], Point2::new(5.0, 10.0));

        //Case 4: right-hand side
        let start = Point2::new(15.0, 5.0);
        let end = Point2::new(5.0, 5.0);
        let line = Line2::new(start, end);
        let clipped_line =line.clip(0.0, 10.0, 0.0, 10.0).unwrap();
        assert_eq!(clipped_line.points[0], Point2::new(10.0, 5.0));

        //Case 5: Bottom
        let start = Point2::new(5.0, -5.0);
        let end = Point2::new(5.0, 5.0);
        let line = Line2::new(start, end);
        let clipped_line =line.clip(0.0, 10.0, 0.0, 10.0).unwrap();
        assert_eq!(clipped_line.points[0], Point2::new(5.0, 0.0));
    }
}