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
use palette::Srgb;

use rayon::prelude::*;

fn main() {
    let start = time::precise_time_s();
    let mut scene = Scene::new(Color::black());
    scene.ambient_lights.push(Color::new(0.1, 0.1, 0.1));
    scene
        .directional_lights
        .push((Vector3::new(0.0, 0.0, -1.0), Color::white() * 0.8));

    scene.add(Sphere {
        center: Vector3::new(11.0, 3.0, 0.0),
        size: 3.0,
        color: Color::red(),
    });

    scene.add(Sphere {
        center: Vector3::new(9.0, -2.5, -2.5),
        size: 3.0,
        color: Color::green(),
    });

    scene.add(Sphere {
        center: Vector3::new(10.0, -1.5, 3.5),
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
    println!("Generated in {:.3} seconds", time::precise_time_s() - start);
    let start = time::precise_time_s();

    let mut img = image::ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    for (x, y, color) in pixels {
        use palette::rgb::Rgb;
        use palette::Alpha;
        use palette::encoding::pixel::Pixel;
        let rgb_linear = Rgb::new(color.r, color.g, color.b);
        let srgba = Alpha {
            color: Srgb::from_linear(rgb_linear),
            alpha: 1.0
        };
        let pixel: [u8; 4] = srgba
            .into_format()
            .into_raw();
        
        img.put_pixel(
            x,
            y,
            image::Rgba(pixel),
        )
    }
    img.save("out.png").expect("Could not save image");
    println!("Exported in {:.3} seconds", time::precise_time_s() - start);
}

fn calculate_color(scene: &Scene, mut ray: Ray, max_bounces: usize) -> Color {
    let mut exclude = None;
    let mut color = scene.background_color;
    let mut scale = Color::white();

    for i in 0..max_bounces {
        let hit = scene.calculate_hit(&ray, &exclude);
        if let Some(hit) = hit {
            exclude = Some(hit.object);

            let mut this_color = Color::black();
            let object_color = hit.object.color();
            for &(direction, lamp_color) in &scene.directional_lights {
                let shade = direction.dot(hit.normal);
                if shade > 0.0 {
                    this_color.add(object_color * lamp_color * scale, shade);
                }
            }

            for &ambient in &scene.ambient_lights {
                this_color.add(object_color * ambient * scale, 1.0);
            }

            let pass_through = 0.2 + 0.8 * ray.direction.dot(hit.normal).powi(2);
            let refraction = 1.0 - pass_through;

            color = color + this_color * pass_through;

            // adjust refraction factor
            scale = scale * refraction;

            ray = Ray {
                start: hit.position,
                direction: hit.out_angle(ray.direction),
            };
        } else {
            break;
        }
    }

    color
}
