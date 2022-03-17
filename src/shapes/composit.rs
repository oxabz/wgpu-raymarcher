use crate::shapes::cuboid::Cuboid;
use crate::shapes::sphere::Sphere;
use bytemuck::{Pod,Zeroable};
use crate::shapes::ShapeProperties;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Composit{
    a: u32,
    b: u32,
    comp_type:u32,
    alpha:f32
}

impl Composit {
    pub(crate) fn new(a: u32, b: u32, comp_type: u32, alpha:f32) -> Self {
        Self{a,b,comp_type, alpha }
    }
}

pub enum CompositDescriptor{
    CUBOID(Cuboid, ShapeProperties),
    SPHERE(Sphere, ShapeProperties),
    UNION(Box<CompositDescriptor>,Box<CompositDescriptor>),
    BLEND(Box<CompositDescriptor>,Box<CompositDescriptor>, f32),
    INTERSECTION(Box<CompositDescriptor>,Box<CompositDescriptor>),
    DIFFERENCE(Box<CompositDescriptor>,Box<CompositDescriptor>)
}