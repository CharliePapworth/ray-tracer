use crate::image::Color;
use crate::primitives::{GeometricPrimitive, GeometricPrimitives, Primitives};
use crate::{material::*, sampler};
use crate::primitives::rect::*;
use crate::util::*;
use crate::nalgebra::{Vector3, Point3};

#[derive (Clone)]

/// Contains all information regarding the scene. The raytracing_primitives and the rasterization_primitives contain
/// the same primtitives, but raytracing_primitives may contain acceleration structures designed to improve
/// raytracing performance. The background color is the ambient color of the scene.
pub struct SceneData {
    pub raytracing_primitives: Primitives,
    pub rasterization_primitives: GeometricPrimitives,
    pub background: Color,   
}

/// Returns a world filled with spheres.
pub fn sphere_world() -> (GeometricPrimitives, Color, Point3<f64>, Point3<f64>) {
    let mut world = GeometricPrimitives::new();
    let background = Color::new(0.7, 0.8, 1.0);
    let look_from = Point3::<f64>::new(13.0, 2.0, 3.0);
    let look_at = Point3::<f64>::new(0.0, 0.0, 0.0);

    let mat_ground = Material::new_lambertian(Color::new(0.5, 0.5, 0.5));
    let ground = GeometricPrimitive::new_sphere(Point3::<f64>::new(0.0,-1000.0,0.0), 1000.0, mat_ground);
    world.add(ground);

    for a in -11..12{
        for b in -11..12{
            let choose_mat = rand_double(0.0, 1.0);
            let center = Point3::<f64>::new(a as f64 + 0.9*rand_double(0.0, 1.0), 0.2, b as f64 + 0.9*rand_double(0.0, 1.0));

            if choose_mat < 0.6{
                let albedo = sampler::rand(0.0, 1.0).component_mul(&sampler::rand(0.0, 1.0));
                let sphere_material = Material::new_lambertian(albedo);
                let sphere = GeometricPrimitive::new_sphere(center, 0.2, sphere_material);
                world.add(sphere);
            } else if choose_mat < 0.9{
                let albedo = sampler::rand(0.5, 1.0);
                let fuzz = rand_double(0.0, 0.5);
                let sphere_material = Material::new_metal(albedo, fuzz);
                let sphere = GeometricPrimitive::new_sphere(center, 0.2, sphere_material);
               world.add(sphere);
            } else {
                let sphere_material = Material::new_dielectric(1.5);
                let sphere = GeometricPrimitive::new_sphere(center, 0.2, sphere_material);
                world.add(sphere);
            }
        }
    }
    let mat_center = Material::new_dielectric(1.5);
    let mat_left = Material::new_lambertian(Color::new(0.4, 0.2, 0.1));
    let mat_right = Material::new_metal(Color::new(0.7, 0.6, 0.5), 0.0);

    let sphere_center = GeometricPrimitive::new_sphere(Point3::<f64>::new(0.0,1.0,0.0), 1.0, mat_center);
    let sphere_left = GeometricPrimitive::new_sphere(Point3::<f64>::new(-4.0,1.0,0.0), 1.0, mat_left);
    let sphere_right = GeometricPrimitive::new_sphere(Point3::<f64>::new(4.0,1.0,0.0), 1.0, mat_right);
    
    world.add(sphere_center);
    world.add(sphere_left);
    world.add(sphere_right);


    (world, background, look_from, look_at)
}

/// Returns a scene containing a single light.
pub fn light_test() -> (GeometricPrimitives, Color, Point3<f64>, Point3<f64>) {
    let mut world = GeometricPrimitives::new();
    let background = Color::new(0.9, 0.9, 0.9);
    let look_from = Point3::<f64>::new(26.0, 3.0, 6.0);
    let look_at = Point3::<f64>::new(0.0, 2.0, 0.0);

    let mat = Material::new_lambertian(Color::new(0.4, 0.2, 0.1));
    let ground = GeometricPrimitive::new_sphere(Point3::<f64>::new(0.0, -1000.0, 0.0), 1000.0, mat);
    let sphere = GeometricPrimitive::new_sphere(Point3::<f64>::new(0.0, 2.0, 0.0), 2.0, Material::new_lambertian(Color::new(0.8, 0.8, 0.8))); 

    let diff_light = Material::new_diffuse_light(Color::new(4.0,4.0,4.0));
    let _rect = Box::new(Rect::new(RectAxes::XY, -1.0, 2.0, 1.0, 3.0, 4.0, diff_light));
    world.add(ground);
    world.add(sphere);
    
    (world, background, look_from, look_at)

}

