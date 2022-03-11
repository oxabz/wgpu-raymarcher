use wgpu::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferDescriptor, BufferSize, BufferUsages, Device, Queue, ShaderStages};
use winit::dpi::PhysicalSize;
use bytemuck::{Zeroable,Pod};

#[repr(C)]
#[derive(Copy, Clone, Debug, Zeroable, Pod)]
struct CameraUniform {
    ratio:f32
}

pub struct CameraManager{
    dirty:bool,
    size:PhysicalSize<u32>,

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

        Self{ dirty: false, size, camera_uniform, camera_bind_group }
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
        CameraUniform{
            ratio: self.size.height as f32/self.size.width as f32
        }
    }

    pub fn bind_group(&self)->&BindGroup{&self.camera_bind_group}
}