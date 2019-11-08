#[macro_use]
extern crate itertools;

mod color;
mod math;
mod ray;
mod scene;

pub use crate::color::Color;
pub use crate::math::Vector3;
pub use crate::ray::{Hit, Ray};
pub use crate::scene::{Object, Scene, Sphere};

use rayon::prelude::*;

fn main() {
    let mut scene = Scene::new(Color::black());
    scene.ambient_lights.push(Color::new(0.1, 0.1, 0.1));
    scene
        .directional_lights
        .push((Vector3::new(0.0, 0.0, 1.0), Color::white()));

    scene.add(Sphere {
        center: Vector3::new(11.0, 2.0, 0.0),
        size: 3.0,
        color: Color::red(),
    });

    scene.add(Sphere {
        center: Vector3::new(9.0, -2.0, -2.0),
        size: 3.0,
        color: Color::green(),
    });

    scene.add(Sphere {
        center: Vector3::new(10.0, 0.0, 3.0),
        size: 3.0,
        color: Color::blue(),
    });

    const IMAGE_WIDTH: u32 = 600;
    const IMAGE_HEIGHT: u32 = 600;

    let pixels = iproduct!(0..IMAGE_WIDTH, 0..IMAGE_HEIGHT).collect::<Vec<_>>();

    let pixels = pixels
        .into_par_iter()
        .map(|(x, y)| {
            let fx = (x as f32) / 50. - 5.5;
            let fy = (y as f32) / 50. - 5.5;
            let ray = Ray {
                start: Vector3::new(0.0, fx, fy),
                direction: Vector3::new(1.0, 0.0, 0.0),
            };
            let color = calculate_color(&scene, ray, 10);
            (x, y, color)
        })
        .collect::<Vec<_>>();

    let mut img = image::ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    for (x, y, color) in pixels {
        img.put_pixel(
            x,
            y,
            image::Rgba([
                (color.r * 255.0) as u8,
                (color.g * 255.0) as u8,
                (color.b * 255.0) as u8,
                255,
            ]),
        )
    }
    img.save("out.png").expect("Could not save image");
}

fn calculate_color(scene: &Scene, mut ray: Ray, max_bounces: usize) -> Color {
    let mut exclude = None;
    let mut color: Option<Color> = None;
    for i in 0..max_bounces {
        let hit = scene.calculate_hit(&ray, &exclude);
        if let Some(hit) = hit {
            exclude = Some(hit.object);

            let mut object_color = hit.object.color();
            for (direction, color) in &scene.directional_lights {
                let shade = direction.dot(hit.normal);
                if shade > 0.0 {
                    object_color.add_directional(*color, shade);
                }
            }

            for ambient in &scene.ambient_lights {
                object_color.add_ambient(*ambient);
            }

            color = Some(match color {
                Some(mut color) => {
                    color.change_towards(object_color, 0.1 / i as f32);
                    color
                }
                None => object_color,
            });

            ray = Ray {
                start: hit.position,
                direction: hit.out_angle(ray.direction),
            };
        } else {
            break;
        }
    }

    color.unwrap_or(scene.background_color)
}