/// Returns a scene containing a single triangle.
pub fn triangle_test() -> (GeometricPrimitives, Color, Point3<f64>, Point3<f64>) {
    let mut world = GeometricPrimitives::new();
    let background = Color::new(0.9, 0.9, 0.9);
    let look_from = Point3::<f64>::new(0.0, 2.0, 26.0);
    let look_at = Point3::<f64>::new(0.0, 0.0, 0.0);

    let mat = Material::new_lambertian(Color::new(0.4, 0.2, 0.1));
    let _ground = GeometricPrimitive::new_sphere(Point3::<f64>::new(0.0, -1000.0, 0.0), 1000.0, mat);
 
    let mat = Material::new_lambertian(Vector3::<f64>::new(0.8, 0.8, 0.8));
    let v0 = Point3::<f64>::new(-2.0, 0.1, 0.0);
    let v1 = Point3::<f64>::new(2.0, 0.1, 0.0);
    let v2 = Point3::<f64>::new(0.0, 2.1, 0.0);
    let norms = [Vector3::<f64>::new(0.0, 0.0, 1.0); 3];
    let tri = GeometricPrimitive::new_triangle([v0, v1, v2], norms, mat);
    //world.add(ground);
    world.add(tri);
    
    (world, background, look_from, look_at)

}


/// Returns a scene containing an object defined by a .obj file (on a spherical world).
pub fn obj_test() -> (GeometricPrimitives, Color, Point3<f64>, Point3<f64>) {
    let _world = GeometricPrimitives::new(); 
    let background = Color::new(0.9, 0.9, 0.9);
    let look_from = Point3::<f64>::new(-20.0, 5.0, 20.0);
    let look_at = Point3::<f64>::new(0.0, 0.0, 0.0);

    let mut mesh = GeometricPrimitives::new(); 
    let mat = Material::new_lambertian(Color::new(0.4, 0.2, 0.1));
    let ground = GeometricPrimitive::new_sphere(Point3::<f64>::new(0.0, -1000.0, 0.0), 1000.0, mat);
    let (models, materials) = import_obj("C:/Users/Charlie/Ray_Tracer/ray-tracer/obj/car.obj");
    let diff_light = Material::new_diffuse_light(Color::new(4.0,4.0,4.0));
    let rect = GeometricPrimitive::new_rect(RectAxes::XY, -4.0, -2.0, 1.0, 8.0, 4.0, diff_light);
    mesh.add_obj(models, materials);
    mesh.add(ground);
    //mesh.add(rect);
    
    (mesh, background, look_from, look_at)
}

pub fn mesh_test() -> (GeometricPrimitives, Color, Point3<f64>, Point3<f64>) {
    let mut world = GeometricPrimitives::new(); 
    let background = Color::new(0.9, 0.9, 0.9);
    let look_from = Point3::<f64>::new(26.0, 10.0, 10.0);
    let look_at = Point3::<f64>::new(0.0, 0.0, 0.0);

    let mut mesh_1 = tobj::Mesh::default();
    let mut mesh_2 = tobj::Mesh::default();
    let mut mesh_3 = tobj::Mesh::default();

    mesh_1.positions = vec!(-2.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 1.0, 0.0,
                            0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 1.0, 1.0, 0.0,
                            2.0, 0.0, 0.0, 4.0, 0.0, 0.0, 3.0, 1.0, 0.0);
                            
    mesh_1.normals = vec!(0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
                          0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
                          0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0);

    mesh_1.indices = vec!(0, 1, 2, 3, 4, 5, 6, 7, 8);


    mesh_2.positions = vec!(-2.0, 1.0, 0.0, 0.0, 1.0, 0.0, -1.0, 2.0, 0.0,
                            0.0, 1.0, 0.0, 2.0, 1.0, 0.0, 1.0, 2.0, 0.0,
                            2.0, 1.0, 0.0, 4.0, 1.0, 0.0, 3.0, 2.0, 0.0);

    mesh_2.normals = vec!(0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
                        0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
                        0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0);

    mesh_2.indices = vec!(0, 1, 2, 3, 4, 5, 6, 7, 8);
                    

    mesh_3.positions = vec!(-2.0, 2.0, 0.0, 0.0, 2.0, 0.0, -1.0, 3.0, 0.0,
                            0.0, 2.0, 0.0, 2.0, 2.0, 0.0, 1.0, 3.0, 0.0,
                            2.0, 2.0, 0.0, 4.0, 2.0, 0.0, 3.0, 3.0, 0.0);

    mesh_3.normals = vec!(0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
                          0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
                          0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0);

    mesh_3.indices = vec!(0, 1, 2, 3, 4, 5, 6, 7, 8);


    let test_1 = tobj::Model::new(mesh_1, "test_1".to_string());
    let test_2 = tobj::Model::new(mesh_2, "test_1".to_string());
    let test_3 = tobj::Model::new(mesh_3, "test_1".to_string());


    let test = vec!(test_1, test_2, test_3);
    world.add_obj(test, None);

    (world, background, look_from, look_at)

}