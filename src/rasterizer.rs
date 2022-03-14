use std::ops::Index;

use crate::vec::{Vec2, Vec3, Point3};
use crate::camera::*;

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

    pub fn bresenham(&self) -> Vec<[usize; 2]> {
        let line_start = (self.start().x().round() as isize, self.start().y().round() as isize);
        let line_end = (self.end().x().round() as isize, self.end().y().round() as isize);
        bresenham::Bresenham::new(line_start, line_end).map(|(x,y)|[x as usize, y as usize]).collect()
    }
}

impl Index<usize> for Line2 {
    type Output = Vec2;
    fn index(&self, index: usize) -> &Self::Output {
        &self.points[index]
    }
}

#[derive (PartialEq, Debug, Copy, Clone, Default)]
pub struct Line3{
    points: [Vec3; 2]
}

impl Line3{
    pub fn new(start: Vec3, end: Vec3) -> Line3 {
        Line3 {points: [start, end]}
    }

    pub fn project(&self,  cam: &Camera) -> Option<Line2> {
        let mut points: [Vec2; 2] = Default::default();
        let normal = cam.orientation.w;

        //Project on to the infinite plane first
        for i in 0..2 as usize{
            let dir = self[i] - cam.origin;
            let t = (cam.lower_left_corner - cam.origin).dot(normal)/ dir.dot(normal);
            let projection_3d = cam.origin + dir * t;
            let relative_point = projection_3d - cam.lower_left_corner;
            points[i] = Vec2::new(relative_point.dot(cam.orientation.v), relative_point.dot(cam.orientation.u));
        }

        //Clamp the line to the bounded camera plane
        let start = true;
        let mut bounded_points: [Vec2; 2] = Default::default();
        for i in 0..2 {
            let other_point = points[start as usize];
            let delta: Vec2 = other_point - points[i];
            let length = delta.length();
            let gradient = delta.unit_vector();
            let mut t = f64::INFINITY;
            if points[i].x() < 0.0 {
                let t_temp =  - points[i][0] / gradient[0];
                if t_temp > 0.0 && t_temp < t && t_temp < length {
                    t = t_temp;
                }
            }

            if points[i].x() > cam.horizontal[0] {
                let t_temp =  (cam.horizontal[0]- points[i][0]) / gradient[0];
                if t_temp > 0.0 && t_temp < t && t_temp < length {
                    t = t_temp;
                }
            }

            if points[i].y() < 0.0 {
                let t_temp =  - points[i][1] / gradient[1];
                if t_temp > 0.0 && t_temp < length && t_temp < t {
                    t = t_temp;
                }
            }

            if points[i].y() > cam.vertical[1] {
                let t_temp =  (cam.vertical[0]- points[i][1]) / gradient[1];
                if t_temp > 0.0 && t_temp < length && t_temp < t {
                    t = t_temp;
                }
            }

            if t < f64::INFINITY {
                bounded_points[i] = other_point + t * gradient;
            }
            else {
                bounded_points[i] = points[i];
            }
        }

        //Check the points now lie within the bounds of the rectangle
        let mut point_0_out_of_bounds = false;
        let mut point_1_out_of_bounds = false;

        if (bounded_points[0][0] < 0.0  || bounded_points[0][0] > cam.horizontal[0]) && (bounded_points[0][1] < 0.0  || bounded_points[0][1] > cam.vertical[1]) {
            point_0_out_of_bounds = true;
        }

        if (bounded_points[1][0] < 0.0  || bounded_points[1][0] > cam.horizontal[0]) && (bounded_points[1][1] < 0.0  || bounded_points[1][1] > cam.vertical[1]) {
            point_1_out_of_bounds = true;
        }

        if point_0_out_of_bounds && point_1_out_of_bounds {
            return None
        } 
        else {
            Some(Line2{points: bounded_points})
        }

    }
}

impl Index<usize> for Line3 {
    type Output = Vec3;
    fn index(&self, index: usize) -> &Self::Output {
        &self.points[index]
    }
}

impl WireFrame for Line3 {
    fn draw_wireframe(&self, cam: &Camera) -> Option<Vec<[usize; 2]>>
    {
        if let Some(projected_line) = self.project(cam) {
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
        let projected_line = line.project(&cam).unwrap();
        assert!((projected_line[0] - Vec2::new(11.0, 11.0)).length() < 0.00001);
        assert!((projected_line[1] - Vec2::new(15.0, 14.0)).length() < 0.00001);
    }
}