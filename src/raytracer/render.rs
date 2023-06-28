use image::{DynamicImage, GenericImage};

use crate::primitives::{Point3, Ray};
use super::{Scene, intersect_scene_from_view, get_color, get_color_recursive};

fn make_ray(scene: &Scene, pixel_coords: (usize, usize)) -> Ray {
    // Create coordinate frame
    let w = (scene.camera.eye - &scene.camera.center).norm();
    let u = scene.camera.up.cross(&w).norm();
    let v = w.cross(&u);

    let fov_y_rad = scene.camera.fovy.to_radians();
    let weight_a = ((0.5 * fov_y_rad).tan() / (0.5 * (scene.img_height as f64))) * (((pixel_coords.1 as f64) + 0.5) - (0.5 * (scene.img_width as f64)));
    let weight_b = ((0.5 * fov_y_rad).tan() / (0.5 * (scene.img_height as f64))) * ((0.5 * (scene.img_height as f64)) - (0.5 + (pixel_coords.0 as f64)));

    let ray_dir = (u * weight_a + &(v * weight_b) - &w).norm();

    Ray {
        position: Point3 { point: scene.camera.eye.vec },
        direction: ray_dir,
    }
}

pub fn render(scene: &Scene) -> Vec <u8> {
    let mut pixels: Vec <u8> = Vec::new();
    pixels.resize(scene.img_width * scene.img_height * 3, 0u8);

    for i in 0..scene.img_height {
        for j in 0..scene.img_width {
            // Pass through ray in center of (i, j) pixel
            let ray = make_ray(scene, (i, j));
            let start_idx = (i * scene.img_width + j) * 3;

            // Intersection test with scene
            if let Some(id) = intersect_scene_from_view(ray, scene) {
                // Use get_color_recursive to get reflections
                //let pix_color = get_color(ray, scene, id, &scene.lights);
                let pix_color = get_color_recursive(ray, &scene, id, 0);
                
                pixels[start_idx + 0] = (255.0 * pix_color[0]) as u8;
                pixels[start_idx + 1] = (255.0 * pix_color[1]) as u8;
                pixels[start_idx + 2] = (255.0 * pix_color[2]) as u8;
            }
            else {
                // Color all pixels black
                pixels[start_idx + 0] = 0u8;
                pixels[start_idx + 1] = 0u8;
                pixels[start_idx + 2] = 0u8;
            }
        }
    }

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

    // Write in column major (height, then width)
    for y in 0..image_dim.1 {
        for x in 0..image_dim.0 {
            let start_idx = (y * image_dim.0 + x) * 3;

            image.put_pixel(x as u32, y as u32, image::Rgba([pixels[start_idx], pixels[start_idx + 1], pixels[start_idx + 2], 0]));
        }
    }

    image
}