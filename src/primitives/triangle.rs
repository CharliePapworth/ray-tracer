use crate::camera::Camera;
use crate::geometry::lines::*;
use crate::geometry::points::Point3ExtensionMethods;
use crate::nalgebra::{Vector3, Point3};
use crate::primitives::bvh::*;
use crate::material::*;
use crate::util::*;
use crate::rasterizing::*;
use crate::raytracing::{HitRecord, Hit, Ray};

#[derive (Clone)]
pub struct Triangle {
    vertices: [Point3::<f32>; 3],
    normals: [Vector3<f32>; 3],
    material: Material
}

impl Triangle{

    pub fn new(vertices: [Point3::<f32>; 3], normals: [Vector3<f32>;3], mat: Material) -> Triangle{
        Triangle{vertices, normals, material: mat}
    }

    ///Returns the vertex corresponding to the index.
    /// 
    ///The index must be in the range [0, 3)
    pub fn get_vertex(&self, index: usize) -> Point3::<f32>{
        self.vertices[index]
    }

    ///Determine where (0,0) lies with respect to the 
    ///oriented line connecting p0 to p1.
    ///
    ///If e0 < 0, the point lies to the left of the line. If e0 > 0, the
    ///point lies to the right of the line. If e0 = 0, the point lies on the line
    pub fn edge_fn(p0: Point3::<f32>, p1: Point3::<f32>) -> f32{
        p0[0] * p1[1] - p0[1] * p1[0]
    }

    /// Shears the x and y dimensions of the triangle
    fn shear_xy(&mut self, r: &Ray){
        let sx = -r.direction()[0]/ r.direction()[2];
        let sy = -r.direction()[1]/r.direction()[2];

        for i in 0..3{
            self.vertices[i] = Point3::<f32>::new(self.get_vertex(i)[0] + sx * self.get_vertex(i)[2],
                                           self.get_vertex(i)[1] + sy * self.get_vertex(i)[2],
                                            self.get_vertex(i)[2]);
        }

    } 

    ///Shears the z-dimension of the triangle
    fn shear_z(&mut self, r: &Ray){
        let sz = 1.0/r.direction()[2];
        self.vertices[0][2] *= sz;
        self.vertices[1][2] *= sz;
        self.vertices[2][2] *= sz;
    }
}

