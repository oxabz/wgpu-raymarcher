use std::thread::sleep;
use std::time::Duration;
use pollster::block_on;
use wgpu::{AddressMode, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindingResource, Buffer, BufferDescriptor, BufferUsages, ComputePassDescriptor, ComputePipeline, Device, Extent3d, FilterMode, IndexFormat, PipelineLayoutDescriptor, Queue, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, RequestAdapterOptions, SamplerDescriptor, ShaderModuleDescriptor, Surface, SurfaceConfiguration, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureViewDescriptor, TextureViewDimension, VertexBufferLayout};
use wgpu::BindingType::Texture;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::window::Window;
use crate::camera::CameraManager;
use crate::color::Color;
use crate::shapes::ShapeCollection;
use crate::shapes::sphere::Sphere;

const WORKGROUP_SIZE_X: u32 = 16;
const WORKGROUP_SIZE_Y: u32 = 16;
const TARGET_TEXTURE_X: u32 = 800;
const TARGET_TEXTURE_Y: u32 = 800;

pub struct AppState {
    surface: Surface,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    pub(crate) size: PhysicalSize<u32>,

    render_pipeline: ComputePipeline,
    copy_pipeline: RenderPipeline,

    indices_buffer:Buffer,
    vertices_buffer:Buffer,

    target_texture_bind_group: BindGroup,
    copied_texture_bind_group: BindGroup,

    shape_collection: ShapeCollection,
    camera_manager: CameraManager
}

impl AppState {
    pub async fn new(window: &Window) -> Self {
        println!("Start");
        // Getting the size
        let size = window.inner_size();

        // Initializing the wgpu context
        let (surface, device, queue, config) = Self::wgpu_init(window).await;
        println!("WGPU Initiated");

        // Defining and setting up the render pipeline
        let (render_pipeline, target_texture_bind_group_layout) = Self::init_render_pipeline(&device);
        println!("Render pipeline created");

        // Defining and setting up the pipeline that display the result of the render pipeline
        let (copy_pipeline, copied_texture_bind_group_layout) = Self::init_copy_pipeline(&device,&config);
        println!("Copy pipeline created");

        // Creating the vertex buffer and the index buffer
        let (vertices_buffer, indices_buffer) = Self::init_vertices(&device);
        println!("Copy buffer created");

        // Create texture to render to
        let target_texture = device.create_texture(&TextureDescriptor{
            label: Some("Target texture"),
            size: Extent3d{
                width: TARGET_TEXTURE_X,
                height: TARGET_TEXTURE_Y,
                depth_or_array_layers: 1
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            usage: TextureUsages::TEXTURE_BINDING|TextureUsages::STORAGE_BINDING
        });

        let write_view = target_texture.create_view(&TextureViewDescriptor{
            label: Some("Target Write View"),
            format: Some(TextureFormat::Rgba8Unorm),
            dimension: Some(TextureViewDimension::D2),
            aspect: Default::default(),
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None
        });

        let target_texture_bind_group = device.create_bind_group(&BindGroupDescriptor{
            label: Some("Target Texture Bind Group"),
            layout: &target_texture_bind_group_layout,
            entries: &[BindGroupEntry{ binding: 0, resource: BindingResource::TextureView(&write_view) }]
        });


        let read_view = target_texture.create_view(&TextureViewDescriptor{
            label: Some("Target Read View"),
            format: Some(TextureFormat::Rgba8Unorm),
            dimension: Some(TextureViewDimension::D2),
            aspect: Default::default(),
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None
        });

        let sampler = device.create_sampler(&SamplerDescriptor{
            label: Some("Sampler"),
            address_mode_u: AddressMode::MirrorRepeat,
            address_mode_v: AddressMode::MirrorRepeat,
            address_mode_w: AddressMode::MirrorRepeat,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: Default::default(),
            lod_min_clamp: 0.0,
            lod_max_clamp: 0.0,
            compare: None,
            anisotropy_clamp: None,
            border_color: None
        });

        let copied_texture_bind_group = device.create_bind_group(&BindGroupDescriptor{
            label: Some("Copy Texture group"),
            layout: &copied_texture_bind_group_layout,
            entries: &[
                BindGroupEntry{ binding: 0, resource: BindingResource::TextureView(&read_view) },
                BindGroupEntry{ binding: 1, resource: BindingResource::Sampler(&sampler) }
            ]
        });



        let mut shape_collection = ShapeCollection::new(&device);
        for _ in 0..20 {
            shape_collection.add_sphere(Sphere::new_rand([-10.0, -10.0, 20.0], [10.0, 10.0, 60.0], 0.1, 2.0), Color::random(), 0.0);
        }

        shape_collection.add_sphere(Sphere::new([0.0, 0.0, 30.0], 3.0), Color(0.2,0.2,0.2), 0.9);
        shape_collection.add_sphere(Sphere::new([0.0, 4.0, 25.0], 1.0), Color(0.2,0.2,0.2), 0.9);

        shape_collection.update_buffers(&queue);

        let mut camera_manager = CameraManager::new(&device,size.clone());
        camera_manager.update_buffers(&queue);

        Self {
            surface,
            device,
            queue,
            config,
            size,

            render_pipeline,
            copy_pipeline,

            indices_buffer,
            vertices_buffer,

            target_texture_bind_group,
            copied_texture_bind_group,

            shape_collection,
            camera_manager
        }
    }

    pub(crate) fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            println!("resizes");
            self.camera_manager.set_size(new_size);
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub(crate) fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }


    pub(crate) fn update(&mut self) {

    }

