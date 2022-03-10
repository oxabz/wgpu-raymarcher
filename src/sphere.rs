use std::sync::Mutex;
use bytemuck::{Pod, Zeroable};
use lazy_static::lazy_static;
use rand_pcg::Lcg128Xsl64;
use rand::distributions::{Distribution, Uniform};
use rand::SeedableRng;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Sphere{
    position: [f32;3],
    radius: f32
}

lazy_static!{
    static ref RNG: Mutex<Lcg128Xsl64> = Mutex::new(rand_pcg::Pcg64::seed_from_u64(42));
}

impl Sphere {
    pub fn new(position: [f32;3], radius: f32)-> Self{
        Self{
            position,
            radius
        }
    }
    pub fn new_rand(a:[f32; 3], b:[f32; 3], rad_min:f32, rad_max:f32)->Self{
        let rng = &mut *RNG.lock().unwrap();
        let x = Uniform::new(a[0], b[0]).sample(rng);
        let y = Uniform::new(a[1], b[1]).sample(rng);
        let z = Uniform::new(a[2], b[2]).sample(rng);
        let radius = Uniform::new(rad_min, rad_max).sample(rng);
        Self{
            position: [x,y,z],
            radius
        }
    }
}