impl Hit for Triangle {
    fn hit(&self ,r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>{

        let mut rc = *r;
        rc.dir = r.dir / r.dir.norm();
        let mut t = *self;
        let origin = Point3::<f32>::new(0.0, 0.0, 0.0);

        //Translate vertices
        t.vertices[0] = origin + (t.vertices[0] - rc.origin());
        t.vertices[1] = origin + (t.vertices[1] - rc.origin());
        t.vertices[2] = origin + (t.vertices[2] - rc.origin());

        //swap dimensions
        let max_dim = r.direction().iamax();
        if max_dim < 2 {
            t.vertices[0].swap(max_dim, 2);
            t.vertices[1].swap(max_dim, 2);
            t.vertices[2].swap(max_dim, 2);
            rc.dir.swap_rows(max_dim, 2);
        }

        //Only shear the (x,y) coordinates to minimise computations
        t.shear_xy(&rc);

        //Call edge function on all three sides
        let e0 = Triangle::edge_fn(t.vertices[1], t.vertices[2]);
        let e1 = Triangle::edge_fn(t.vertices[2], t.vertices[0]);
        let e2 = Triangle::edge_fn(t.vertices[0], t.vertices[1]);

        //Check for miss
        if (e0 < 0.0 || e1 < 0.0 || e2 < 0.0) && (e0 > 0.0 || e1 > 0.0 || e2 > 0.0){
            return None;
        }

        // //Check for collision on triangle edge
        let det = e0 + e1 + e2;
        if det == 0.0 {
            return None;
        }

        //Compute scaled hit distance to triangle and test against ray range
        t.shear_z(&rc);
        let t_scaled = e0 * t.vertices[0][2] + e1 * t.vertices[1][2] + e2 * t.vertices[2][2];
        if det < 0.0 && (t_scaled >= t_min * det || t_scaled < t_max * det){
            return None;
        } else if det > 0.0 && (t_scaled <= t_min * det || t_scaled > t_max * det){
            return None;
        }

        //Compute barycentric coordinates and t value for triangle intersection
        let inv_det = 1.0/det;
        let t = t_scaled * inv_det; 
        let b0 = e0 * inv_det;
        let b1 = e1* inv_det;
        let b2 = e2 * inv_det;

        let norm = (b0*self.normals[0] + 
                         b1*self.normals[1] + 
                         b2*self.normals[2]).normalize();


        let x_err = (b0 * self.vertices[0][0]).abs() + (b1 * self.vertices[1][0]).abs() + 
                        (b2 * self.vertices[2][0]).abs();

        let y_err = (b0 * self.vertices[0][1]).abs() + (b1 * self.vertices[1][1]).abs() + 
                        (b2 * self.vertices[2][1]).abs();
                        
        let z_err = (b0 * self.vertices[0][2]).abs() + (b1 * self.vertices[1][2]).abs() + 
                        (b2 * self.vertices[2][2]).abs();                

       let p_err = gamma(7) * Vector3::<f32>::new(x_err, y_err, z_err);
       let p = b0 * self.vertices[0] + b1 * self.vertices[1].coords + b2 * self.vertices[2].coords;
       Some(HitRecord::new(p, norm, self.material, - r.dir, t, *r, p_err))
    }

    fn bounding_box(&self) -> Option<Aabb>{
        let min_x = self.vertices[0][0].min(self.vertices[1][0]).min(self.vertices[2][0]) - 0.001;
        let min_y = self.vertices[0][1].min(self.vertices[1][1]).min(self.vertices[2][1]) - 0.001;
        let min_z = self.vertices[0][2].min(self.vertices[1][2]).min(self.vertices[2][2]) - 0.001;

        let max_x = self.vertices[0][0].max(self.vertices[1][0]).max(self.vertices[2][0]) + 0.001;
        let max_y = self.vertices[0][1].max(self.vertices[1][1]).max(self.vertices[2][1]) + 0.001;
        let max_z = self.vertices[0][2].max(self.vertices[1][2]).max(self.vertices[2][2]) + 0.001;

        Some(Aabb::new(Point3::<f32>::new(min_x, min_y, min_z), Point3::<f32>::new(max_x, max_y, max_z)))
    }
}

impl Rasterize for Triangle {
    fn outline(&self, cam: &Camera) -> Option<Vec<[usize; 2]>> {

        let line_1 = Line3::new(self.vertices[1], self.vertices[0]);
        let line_2 = Line3::new(self.vertices[2],self.vertices[0]);
        let line_3 = Line3::new(self.vertices[2],self.vertices[1]);

        let lines = vec!(line_1, line_2, line_3);
        lines.outline(cam)
    }
}

#[cfg(test)]
mod tests {
    use crate::material::lambertian::Lambertian;

    use super::*;

    #[test]
    fn test_shear_xy(){
        let v0 = Point3::<f32>::new(0.0, 0.0, 0.0);
        let v1 = Point3::<f32>::new(1.0, 2.0, 3.0);
        let v2 = Point3::<f32>::new(0.5, 2.0, 2.0);
        let mat = Material::Lambertian(Lambertian::default());
        let norm = [Vector3::<f32>::new(0.0, -1.0, -1.0).normalize(); 3];
        let mut t = Triangle::new([v0, v1, v2], norm, mat);
        let r = Ray::new(Point3::<f32>::new(0.5, -1.0, 0.5), Vector3::<f32>::new(1.0, 4.0, 1.0));

        t.shear_xy(&r);
        assert_eq!(t.get_vertex(0), Point3::<f32>::new(0.0, 0.0, 0.0));
        assert_eq!(t.get_vertex(1), Point3::<f32>::new(-2.0, -10.0, 3.0));
        assert_eq!(t.get_vertex(2), Point3::<f32>::new(-1.5, -6.0, 2.0));
    }

    #[test]
    fn test_shear_z(){
        let v0 = Point3::<f32>::new(0.0, 0.0, 0.0);
        let v1 = Point3::<f32>::new(1.0, 2.0, 3.0);
        let v2 = Point3::<f32>::new(0.5, 2.0, 2.0);
        let mat = Material::Lambertian(Lambertian::default());
        let norm = [Vector3::<f32>::new(0.0, -1.0, -1.0).normalize(); 3];
        let mut t = Triangle::new([v0, v1, v2], norm, mat);
        let r = Ray::new(Point3::<f32>::new(0.5, -1.0, 0.5), Vector3::<f32>::new(1.0, 4.0, 0.5));

        t.shear_z(&r);
        assert_eq!(t.get_vertex(0), Point3::<f32>::new(0.0, 0.0, 0.0));
        assert_eq!(t.get_vertex(1), Point3::<f32>::new(1.0, 2.0, 6.0));
        assert_eq!(t.get_vertex(2), Point3::<f32>::new(0.5, 2.0, 4.0));
    }

