use std::sync::Mutex;
use bytemuck::{Pod, Zeroable};
use lazy_static::lazy_static;
use rand_pcg::Lcg128Xsl64;
use rand::distributions::{Distribution, Uniform};
use rand::SeedableRng;

lazy_static!{
    static ref RNG: Mutex<Lcg128Xsl64> = Mutex::new(rand_pcg::Pcg64::seed_from_u64(42));
    static ref COLOR_DISTRIB : Uniform<f32> = Uniform::from(0.0 .. 1.0);
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Color(pub f32, pub f32, pub f32);

impl Color {
    pub fn random()->Self{
        let rng = &mut *RNG.lock().unwrap();
        Self(COLOR_DISTRIB.sample(rng), COLOR_DISTRIB.sample(rng), COLOR_DISTRIB.sample(rng))
    }
}