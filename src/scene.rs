use crate::math::Vector3;
use crate::{Hit, Ray};

#[derive(Debug, Default)]
pub struct Scene {
    pub objects: Vec<Box<dyn Object>>,
}

impl Scene {
    pub fn calculate_hit(&self, ray: &Ray, exclude: &Option<&dyn Object>) -> Option<Hit> {
        let mut nearest_hit: Option<(Hit, f32)> = None;

        // Get the pointer to the exclude object, or std::ptr::null() if no exclude object is provided
        // This is completely safe, we're just using this to compare the pointer to another pointer later
        // As long as we don't dereference this, everything is fine
        for obj in &self.objects {
            if let Some(exclude) = exclude {
                let obj_ptr = &**obj as *const dyn Object;
                let exclude_ptr = &**exclude as *const dyn Object;
                println!("Comparing pointers\n - {:?}\n - {:?}", obj_ptr, exclude_ptr);
                println!("std::ptr::eq: {:?}", std::ptr::eq(obj_ptr, exclude_ptr));
                println!("          ==: {:?}", obj_ptr == exclude_ptr);
                /*
                Comparing pointers
                - 0x55756aa87c80
                - 0x55756aa87c80
                std::ptr::eq: false
                          ==: false
                */
                if obj_ptr == exclude_ptr {
                    println!("Pointers equal, ignoring");
                    continue;
                }
            }

            println!("Found object {:?}", obj);

            if let Some(hit) = obj.intersect(ray) {
                let distance_squared = (hit.position - ray.start).length_squared();
                if let Some((_, previous_distance_squared)) = &nearest_hit {
                    if *previous_distance_squared < distance_squared {
                        continue;
                    }
                }
                nearest_hit = Some((hit, distance_squared));
            }
        }

        nearest_hit.map(|(hit, _)| hit)
    }

    pub fn add(&mut self, obj: impl Object + 'static) {
        self.objects.push(Box::new(obj));
    }
}

pub trait Object: std::fmt::Debug {
    fn intersect<'a>(&'a self, ray: &Ray) -> Option<Hit<'a>>;
}

#[derive(Debug)]
pub struct Sphere {
    pub center: Vector3,
    pub size: f32,
}

impl Object for Sphere {
    fn intersect<'a>(&'a self, ray: &Ray) -> Option<Hit<'a>> {
        let distance_to_center = self.center - ray.start;
        let v = distance_to_center.dot(ray.direction);
        let disc = self.size.powf(2.0) - (distance_to_center.dot(distance_to_center) - v.powf(2.0));

        if disc < 0.0 {
            return None;
        }

        let d = disc.sqrt();
        let intersect_at = ray.start + (v - d) * ray.direction;
        let normal = (self.center - intersect_at).normalize();

        Some(Hit {
            position: intersect_at,
            normal,
            object: self,
        })
    }
}
