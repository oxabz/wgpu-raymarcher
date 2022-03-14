use std::f32::consts::PI;
use wgpu::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferDescriptor, BufferSize, BufferUsages, Device, Queue, ShaderStages};
use winit::dpi::PhysicalSize;
use bytemuck::{Zeroable,Pod};

#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
struct CameraUniform {
    ray_dir_mat:[[f32;4];3]
}

pub struct CameraManager{
    dirty:bool,
    size:PhysicalSize<u32>,

    angle: f32,
    fov: f32,

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

        Self{ dirty: false, size, angle:(PI), fov: 0.2, camera_uniform, camera_bind_group }
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
        let forward = [self.angle.cos(), 0.0, self.angle.sin(),0.0];
        //let forward = [0.0, 0.0, 1.0,0.0];
        let up = [0.0, 1.0 , 0.0,0.0];
        let ratio = self.size.width as f32 / self.size.height as f32;
        //let right = [forward&[0]*up[0]*ratio, forward[1]*up[1]*ratio, forward[2]*up[2]*ratio, 0.0];
        //let right = [1.0, 0.0, 0.0,0.0];
        let right = [(self.angle-PI/2.0).cos() / ratio, 0.0, (self.angle-PI/2.0).sin() / ratio, 0.0];


        CameraUniform{
            ray_dir_mat: [right, up, forward]
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
}