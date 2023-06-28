mod primitives;
mod geometry;
mod file_io;
mod raytracer;

use std::env;
use primitives::{Vector3, Point3, Matrix4, Ray};

use std::time::Instant;

fn test1() {
    let mymat = Matrix4::new(
        2.0, 2.0, 3.0, 3.0,
        5.0, 6.0, 7.0, 7.0,
        9.0, 10.0, 11.0, 11.0,
        0.0, 0.0, 0.0, 1.0,
    );
    let pos = Point3::new(8.0, 4.0, -2.0);
    let dir = Vector3::new(-3.0, -4.0, 7.0);
    let myray = Ray { position: pos, direction: dir };

    println!("Ray pos: {:?}", myray.position);
    println!("Ray dir: {:?}", myray.direction);

    println!("Transforming ray...");
    let myray1 = mymat * &myray;
    println!("Ray pos: {:?}", myray1.position);
    println!("Ray dir: {:?}", myray1.direction);

    println!("Inverse transforming ray...");
    let myray2 = mymat.mat_invtf_ray(&myray);
    println!("Ray pos: {:?}", myray2.position);
    println!("Ray dir: {:?}", myray2.direction);

    println!("Undo inverse transform on ray...");
    let myray3 = mymat * &myray2;
    println!("Ray pos: {:?}", myray3.position);
    println!("Ray dir: {:?}", myray3.direction);
}

fn main() {
    let start = Instant::now();

    let args: Vec <String> = env::args().collect();

    if args.len() < 2 {
        panic!("No scene files to render!");
    }

    println!("Reading scene file \"{}\"...", args[1]);
    let scene = file_io::read_scene_file(&args[1]);
    println!("===== SCENE INFO =====");
    println!("Image size: {}x{}", scene.img_width, scene.img_height);
    println!("# vertices: {}", scene.vertices.0.len());
    println!("# shapes: {}", scene.shapes.0.len());
    println!("# lights: {}", scene.lights.lights.len());

    println!("Rendering scene. This will take some time...");
    let pixels = raytracer::render(&scene);

    println!("Saving scene to \"{}\"", "out.png");
    let img_obj = raytracer::build_image((scene.img_width, scene.img_height), &pixels);
    file_io::write_image(&"out.png".to_string(), img_obj);

    let end = start.elapsed();

    println!("===== RENDERING DONE!!! =====");

    println!("The code took {:?}", end);
}