    #[test]
    fn test_hit(){

        //Initialisations
        let mat = Material::Lambertian(Lambertian::default());
        let v0 = Point3::<f32>::new(-2.0, 2.0, 0.0);
        let v1 = Point3::<f32>::new(2.0, 2.0, 0.0);
        let v2 = Point3::<f32>::new(0.0, 4.0, 0.0);
        let norm = [Vector3::<f32>::new(0.0, 0.0, 1.0).normalize(); 3];
        let t = Triangle::new([v0, v1, v2], norm, mat);
        
        //Case 1: Front-facing intersection
        let r = Ray::new(Point3::<f32>::new(0.0, 3.0, 20.0), Vector3::<f32>::new(0.0, 0.0, -1.0));
        let result = t.hit(&r, 0.0, 100.0);
        assert!(result.is_some());
        let rec = result.unwrap();
        assert_eq!(rec.time, 20.0);
        assert_eq!(rec.point_in_scene, Point3::<f32>::new(0.0, 3.0, 0.0));
        assert_eq!(rec.front_face, true);

        //Case 2: Back-facing interection
        let r = Ray::new(Point3::<f32>::new(0.0, 3.0, -20.0), Vector3::<f32>::new(0.0, 0.0, 1.0));
        let result = t.hit(&r, 0.0, 100.0);
        assert!(result.is_some());
        let rec = result.unwrap();
        assert_eq!(rec.time, 20.0);
        assert_eq!(rec.point_in_scene, Point3::<f32>::new(0.0, 3.0, 0.0));
        assert_eq!(rec.front_face, false);

        //Case 3: Edge-on intersection
        let r = Ray::new(Point3::<f32>::new(-10.0, 2.0, 0.0), Vector3::<f32>::new(1.0, 0.0, 0.0));
        let result = t.hit(&r, 0.0, 100.0);
        assert!(result.is_none());

        //Case 4: Edge intersection
        let r = Ray::new(Point3::<f32>::new(0.0, 2.0,10.0), Vector3::<f32>::new(0.0, 0.0, -1.0));
        let result = t.hit(&r, 0.0, 10.0);
        assert!(result.is_some());
        let rec = result.unwrap();
        assert_eq!(rec.time, 10.0);
        assert_eq!(rec.point_in_scene, Point3::<f32>::new(0.0, 2.0, 0.0));
        assert_eq!(rec.front_face, true);

        //Case 5: Miss (due to timeout)
        let r = Ray::new(Point3::<f32>::new(0.0, 2.0,10.0), Vector3::<f32>::new(0.0, 0.0, -1.0));
        let result = t.hit(&r, 0.0, 10.0 - - std::f32::MIN_POSITIVE);
        assert!(result.is_some());

        //Case 6: Miss (due to geometry)
        let r = Ray::new(Point3::<f32>::new(0.5, -1.0, 3.0), Vector3::<f32>::new(0.0, 1.0, 0.0));
        let result = t.hit(&r, 0.0, 100.0);
        assert!(result.is_none());

    }

    #[test]
    fn test_bounding_box(){
        let v0 = Point3::<f32>::new(0.0, 0.0, 0.0);
        let v1 = Point3::<f32>::new(1.0, 0.0, 0.0);
        let v2 = Point3::<f32>::new(0.5, 2.0, 2.0);
        let mat = Material::Lambertian(Lambertian::default());
        let norm = [Vector3::<f32>::new(0.0, -1.0, -1.0).normalize(); 3];
        let t = Triangle::new([v0, v1, v2], norm, mat);
        let result = t.bounding_box();
        let bb = result.unwrap();
        assert_eq!(bb, Aabb::new(Point3::<f32>::new(-0.001, -0.001, -0.001), Point3::<f32>::new(1.0 + 0.001, 2.0 + 0.001, 2.0 + 0.001)));
    }
}