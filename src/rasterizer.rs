use core::time;
use std::ops::{Index, IndexMut};

use line_drawing::Bresenham;

use crate::vec::{Vec2, Vec3, Point3, Point2};
use crate::camera::{Camera, CameraSettings, Orientation};


pub type OutCode = i8;

#[derive (PartialEq, Debug, Copy, Clone, Default)]
pub struct Plane {
    orientation: Orientation,
    origin: Point3
}

impl Plane {
    pub fn new(orientation: Orientation, origin: Point3) -> Plane {
        Plane { orientation, origin }
    }
}
#[derive (PartialEq, Debug, Copy, Clone, Default)]
pub struct Line2 {
    points: [Vec2; 2]
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
            if !(outcode_0 | outcode_1) != 0 {
                // bitwise OR is 0: both points inside window; trivially accept and exit loop
                return Some(Line2::new(point_0, point_1));
            } else if outcode_0 & outcode_1 != 0 {
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

#[derive (PartialEq, Debug, Copy, Clone)]

pub enum LinePlaneIntersection {
    Line(Line3),
    Point(Point3)
}

#[derive (PartialEq, Debug, Copy, Clone, Default)]
pub struct Line3{
    pub points: [Vec3; 2]
}

impl Line3{
    pub fn new(start: Vec3, end: Vec3) -> Line3 {
        Line3 {points: [start, end]}
    }

    pub fn length(&self) -> f64 {
        (self[1] - self[0]).length()
    }

    pub fn plane_intersection(&self, plane: Plane) -> Option<LinePlaneIntersection> {
        let dir = self[1] - self[0];
        let plane_normal = plane.orientation.w;

        //Check if line is parallel to plane
        if dir.dot(plane_normal) == 0.0 {
            //If so, check if the line lies in the plane.
            if (plane.origin - self[0]).dot(plane_normal) == 0.0 {
                return Some(LinePlaneIntersection::Line(*self))
            } else {
                return None
            }
        } 

        //Check if the intersection point lies within the bounds of the line
        let time_of_intersection = (plane.origin - self[0]).dot(plane_normal) / (dir.dot(plane_normal));
        if time_of_intersection > self.length() {
            return None
        }
        let intersection_point = self[0] + time_of_intersection * dir;
        Some(LinePlaneIntersection::Point(intersection_point))
    }

    pub fn project(&self, plane: Plane, camera_origin: Point3) -> Option<Line2> {
        let mut points: [Vec2; 2] = Default::default();
        let normal = plane.orientation.w;
        let mut visible_line = self.clone();

        //Check if the line intersects the plane. If so, reduce it to the portion which is visible.
        if let Some(intersection) = self.plane_intersection(plane){
            match intersection {
                LinePlaneIntersection::Point(intersection_point) => {
                    //Find the point which is out of view
                    for i in 0..2 {
                        let line = Line3::new(self[i], camera_origin);
                        if line.plane_intersection(plane).is_none() {
                            visible_line[i] = intersection_point;
                            break;
                        }
                    }

                }
                _ => {}
            }
        }
        
        //Project the remaining line on the plane
        for i in 0..2 as usize{
            let dir = self[i] - camera_origin;
            let t = (plane.origin - camera_origin).dot(normal)/ dir.dot(normal);
            let projection_3d = camera_origin + dir * t;
            let relative_point = projection_3d - plane.origin;
            points[i] = Vec2::new(relative_point.dot(plane.orientation.v), relative_point.dot(plane.orientation.u));
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

impl WireFrame for Line3 {
    fn draw_wireframe(&self, cam: &Camera) -> Option<Vec<[usize; 2]>>
    {
        if let Some(projected_line) = self.project(Plane::new(cam.orientation, cam.lower_left_corner), cam.origin) {
            Some(projected_line.bresenham())
        }
        else {
            return None
        }
    }
}

pub trait WireFrame{
    fn draw_wireframe(&self, cam: &Camera) -> Option<Vec<[usize; 2]>>;
}

impl<W> WireFrame for Vec<W> where W: WireFrame {
    fn draw_wireframe(&self, cam: &Camera) -> Option<Vec<[usize; 2]>> {
        let mut pixels: Vec<[usize; 2]> = Default::default();
        for object in self {
            if let Some(new_pixels) = object.draw_wireframe(cam) {
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
        let camera_settings = CameraSettings { look_from, look_at, v_up, v_fov, aspect_ratio, aperture, focus_dist };
        let cam = Camera::new(camera_settings);
        
        let line = Line3::new(Point3::new(1.0, 1.0, 1.0), Point3::new(1.0, 5.0, 4.0));
        let projected_line = line.project(Plane::new(cam.orientation, cam.lower_left_corner), cam.origin).unwrap();
        assert!((projected_line[0] - Vec2::new(11.0, 11.0)).length() < 0.00001);
        assert!((projected_line[1] - Vec2::new(15.0, 14.0)).length() < 0.00001);
    }
}