use std::sync::Mutex;
use bytemuck::{Pod, Zeroable};
use lazy_static::lazy_static;
use rand_pcg::Lcg128Xsl64;
use rand::distributions::{Distribution, Uniform};
use rand::SeedableRng;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Cuboid{
    position: [f32;3],
    _pad1:[f32;1],
    scaling: [f32;3],
    _pad2:[f32;1],
}

lazy_static!{
    static ref RNG: Mutex<Lcg128Xsl64> = Mutex::new(rand_pcg::Pcg64::seed_from_u64(42));
}

impl Cuboid {
    pub fn new(position: [f32;3], scaling: [f32;3])-> Self{
        Self{
            position,
            scaling,
            _pad1:[0.0],
            _pad2:[0.0],
        }
    }
    pub fn new_rand(a:[f32; 3], b:[f32; 3], c:[f32; 3], d:[f32; 3])->Self{
        let rng = &mut *RNG.lock().unwrap();
        let x = Uniform::new(a[0], b[0]).sample(rng);
        let y = Uniform::new(a[1], b[1]).sample(rng);
        let z = Uniform::new(a[2], b[2]).sample(rng);
        let sx = Uniform::new(c[0], d[0]).sample(rng);
        let sy = Uniform::new(c[1], d[1]).sample(rng);
        let sz = Uniform::new(c[2], d[2]).sample(rng);
        Self{
            position: [x,y,z],
            _pad1: [0.0],
            scaling: [sx, sy, sz],
            _pad2: [0.0]
        }
    }
}