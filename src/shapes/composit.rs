use crate::shapes::cuboid::Cuboid;
use crate::shapes::sphere::Sphere;
use bytemuck::{Pod,Zeroable};
use crate::shapes::ShapeProperties;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Composit{
    a: u32,
    b: u32,
    comp_type:u32
}

impl Composit {
    pub(crate) fn new(a: u32, b: u32, comp_type: u32) -> Self {
        Self{a,b,comp_type}
    }
}

pub enum CompositDescriptor{
    CUBOID(Cuboid, ShapeProperties),
    SPHERE(Sphere, ShapeProperties),
    UNION(Box<CompositDescriptor>,Box<CompositDescriptor>),
    INTERSECTION(Box<CompositDescriptor>,Box<CompositDescriptor>),
    DIFFERENCE(Box<CompositDescriptor>,Box<CompositDescriptor>)
}