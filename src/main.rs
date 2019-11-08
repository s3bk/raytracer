mod color;
mod math;
mod ray;
mod scene;

pub use crate::color::Color;
pub use crate::math::Vector3;
pub use crate::ray::{Hit, Ray};
pub use crate::scene::{Object, Scene, Sphere};

fn main() {
    println!("Hello, world!");

    let mut scene = Scene::default();
    scene.add(Sphere {
        center: Vector3::new(10.0, 0.0, 0.0),
        size: 5.0,
    });

    let mut ray = Ray {
        start: Vector3::new(0.0, 0.0, 3.0),
        direction: Vector3::new(1.0, 0.0, 0.0),
    };
    let mut exclude = None;
    for _ in 0..5 {
        let hit = scene.calculate_hit(&ray, &exclude);
        println!("{:?}", hit);
        if let Some(hit) = hit {
            exclude = Some(hit.object);

            println!("    distance: {:?}", (hit.position - ray.start).length());
            ray = Ray {
                start: hit.position,
                direction: hit.out_angle(ray.direction),
            };
        }
    }
}
