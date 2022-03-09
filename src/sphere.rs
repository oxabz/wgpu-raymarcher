use bytemuck::{Pod, Zeroable};
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Sphere{
    position: [f32;3],
    radius: f32
}

impl Sphere {
    pub fn new(position: [f32;3], radius: f32)-> Self{
        Self{
            position,
            radius
        }
    }
}