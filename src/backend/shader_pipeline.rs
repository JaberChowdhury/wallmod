use image::RgbaImage;
use std::borrow::Cow;
use wgpu::util::DeviceExt;

/// Applies a WebGPU (WGSL) compute shader to the given image.
/// The shader must have a compute entry point named `main`.
/// The shader must declare a storage buffer for the image pixels:
/// `@group(0) @binding(0) var<storage, read_write> pixels: array<u32>;`
/// It should also expect a uniform for image dimensions:
/// `@group(0) @binding(1) var<uniform> dimensions: vec2<u32>;`
/// `@group(0) @binding(2) var<uniform> params: vec4<f32>;`
pub async fn run_compute_shader(
    input_image: &RgbaImage,
    wgsl_source: &str,
    params: [f32; 4],
) -> Option<RgbaImage> {
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await?;

    let (device, queue) =
        adapter.request_device(&wgpu::DeviceDescriptor::default(), None).await.ok()?;

    let (width, height) = input_image.dimensions();
    let size = (width * height * 4) as wgpu::BufferAddress;

    // Buffer to hold the image pixels
    let pixel_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Pixel Buffer"),
        contents: input_image.as_raw(),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
    });

    // Buffer to hold the image dimensions
    let dims = [width, height];
    let dimensions_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Dimensions Buffer"),
        contents: bytemuck::cast_slice(&dims),
        usage: wgpu::BufferUsages::UNIFORM,
    });

    // Buffer to hold the params (vec4<f32>)
    let params_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Params Buffer"),
        contents: bytemuck::cast_slice(&params),
        usage: wgpu::BufferUsages::UNIFORM,
    });

    // Staging buffer to read results back to CPU
    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Staging Buffer"),
        size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    // Compile shader
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Custom Compute Shader"),
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(wgsl_source)),
    });

    // Bind group layout
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Bind Group Layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage {
                        read_only: false,
                    },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Bind Group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: pixel_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: dimensions_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: params_buffer.as_entire_binding(),
            },
        ],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Compute Pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: "main",
    });

    // Encode commands
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: None,
    });
    {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: None,
            timestamp_writes: None,
        });
        cpass.set_pipeline(&compute_pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);

        let workgroup_x = (width + 15) / 16;
        let workgroup_y = (height + 15) / 16;
        cpass.dispatch_workgroups(workgroup_x, workgroup_y, 1);
    }

    // Copy to staging buffer
    encoder.copy_buffer_to_buffer(&pixel_buffer, 0, &staging_buffer, 0, size);

    // Submit and await completion
    queue.submit(Some(encoder.finish()));

    let buffer_slice = staging_buffer.slice(..);
    let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
    buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

    device.poll(wgpu::Maintain::Wait);

    if receiver.receive().await.is_some() {
        let data = buffer_slice.get_mapped_range();
        let result = RgbaImage::from_raw(width, height, data.to_vec());
        drop(data);
        staging_buffer.unmap();
        return result;
    }

    None
}
#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbaImage;

    #[tokio::test]
    async fn test_shader_compilation_and_execution() {
        let img = RgbaImage::new(100, 100);
        let shader = crate::backend::shaders::get_shader("CRT Scanlines").unwrap();
        let res = run_compute_shader(&img, shader, [1.0, 1.0, 1.0, 1.0]).await;
        assert!(res.is_some(), "Shader pipeline failed!");
    }
}