    pub(crate) fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        //Surface texture
        let output =  match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(_) => {
                self.surface.configure(&self.device, &self.config);
                println!("Reconfigured");
                self.surface
                    .get_current_texture()?
            }
        };

        self.camera_manager.update_buffers(&self.queue);

        //Setup
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let color_attachments = [wgpu::RenderPassColorAttachment {
            view:&view,
            resolve_target: None,
            ops: wgpu::Operations {
                // Not clearing here in order to test wgpu's zero texture initialization on a surface texture.
                // Users should avoid loading uninitialized memory since this can cause additional overhead.
                load: wgpu::LoadOp::Load,
                store: true,
            },
        }];

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor{
            label:Some("Render Encoder")
        });
        {
            let mut compute_pass = encoder.begin_compute_pass(&ComputePassDescriptor{ label: None });
            compute_pass.set_pipeline(&self.render_pipeline);
            compute_pass.set_bind_group(0,&self.target_texture_bind_group,&[]);
            compute_pass.set_bind_group(1, self.shape_collection.bind_group(),&[]);
            compute_pass.set_bind_group(2, self.camera_manager.bind_group(),&[]);
            compute_pass.dispatch(TARGET_TEXTURE_X/WORKGROUP_SIZE_X, TARGET_TEXTURE_Y/WORKGROUP_SIZE_Y, 1)

        }
        {
            let mut compute_pass = encoder.begin_render_pass(&RenderPassDescriptor{
                label: Some("Render Pass"),
                color_attachments: &color_attachments,
                depth_stencil_attachment: None,
            });
            compute_pass.set_pipeline(&self.copy_pipeline);
            compute_pass.set_bind_group(0,&self.copied_texture_bind_group,&[]);
            compute_pass.set_vertex_buffer(0,self.vertices_buffer.slice(..));
            compute_pass.set_index_buffer(self.indices_buffer.slice(..),IndexFormat::Uint16);
            compute_pass.draw_indexed(0..6,0,0..1)
        }
        let done = self.queue.on_submitted_work_done();
        self.queue.submit(Some(encoder.finish()));
        output.present();
        block_on(done);
        Ok(())
    }

    async fn wgpu_init(window: &Window) -> (Surface, Device, Queue, SurfaceConfiguration) {
        // WGPU Boiler plate
        let instance = wgpu::Instance::new(wgpu::Backends::all());

        let (size, surface) = unsafe {
            let size = window.inner_size();
            let surface = instance.create_surface(&window);
            (size, surface)
        };

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        }).await.unwrap();
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                limits: wgpu::Limits::default()
                    .using_resolution(adapter.limits()),
                label: None,
            },
            None, // Trace path
        ).await.unwrap();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };
        surface.configure(&device, &config);
        (surface, device, queue, config)
    }

    fn init_render_pipeline(device:&Device) -> (ComputePipeline, BindGroupLayout) {
        let target_texture_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor{
                label: Some("Target Texture Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::StorageTexture {
                            access: wgpu::StorageTextureAccess::WriteOnly,
                            format: wgpu::TextureFormat::Rgba8Unorm,
                            view_dimension: TextureViewDimension::D2
                        },
                        count: None,
                    },
                ]
            });


        let shapes_bind_group = ShapeCollection::bind_group_layout(&device);
        let camera_bind_group = CameraManager::bind_group_layout(&device);

        let compute_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor{
            label: Some("Ray Marcher Layout"),
            bind_group_layouts: &[&target_texture_bind_group_layout, &shapes_bind_group, &camera_bind_group],
            push_constant_ranges: &[]
        });

        let compute_shader = device.create_shader_module(&ShaderModuleDescriptor{
            label: Some("Ray Marcher Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("raymarcher.wgsl").into())
        });

        let render_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor{
            label: Some("Ray Marcher Pipeline"),
            layout: Some(&compute_pipeline_layout),
            module: &compute_shader,
            entry_point: "render"
        });

        (render_pipeline, target_texture_bind_group_layout)
    }

    fn init_copy_pipeline(device:&Device, config: &SurfaceConfiguration)->(RenderPipeline, BindGroupLayout) {
        let transfer_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor{
            label: Some("Copied Texture Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float {filterable:true},
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(
                        wgpu::SamplerBindingType::Filtering,
                    ),
                    count: None,
                },
            ]
        });

        let copy_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor{
            label: Some("Copy Layout"),
            bind_group_layouts: &[&transfer_bind_group_layout],
            push_constant_ranges: &[]
        });

        let copy_shader = device.create_shader_module(&ShaderModuleDescriptor{
            label: Some("Copy Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("copy.wgsl").into())
        });

        let vertex_buffer_layout = VertexBufferLayout{
            array_stride: 2*std::mem::size_of::<f32>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x2],
        };

        let copy_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor{
            label: Some("Copy Pipeline"),
            layout: Some(&copy_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &copy_shader,
                entry_point: "vs_main",
                buffers: &[vertex_buffer_layout],
            },
            fragment: Some(wgpu::FragmentState {
                module: &copy_shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        (copy_pipeline, transfer_bind_group_layout)
    }

    fn init_vertices(device:&Device) -> (Buffer, Buffer){
        const VERTECES: &[f32; 8] = &[
            -1.0,-1.0,
            -1.0,1.0,
            1.0,1.0,
            1.0,-1.0
        ];
        const INDICES: &[u16; 6] = &[
            0,2,1,
            3,2,0,
        ];


        let vertices_buffer = device.create_buffer_init(&BufferInitDescriptor{
            label: Some("Vertices Buffer"),
            contents: bytemuck::bytes_of(VERTECES),
            usage: BufferUsages::VERTEX,
        });
        let indices_buffer = device.create_buffer_init(&BufferInitDescriptor{
            label: Some("Indices Buffer"),
            contents: bytemuck::bytes_of(INDICES),
            usage: BufferUsages::INDEX,
        });

        (vertices_buffer,indices_buffer)
    }
}