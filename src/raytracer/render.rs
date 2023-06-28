use image::{DynamicImage, GenericImage};

use crate::primitives::{Point3, Ray};
use super::{Scene, intersect_scene_from_view, get_color, get_color_recursive};
use crate::Vector3;

use rayon::prelude::*;
use std::sync::Mutex;

fn make_ray(scene: &Scene, pixel_coords: (usize, usize), u: &Vector3, v: &Vector3, w: &Vector3, fov_y_rad: f64) -> Ray {
    let weight_a = ((0.5 * fov_y_rad).tan() / (0.5 * (scene.img_height as f64))) * (((pixel_coords.1 as f64) + 0.5) - (0.5 * (scene.img_width as f64)));
    let weight_b = ((0.5 * fov_y_rad).tan() / (0.5 * (scene.img_height as f64))) * ((0.5 * (scene.img_height as f64)) - (0.5 + (pixel_coords.0 as f64)));

    let ray_dir = (*u * weight_a + &(*v * weight_b) - &w).norm();

    Ray {
        position: Point3 { point: scene.camera.eye.vec },
        direction: ray_dir,
    }
}

pub fn render(scene: &Scene) -> Vec<u8> {
    let w = (scene.camera.eye - &scene.camera.center).norm();
    let u = scene.camera.up.cross(&w).norm();
    let v = w.cross(&u);

    let fov_y_rad = scene.camera.fovy.to_radians();


    let pixels: Vec<u8> = (0..scene.img_width * scene.img_height)
        .into_par_iter()
        .flat_map(|idx| {
            let i = idx / scene.img_width;
            let j = idx % scene.img_width;
            let ray = make_ray(scene, (i, j), &u, &v, &w, fov_y_rad);
            let start_idx = idx * 3;

            if let Some(id) = intersect_scene_from_view(ray, scene) {
                let pix_color = get_color_recursive(ray, &scene, id, 0);
                vec![
                    (255.0 * pix_color[0]) as u8,
                    (255.0 * pix_color[1]) as u8,
                    (255.0 * pix_color[2]) as u8,
                ]
            } else {
                vec![0u8, 0u8, 0u8]
            }
        })
        .collect();

    pixels
}


pub fn build_image(image_dim: (usize, usize), pixels: &Vec <u8>) -> DynamicImage {
    if pixels.len() % 3 != 0 {
        panic!("Number of pixel values ({}) provided is not divisible by 3!", pixels.len());
    }

    if (pixels.len() / 3) % image_dim.0 != 0 || (pixels.len() / 3) % image_dim.1 != 0 {
        panic!("Number of pixel values ({}) provided is not divisible by the dimensions!", pixels.len());
    }

    let mut image = DynamicImage::new_rgb8(image_dim.0 as u32, image_dim.1 as u32);

    let image_mutex = Mutex::new(image);

    // Write in column major (height, then width)
    (0..image_dim.1).into_par_iter().for_each(|y| {
        (0..image_dim.0).into_par_iter().for_each(|x| {
            let start_idx = (y * image_dim.0 + x) * 3;

            let mut image = image_mutex.lock().unwrap();

            image.put_pixel(x as u32, y as u32, image::Rgba([pixels[start_idx], pixels[start_idx + 1], pixels[start_idx + 2], 0]));
        });
    });

    let image = image_mutex.lock().unwrap().to_owned();
    image
}