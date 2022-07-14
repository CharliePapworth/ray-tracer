use nalgebra::{Point3, Point2};
use super::{plane::Plane, lines::OutCode};

pub trait Point3ExtensionMethods {
    fn swap(&mut self, i: usize, j: usize);
    fn is_in_front(self, plane: Plane) -> bool;
    fn distance_to_plane(self, plane: Plane) -> f32;
    fn is_on_the_side_of(self, plane: Plane, other: Point3<f32>)  -> bool;
}

pub trait Point2ExtensionMethods {
    fn compute_outcode(&self, min_x: f32, max_x: f32, min_y: f32, max_y: f32) -> OutCode;
}

impl Point3ExtensionMethods for Point3<f32> {
    fn swap(&mut self, i: usize, j: usize) {
        let temp = self[j];
        self[j] = self[i];
        self[i] = temp;
    }

    fn is_in_front(self, plane: Plane) -> bool {
        let (_, _, _, d) = plane.get_coefficients();
        plane.orientation.w.dot(&self.coords) + d <= 0.0
    } 

    fn distance_to_plane(self, plane: Plane) -> f32 {
        let (_, _, _, d) = plane.get_coefficients();
        let normal = plane.orientation.w;
        (self.coords.dot(&normal) + d).abs() / normal.norm()
    }

    fn is_on_the_side_of(self, plane: Plane, other: Point3<f32>)  -> bool {
        let (_, _, _, d) = plane.get_coefficients();
        plane.orientation.w.dot(&self.coords) == plane.orientation.w.dot(&other.coords)
    }

    
}

impl Point2ExtensionMethods for Point2<f32> {
    fn compute_outcode(&self, min_x: f32, max_x: f32, min_y: f32, max_y: f32) -> OutCode { 
        let inside = 0; // 0000
        let left = 1;   // 0001
        let right = 2;  // 0010
        let bottom = 4; // 0100
        let top = 8;    // 1000

        let mut code = inside; // initialised as being inside of [[clip window]]

        if self[0] < min_x { // to the left of clip window
            code |= left;
        } 
        else if self[0] > max_x {  // to the right of clip window
            code |= right; 
        }   
            
        if self[1] < min_y {      // below the clip window
            code |= bottom;
        }          
            
        else if self[1] > max_y { // above the clip window
            code |= top;
            }     
        code
    }
}