pub mod sphere;
pub mod cuboid;

use bytemuck::{Pod, Zeroable};
use wgpu::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferDescriptor, BufferSize, BufferUsages, Device, Queue, ShaderStages};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use crate::color::Color;
use crate::shapes::cuboid::Cuboid;
use crate::shapes::sphere::Sphere;


const SHAPE_CAPACITY: u64 = 64;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Shape {
    color: Color,
    index:u32,
    shape_type: u32,
    reflectivity: f32,
    _pad:[f32;2]
}



#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ShapeCount (u32);

impl Shape {
    pub fn new( color: Color, shape_type:u32, index:u32, reflectivity:f32 )->Self{
        Self{
            color,
            shape_type,
            index,
            reflectivity,
            _pad: [0.0;2]
        }
    }
}

pub struct ShapeCollection {
    shapes: Vec<Shape>,
    spheres: Vec<Sphere>,
    cuboids: Vec<Cuboid>,
    dirty: bool,

    count_uniform: wgpu::Buffer,
    shapes_buffer: wgpu::Buffer,
    spheres_buffer: wgpu::Buffer,
    cuboids_buffer: wgpu::Buffer,

    bind_group: wgpu::BindGroup
}

impl ShapeCollection {
    pub fn new(device: &Device)->Self{
        let (count_uniform,shapes_buffer,spheres_buffer, cuboids_buffer) = Self::create_buffers(device);

        let bind_group_layout = Self::bind_group_layout(device);
        let bind_group = device.create_bind_group(&BindGroupDescriptor{
            label: Some("ShapesBindGroup"),
            layout: &bind_group_layout,
            entries: &[
                BindGroupEntry{
                    binding: 0,
                    resource: count_uniform.as_entire_binding()
                },
                BindGroupEntry{
                    binding: 1,
                    resource: shapes_buffer.as_entire_binding()
                },
                BindGroupEntry{
                    binding: 2,
                    resource: spheres_buffer.as_entire_binding()
                },
                BindGroupEntry{
                    binding: 3,
                    resource: cuboids_buffer.as_entire_binding()
                }
            ]
        });

        Self{ shapes: vec![], spheres: vec![], cuboids: vec![], dirty: false, count_uniform, shapes_buffer, spheres_buffer, cuboids_buffer, bind_group }
    }

    pub fn add_sphere(&mut self, sphere:Sphere, color: Color, reflectivity:f32){
        let index = self.spheres.len() as u32;
        self.spheres.push(sphere);
        self.shapes.push(Shape::new(color, 0, index, reflectivity));
        self.dirty = true;
    }

    pub fn add_cube(&mut self, cuboid:Cuboid, color: Color, reflectivity:f32){
        let index = self.cuboids.len() as u32;
        self.cuboids.push(cuboid);
        self.shapes.push(Shape::new(color, 1, index, reflectivity));
        self.dirty = true;
    }

    fn create_buffers(device: &Device) -> (Buffer, Buffer, Buffer, Buffer) {
        let count_uniform = device.create_buffer_init(&BufferInitDescriptor{
            label: Some("CountUniform"),
            contents: bytemuck::cast_slice(&[ShapeCount(0)]),
            usage: BufferUsages::UNIFORM|BufferUsages::COPY_DST
        });
        let shapes_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("ShapeBuffer"),
            size:std::mem::size_of::<Shape>() as u64 * SHAPE_CAPACITY,
            usage: BufferUsages::STORAGE|BufferUsages::COPY_DST,
            mapped_at_creation: false
        });
        let spheres_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("SphereBuffer"),
            size:std::mem::size_of::<Sphere>() as u64 * SHAPE_CAPACITY,
            usage: BufferUsages::STORAGE|BufferUsages::COPY_DST,
            mapped_at_creation: false
        });
        let cuboids_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("CuboidBuffer"),
            size:std::mem::size_of::<Cuboid>() as u64 * SHAPE_CAPACITY,
            usage: BufferUsages::STORAGE|BufferUsages::COPY_DST,
            mapped_at_creation: false
        });
        (count_uniform, shapes_buffer,spheres_buffer, cuboids_buffer)
    }

    pub fn update_buffers(&mut self, queue:&Queue){
        if self.dirty {
            queue.write_buffer(&self.count_uniform, 0 , bytemuck::bytes_of(&ShapeCount(self.shapes.len() as u32)));
            queue.write_buffer(&self.shapes_buffer, 0 , self.shapes_bytes().as_slice());
            queue.write_buffer(&self.spheres_buffer, 0 , self.sphere_bytes().as_slice());
            queue.write_buffer(&self.cuboids_buffer, 0 , self.cuboids_bytes().as_slice());
            self.dirty = false;
        }
    }

    pub fn shapes_bytes(&self) -> Vec<u8>{
        self.shapes.iter().flat_map(|x|bytemuck::bytes_of(x)).map(|x|*x).collect::<Vec<_>>()
    }

    pub fn sphere_bytes(&self) -> Vec<u8>{
        self.spheres.iter().flat_map(|x|bytemuck::bytes_of(x)).map(|x|*x).collect::<Vec<_>>()
    }

    pub fn cuboids_bytes(&self) -> Vec<u8>{
        self.cuboids.iter().flat_map(|x|bytemuck::bytes_of(x)).map(|x|*x).collect::<Vec<_>>()
    }

    pub fn bind_group(&self) -> &BindGroup{
        &self.bind_group
    }

    pub fn bind_group_layout(device:&Device) -> wgpu::BindGroupLayout{
        let bind_group_layout = wgpu::BindGroupLayoutDescriptor {
            label: Some("ShapesBindGroupLayout"),
            entries: &[
                BindGroupLayoutEntry{
                    binding: 0,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(std::mem::size_of::<ShapeCount>() as u64)
                    },
                    count:None
                },
                BindGroupLayoutEntry{
                    binding: 1,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(std::mem::size_of::<Shape>() as u64)
                    },
                    count:None
                },
                BindGroupLayoutEntry{
                    binding: 2,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size:BufferSize::new(std::mem::size_of::<Sphere>() as u64)
                    },
                    count: None
                },
                BindGroupLayoutEntry{
                    binding: 3,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size:BufferSize::new(std::mem::size_of::<Cuboid>() as u64)
                    },
                    count: None
                },
            ]
        };
        device.create_bind_group_layout(&bind_group_layout)
    }
}
