use std::f32::consts::PI;
use wgpu::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferDescriptor, BufferSize, BufferUsages, Device, Queue, ShaderStages};
use winit::dpi::PhysicalSize;
use bytemuck::{Zeroable,Pod};

#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
struct CameraUniform {
    ray_dir_mat:[[f32;4];3],
    ratio: f32,
    depth:f32,
    _pad:[f32;2]
}

pub struct CameraManager{
    dirty:bool,
    size:PhysicalSize<u32>,

    angle: f32,
    screen_depth: f32,

    camera_uniform:Buffer,
    camera_bind_group:BindGroup
}

impl CameraManager {
    pub fn new(device: &Device, size : PhysicalSize<u32>)->Self{
        let camera_uniform = Self::init_buffers(device);
        let bind_group_layout = Self::bind_group_layout(device);

        let camera_bind_group = device.create_bind_group(&BindGroupDescriptor{
            label: None,
            layout: &bind_group_layout,
            entries: &[
                BindGroupEntry{ binding: 0, resource: camera_uniform.as_entire_binding() }
            ]
        });

        Self{ dirty: false, size, angle:(PI), screen_depth: 2.0, camera_uniform, camera_bind_group }
    }

    pub fn set_size(&mut self, size : PhysicalSize<u32>){
        self.dirty = true;
        self.size = size;
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
                        min_binding_size: BufferSize::new(std::mem::size_of::<CameraUniform>() as u64)
                    },
                    count:None
                }
            ]
        };
        device.create_bind_group_layout(&bind_group_layout)
    }

    fn init_buffers(device:&Device)->wgpu::Buffer{
        device.create_buffer(&BufferDescriptor{
            label: Some("Camera Uniform"),
            size: std::mem::size_of::<CameraUniform>() as u64,
            usage: BufferUsages::UNIFORM|BufferUsages::COPY_DST,
            mapped_at_creation: false
        })
    }

    pub fn update_buffers(&mut self, queue:&Queue){
        if self.dirty {
            queue.write_buffer(&self.camera_uniform, 0 , bytemuck::bytes_of(&self.generate_uniform()));
            self.dirty = false;
        }
    }

    fn generate_uniform(&self)->CameraUniform{
        let forward = self.forward();
        let up = self.up();
        let ratio = self.aspect_ratio();
        let right = self.right();


        CameraUniform{
            ray_dir_mat: [
                [right[0], right[1], right[2], 0.0],
                [up[0], up[1], up[2], 0.0],
                [forward[0], forward[1], forward[2], 0.0]
            ],
            ratio,
            depth: self.screen_depth,
            _pad: [0.0, 0.0]
        }
    }

    pub fn bind_group(&self)->&BindGroup{&self.camera_bind_group}

    pub fn angle(&self)->f32{
        self.angle
    }

    pub fn set_angle(&mut self, angle:f32){
        self.angle = angle;
        self.dirty = true;
    }

    pub fn forward(&self)->[f32;3]{
        [self.angle.cos(), 0.0, self.angle.sin()]
    }

    pub fn right(&self)->[f32;3]{
        [(self.angle-PI/2.0).cos(), 0.0, (self.angle-PI/2.0).sin()]
    }

    pub fn up(&self)-> [f32;3]{
        [0.0, 1.0, 0.0]
    }

    pub fn aspect_ratio(&self) -> f32{
        self.size.width as f32 / self.size.height as f32
    }

    pub fn screen_depth(&self) -> f32{
        self.screen_depth
    }

    pub fn set_screen_depth(&mut self, depth:f32){
        self.screen_depth = depth;
    }

}