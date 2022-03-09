use std::io::{Bytes, Read};
use std::num::NonZeroU32;
use bytemuck::{Contiguous, Pod, Zeroable};
use wgpu::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutEntry, BindingResource, BindingType, BufferBinding, BufferBindingType, BufferDescriptor, BufferSize, BufferUsages, Device, Queue, ShaderStages};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use crate::sphere::Sphere;

const SHAPE_CAPACITY: u64 = 32;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Shape {
    color: [f32;3],
    shape_type: u32,
    index:u32,
    _pad:[f32;3]
}

impl Shape {
    pub fn new( color: [f32;3], shape_type:u32, index:u32 )->Self{
        Self{
            color,
            shape_type,
            index,
            _pad: [0.0;3]
        }
    }
}

pub struct ShapeCollection {
    shapes: Vec<Shape>,
    spheres: Vec<Sphere>,
    dirty: bool,

    shapes_buffer: wgpu::Buffer,
    spheres_buffer: wgpu::Buffer,

    bind_group: wgpu::BindGroup
}

impl ShapeCollection {
    pub fn new(device: &Device)->Self{
        let (shapes_buffer,spheres_buffer) = Self::create_buffers(device);

        let bind_group_layout = Self::bind_group_layout(device);
        let bind_group = device.create_bind_group(&BindGroupDescriptor{
            label: Some("ShapesBindGroup"),
            layout: &bind_group_layout,
            entries: &[
                BindGroupEntry{
                    binding: 0,
                    resource: shapes_buffer.as_entire_binding()
                },
                BindGroupEntry{
                    binding: 1,
                    resource: spheres_buffer.as_entire_binding()
                }
            ]
        });

        Self{ shapes: vec![], spheres: vec![], dirty: false, shapes_buffer, spheres_buffer, bind_group }
    }

    pub fn add_sphere(&mut self, sphere:Sphere, color: [f32;3]){
        let index = self.spheres.len() as u32;
        self.spheres.push(sphere);
        self.shapes.push(Shape::new(color, 0, index));
        self.dirty = true;
    }

    fn create_buffers(device: &Device)->(wgpu::Buffer, wgpu::Buffer){
        let shapes_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("ShapeBuffer"),
            size:std::mem::size_of::<Shape>() as u64 * SHAPE_CAPACITY,
            usage: BufferUsages::STORAGE|BufferUsages::COPY_DST,
            mapped_at_creation: false
        });
        let spheres_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("ShapeBuffer"),
            size:std::mem::size_of::<Sphere>() as u64 * SHAPE_CAPACITY,
            usage: BufferUsages::STORAGE|BufferUsages::COPY_DST,
            mapped_at_creation: false
        });
        (shapes_buffer,spheres_buffer)
    }

    pub fn update_buffers(&mut self, queue:&Queue){
        if self.dirty {
            queue.write_buffer(&self.shapes_buffer, 0 , self.shapes_bytes().as_slice());
            queue.write_buffer(&self.spheres_buffer, 0 , self.sphere_bytes().as_slice());
            self.dirty = false;
        }
    }

    pub fn shapes_bytes(&self) -> Vec<u8>{
        self.shapes.iter().flat_map(|x|bytemuck::bytes_of(x)).map(|x|*x).collect::<Vec<_>>()
    }

    pub fn sphere_bytes(&self) -> Vec<u8>{
        self.spheres.iter().flat_map(|x|bytemuck::bytes_of(x)).map(|x|*x).collect::<Vec<_>>()
    }

    pub fn bind_group(&self) -> &BindGroup{
        &self.bind_group
    }

    pub fn bind_group_layout(device:&Device) -> (wgpu::BindGroupLayout){
        let bind_group_layout = wgpu::BindGroupLayoutDescriptor {
            label: Some("ShapesBindGroupLayout"),
            entries: &[
                BindGroupLayoutEntry{
                    binding: 0,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(std::mem::size_of::<Shape>() as u64)
                    },
                    count:None
                },
                BindGroupLayoutEntry{
                    binding: 1,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size:BufferSize::new(std::mem::size_of::<Sphere>() as u64)
                    },
                    count: None
                }
            ]
        };
        device.create_bind_group_layout(&bind_group_layout)
    }
}