use crate::vec::*;
use crate::sphere::*;
use crate::traceable::*;
use crate::material::*;
use crate::rect::*;
use crate::util::*;
use crate::triangle::*;

pub fn sphere_world() -> (TraceableList, Color, Point3, Point3){
    let mut world = TraceableList::new();
    let background = Color::new(0.7, 0.8, 1.0);
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);

    let mat_ground = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    let ground = Box::new(Sphere::new(Point3::new(0.0,-1000.0,0.0), 1000.0, mat_ground));
    world.add(ground);

    for a in -11..12{
        for b in -11..12{
            let choose_mat = rand_double(0.0, 1.0);
            let center = Point3::new(a as f64 + 0.9*rand_double(0.0, 1.0), 0.2, b as f64 + 0.9*rand_double(0.0, 1.0));

            if choose_mat < 0.6{
                let albedo = Color::rand(0.0, 1.0).elementwise_mult(&Color::rand(0.0, 1.0));
                let sphere_material = Lambertian::new(albedo);
                let sphere = Box::new(Sphere::new(center, 0.2, sphere_material));
                world.add(sphere);
            } else if choose_mat < 0.9{
                let albedo = Color::rand(0.5, 1.0);
                let fuzz = rand_double(0.0, 0.5);
                let sphere_material = Metal::new(albedo, fuzz);
                let sphere = Box::new(Sphere::new(center, 0.2, sphere_material));
                world.add(sphere);
            } else {
                let sphere_material = Dielectric::new(1.5);
                let sphere = Box::new(Sphere::new(center, 0.2, sphere_material));
                world.add(sphere);
            }
        }
    }
    let mat_center = Dielectric::new(1.5);
    let mat_left = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    let mat_right = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);

    let sphere_center = Box::new(Sphere::new(Point3::new(0.0,1.0,0.0), 1.0, mat_center));
    let sphere_left = Box::new(Sphere::new(Point3::new(-4.0,1.0,0.0), 1.0, mat_left));
    let sphere_right = Box::new(Sphere::new(Point3::new(4.0,1.0,0.0), 1.0, mat_right));
    
    world.add(sphere_center);
    world.add(sphere_left);
    world.add(sphere_right);


    (world, background, look_from, look_at)
}

pub fn light_test() -> (TraceableList, Color, Point3, Point3){
    let mut world = TraceableList::new();
    let background = Color::new(0.9, 0.9, 0.9);
    let look_from = Point3::new(26.0, 3.0, 6.0);
    let look_at = Point3::new(0.0, 2.0, 0.0);

    let mat = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    let ground = Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, mat));
    let sphere = Box::new(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, Lambertian::new(Color::new(0.8, 0.8, 0.8)))); 

    let diff_light = DiffuseLights::new(Color::new(4.0,4.0,4.0));
    let rect = Box::new(Rect::new(RectAxes::XY, -1.0, 2.0, 1.0, 3.0, 4.0, diff_light));
    world.add(ground);
    world.add(sphere);
    //world.add(rect);
    
    (world, background, look_from, look_at)

}

pub fn triangle_test() -> (TraceableList, Color, Point3, Point3){
    let mut world = TraceableList::new();
    let background = Color::new(0.9, 0.9, 0.9);
    let look_from = Point3::new(0.0, 2.0, 26.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);

    let mat = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    let ground = Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, mat));
 
    let mat = Lambertian::new(Vec3::new(0.8, 0.8, 0.8));
    let v0 = Vec3::new(-2.0, 0.1, 0.0);
    let v1 = Vec3::new(2.0, 0.1, 0.0);
    let v2 = Vec3::new(0.0, 2.1, 0.0);
    let norms = [Vec3::new(0.0, 0.0, 1.0); 3];
    let tri = Box::new(Triangle::new([v0, v1, v2], norms, mat));
    world.add(ground);
    world.add(tri);
    
    (world, background, look_from, look_at)

}

pub fn obj_test() -> (TraceableList, Color, Point3, Point3){
    let mut world = TraceableList::new(); 
    let background = Color::new(0.9, 0.9, 0.9);
    let look_from = Point3::new(-40.0, 10.0, 40.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);

    let mut mesh = TraceableList::new(); 
    let mat = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    let ground = Box::new(Sphere::new(Point3::new(0.0, -1050.0, 0.0), 1000.0, mat));
    let (mut models, materials) = import_obj("C:/Users/Charlie/Ray_Tracer/ray-tracer/lowpoly_bird.obj");
    let diff_light = DiffuseLights::new(Color::new(4.0,4.0,4.0));
    let rect = Box::new(Rect::new(RectAxes::XY, -4.0, -2.0, 1.0, 8.0, 4.0, diff_light));
    mesh.add_obj(models, materials);
    //world.add(ground);
    //world.add(Box::new(mesh.to_Bvh()));
    
    (mesh, background, look_from, look_at)
}

pub fn mesh_test() -> (TraceableList, Color, Point3, Point3){
    let mut world = TraceableList::new(); 
    let background = Color::new(0.9, 0.9, 0.9);
    let look_from = Point3::new(26.0, 10.0, 10.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);

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