use std::io::{stderr, Write};
use std::rc::Rc;
use crate::hitable_list::HitableList;
use crate::vec3::{Color, Point3, Vec3};
use crate::ray::Ray;
use sphere::Sphere;
use crate::hitable::Hitable;
use rand::{Rng, thread_rng};
use crate::camera::Camera;
use crate::material::{Dielectric, Lambertian, Metal};

mod vec3;
mod ray;
mod hitable;
mod sphere;
mod hitable_list;
mod camera;
mod material;

fn color(r: &Ray, world_list: &HitableList, depth: u64) -> Color {
    if depth <= 0 {
        return Color::new(0.,0.,0.);
    }
    let hit = world_list.hit(r, 0.001,f64::INFINITY );
    match hit {
        Some(rec)=>{
            if let Some((attenuation, scattered)) = rec.get_material().scatter(r, &rec) {
                color(&scattered, world_list, depth -1) * attenuation
            }
            else {
                Color::new(0.,0.,0.)
            }
        },
        None => {
            let unit_direction = r.direction().normalized();
            let t = 0.5 * (unit_direction.y() + 1.);
            (1. - t) * Color::new(1.,1.,1.) + t * Color::new(0.5, 0.7, 1.)
        }
    }
}

fn main() {
    //IMAGE
    const ASPECT_RATIO : f64 = 16.0 / 9.0;
    const IMAGE_WIDTH:u64 = 256;
    const IMAGE_HEIGHT:u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL:u64 = 200;
    const MAX_DEPTH:u64 = 15;
    //CAMERA
    let viewport_height = 2.0;
    let viewport_witdth = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.,0.,0.);
    let horizontal = Vec3::new(viewport_witdth, 0.,0.);
    let vertical = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0., 0., focal_length);

    let camera = Camera::new(origin,lower_left_corner,vertical, horizontal);

    //OBJECTS
    let mut world_list = HitableList::new();
    let mat_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.)));
    let mat_center = Rc::new(Lambertian::new(Color::new(0.7,0.3, 0.3)));
    let mat_left = Rc::new(Metal::new(Color::new(0.8,0.8,0.8), 0.3));
    let mat_right = Rc::new(Dielectric::new(1.5));
    let mat_right_inner = Rc::new(Dielectric::new(1.5));

    let sphere_ground = Sphere::new(Point3::new(0.,-100.5, -1.), 100.0, mat_ground);
    let sphere_center = Sphere::new(Point3::new(0.,0.,-1.), 0.5, mat_center);
    let sphere_left = Sphere::new(Point3::new(-1.,0.,-1.), 0.5, mat_left);
    let sphere_right = Sphere::new(Point3::new(1., 0., -1.), 0.5, mat_right);
    let sphere_right_inner = Sphere::new(Point3::new(-1.0, 0., -1.), -0.4, mat_right_inner);

    world_list.push(Box::new(sphere_ground));
    world_list.push(Box::new(sphere_center));
    world_list.push(Box::new(sphere_left));
    world_list.push(Box::new(sphere_right_inner));
    world_list.push(Box::new(sphere_right));

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    //Initialize RNG
    let mut rng = thread_rng();
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines Remaining: {:3}", IMAGE_HEIGHT -j -1);
        stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            let mut col = Color::new(0.,0.,0.);
            for _s in 0..SAMPLES_PER_PIXEL {
                let random_u:f64 = rng.gen();
                let random_v:f64 = rng.gen();
                let u = (i as f64 + random_u) / (IMAGE_WIDTH -1) as f64;
                let v = (j as f64 + random_v) / (IMAGE_HEIGHT -1) as f64;
                let r = camera.get_ray(u,v);
                col += color(&r, &world_list, MAX_DEPTH);
            }

            println!("{}", col.format_color(SAMPLES_PER_PIXEL))
        }
    }
}