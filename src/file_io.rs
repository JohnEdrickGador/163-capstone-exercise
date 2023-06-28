use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use image::DynamicImage;

use crate::raytracer::Scene;
use crate::primitives::*;
use crate::geometry::*;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn right_mul_transf_stack(m: &Matrix4, transf_stack: &mut Vec <Matrix4>) {
    if let Some(t) = transf_stack.last_mut() {
        *t = *t * m;
    }

}

fn match_cmd(cmd_line: String, transf_stack: &mut Vec <Matrix4>, current_material: &mut Material, scene_info: &mut Scene) {
    let cmd_tokens: Vec <&str> = cmd_line.split_whitespace().collect();

    if cmd_tokens.len() <= 0 {
        return;
    }

    let arg_tokens = &cmd_tokens[1..];

    match cmd_tokens[0] {
        "size" => {
            if arg_tokens.len() >= 2 {
                if let (Ok(w), Ok(h)) = (arg_tokens[0].parse::<usize>(), arg_tokens[1].parse::<usize>()) {
                    scene_info.img_width = w;
                    scene_info.img_height = h;
                }
            }
        },
        "maxdepth" => {
            if arg_tokens.len() >= 1 {
                if let Ok(d) = arg_tokens[0].parse::<usize>() {
                    scene_info.max_recurse_depth = d;
                }
            }
        },
        "camera" => {
            if arg_tokens.len() >= 10 {
                for i in 0..3 {
                    if let (Ok(e), Ok(c), Ok(u)) = (arg_tokens[i].parse::<f64>(), arg_tokens[i + 3].parse::<f64>(), arg_tokens[i + 6].parse::<f64>()) {
                        scene_info.camera.eye[i] = e;
                        scene_info.camera.center[i] = c;
                        scene_info.camera.up[i] = u;
                    }
                }

                if let Ok(f) = arg_tokens[9].parse::<f64>() {
                    scene_info.camera.fovy = f;
                }
            }
        },
        "directional" => {
            if arg_tokens.len() >= 6 {
                let mut light_dir = Vector3::new_empty();
                let mut color = RGBColor::new_empty();


                for i in 0..3 {
                    if let (Ok(d), Ok(c)) = (arg_tokens[i].parse::<f64>(), arg_tokens[i + 3].parse::<f64>()) {
                        light_dir[i] = d;
                        color[i] = c;
                    }
                }

                scene_info.lights.lights.push(LightType::Directional(DirectionalLight {
                    direction: light_dir,
                    color: color,
                }));
            }
        },
        "point" => {
            if arg_tokens.len() >= 6 {
                let mut light_pos = Point3::new_empty();
                let mut color = RGBColor::new_empty();

                for i in 0..3 {
                    if let (Ok(d), Ok(c)) = (arg_tokens[i].parse::<f64>(), arg_tokens[i + 3].parse::<f64>()) {
                        light_pos[i] = d;
                        color[i] = c;
                    }
                }

                scene_info.lights.lights.push(LightType::Point(PointLight {
                    position: light_pos,
                    color: color,
                }));
            }
        },
        "attenuation" => {
            if arg_tokens.len() >= 3 {
                for i in 0..3 {
                    if let Ok(v) = arg_tokens[i].parse::<f64>() {
                        scene_info.lights.attenuation[i] = v;
                    }
                }
            }
        },
        "ambient" => {
            if arg_tokens.len() >= 3 {
                for i in 0..3 {
                    if let Ok(v) = arg_tokens[i].parse::<f64>() {
                        current_material.ambient[i] = v;
                    }
                }
            }
        },
        "diffuse" => {
            if arg_tokens.len() >= 3 {
                for i in 0..3 {
                    if let Ok(v) = arg_tokens[i].parse::<f64>() {
                        current_material.diffuse[i] = v;
                    }
                }
            }
        },
        "specular" => {
            if arg_tokens.len() >= 3 {
                for i in 0..3 {
                    if let Ok(v) = arg_tokens[i].parse::<f64>() {
                        current_material.specular[i] = v;
                    }
                }
            }
        },
        "emission" => {
            if arg_tokens.len() >= 3 {
                for i in 0..3 {
                    if let Ok(v) = arg_tokens[i].parse::<f64>() {
                        current_material.emission[i] = v;
                    }
                }
            }
        },
        "shininess" => {
            if arg_tokens.len() >= 1 {
                if let Ok(v) = arg_tokens[0].parse::<f64>() {
                    current_material.shininess = v;
                }
            }
        },
        "vertex" => {
            if arg_tokens.len() >= 3 {
                let mut new_vertex = Point3::new_empty();

                for i in 0..3 {
                    if let Ok(v) = arg_tokens[i].parse::<f64>() {
                        new_vertex[i] = v;
                    }
                }

                scene_info.vertices.0.push(new_vertex);
            }
        },
        "tri" => {
            if arg_tokens.len() >= 3 {
                let mut new_tri = Triangle::new();

                for i in 0..3 {
                    if let Ok(v_idx) = arg_tokens[i].parse::<usize>() {
                        new_tri.vertices[i] = scene_info.vertices.0[v_idx];
                    }
                }

                new_tri.material = *current_material;

                if let Some(v) = transf_stack.last() {
                    new_tri.transform = *v;
                }

                scene_info.shapes.0.push(ShapeType::Triangle(new_tri));
            }
        },
        "sphere" => {
            let mut new_sphere = Sphere::new();

            if arg_tokens.len() >= 4 {
                for i in 0..3 {
                    if let Ok(v) = arg_tokens[i].parse::<f64>() {
                        new_sphere.center[i] = v;
                    }
                }

                if let Ok(v) = arg_tokens[3].parse::<f64>() {
                    new_sphere.radius = v;
                }

                new_sphere.material = *current_material;

                if let Some(v) = transf_stack.last() {
                    new_sphere.transform = *v;
                }

                scene_info.shapes.0.push(ShapeType::Sphere(new_sphere));
            }
        },
        "scale" => {
            if arg_tokens.len() >= 3 {
                if let (Ok(x), Ok(y), Ok(z)) = (arg_tokens[0].parse::<f64>(), arg_tokens[1].parse::<f64>(), arg_tokens[2].parse::<f64>()) {
                    let trans_mat = Matrix4::new_scale(x, y, z);
                    right_mul_transf_stack(&trans_mat, transf_stack);
                }
            }
        },
        "rotate" => {
            if arg_tokens.len() >= 4 {
                if let (Ok(x), Ok(y), Ok(z), Ok(r)) = (arg_tokens[0].parse::<f64>(), arg_tokens[1].parse::<f64>(), arg_tokens[2].parse::<f64>(), arg_tokens[3].parse::<f64>()) {
                    let trans_mat = Matrix4::new_rotate(&Vector3::new(x, y, z), r);
                    right_mul_transf_stack(&trans_mat, transf_stack);
                }
            }
        },
        "translate" => {
            if arg_tokens.len() >= 3 {
                if let (Ok(x), Ok(y), Ok(z)) = (arg_tokens[0].parse::<f64>(), arg_tokens[1].parse::<f64>(), arg_tokens[2].parse::<f64>()) {
                    let trans_mat = Matrix4::new_translate(x, y, z);
                    right_mul_transf_stack(&trans_mat, transf_stack);
                }
            }
        },
        "pushTransform" => {
            if let Some(t) = transf_stack.last() {
                transf_stack.push(*t);
            }
        },
        "popTransform" => {
            if let None = transf_stack.pop() {
                // No transforms to pop!
            }
        },
        _ => ()
    }
}

pub fn read_scene_file(file_path_str: &String) -> Scene {
    let file_path = Path::new(&file_path_str);
    let mut scene_info = Scene::new();
    let mut current_material = Material::new();
    let mut transf_stack = vec![Matrix4::new_on_diag(1.0)];

    match read_lines(&file_path_str) {
        Err(why) => {
            panic!("Cannot open file {}: {}", file_path.display(), why);
        },
        Ok(file_obj) => {
            for each_line in file_obj {
                if let Ok(each_line_safe) = each_line {
                    match_cmd(each_line_safe, &mut transf_stack, &mut current_material, &mut scene_info);
                }
            }

            return scene_info;
        }
    }
}

pub fn write_image(file_path_str: &String, image: DynamicImage) {
    if let Err(e) = image.save(file_path_str) {
        panic!("Error occurred saving image to file {}: {}", file_path_str, e);
    }
}