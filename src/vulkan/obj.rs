use std::mem::size_of;

use anyhow::{Ok, Result};
use cgmath::{vec2, vec3};
use vulkanalia::prelude::v1_0::*;

use crate::vulkan::{AppData, app::{copy_buffer, create_buffer}};
use std::ptr::copy_nonoverlapping as memcpy;


pub type Vec2 = cgmath::Vector2<f32>;
pub type Vec3 = cgmath::Vector3<f32>;
pub type Mat4 = cgmath::Matrix4<f32>;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pos: Vec2,
    color: Vec3,
}

impl Vertex {
    pub const fn new(pos: Vec2, color: Vec3) -> Self {
        Self { pos, color }
    }

    pub fn binding_description() -> vk::VertexInputBindingDescription {
        vk::VertexInputBindingDescription::builder()
            .binding(0)
            .stride(size_of::<Vertex>() as u32)
            .input_rate(vk::VertexInputRate::VERTEX)
            .build()
    }

    pub fn attribute_descriptions() -> [vk::VertexInputAttributeDescription; 2] {
        let pos = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(0)
            .format(vk::Format::R32G32_SFLOAT)
            .offset(0)
            .build();
        let color = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(1)
            .format(vk::Format::R32G32B32_SFLOAT)
            .offset(size_of::<Vec2>() as u32)
            .build();
        [pos, color]
    }
}

pub unsafe fn create_vertex_buffer(
    instance: &Instance,
    device: &Device,    
    data: &mut AppData,
) -> Result<()> {
    let size = (size_of::<Vertex>() * VERTICES.len()) as u64;
    
    let (staging_buffer, staging_buffer_memory) = create_buffer(
        instance,
        device,
        data,
        size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
    )?;


    let memory = device.map_memory(
        staging_buffer_memory,
        0,
        size,
        vk::MemoryMapFlags::empty(),
    )?;

    memcpy(VERTICES.as_ptr(), memory.cast(), VERTICES.len());

    device.unmap_memory(staging_buffer_memory);

    let (vertex_buffer, vertex_buffer_memory) = create_buffer(
        instance,
        device,
        data,
        size,
        vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::VERTEX_BUFFER,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;

    copy_buffer(device, data, staging_buffer, vertex_buffer, size)?;

    data.vertex_buffer = vertex_buffer;
    data.vertex_buffer_memory = vertex_buffer_memory;

    device.destroy_buffer(staging_buffer, None);
    device.free_memory(staging_buffer_memory, None);

    Ok(())
}

pub unsafe fn create_index_buffer(
    instance: &Instance,
    device: &Device,
    data: &mut AppData,
) -> Result<()> {
    let size = (size_of::<u16>() * INDICES.len()) as u64;

    let (staging_buffer, staging_buffer_memory) = create_buffer(
        instance,
        device,
        data,
        size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
    )?;

    let memory = device.map_memory(
        staging_buffer_memory,
        0,
        size,
        vk::MemoryMapFlags::empty(),
    )?;

    memcpy(INDICES.as_ptr(), memory.cast(), INDICES.len());

    device.unmap_memory(staging_buffer_memory);

    let (index_buffer, index_buffer_memory) = create_buffer(
        instance,
        device,
        data,
        size,
        vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::INDEX_BUFFER,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;

    data.index_buffer = index_buffer;
    data.index_buffer_memory = index_buffer_memory;

    copy_buffer(device, data, staging_buffer, index_buffer, size)?;

    device.destroy_buffer(staging_buffer, None);
    device.free_memory(staging_buffer_memory, None);

    Ok(())
}

// Square formation
pub const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];
pub static mut VERTICES: [Vertex; 4] = [
    Vertex::new(vec2(0.0, 0.0), vec3(1.0, 0.0, 0.0)),
    Vertex::new(vec2(1.0, 0.0), vec3(0.0, 1.0, 0.0)),
    Vertex::new(vec2(1.0, 1.0), vec3(0.0, 0.0, 1.0)),
    Vertex::new(vec2(0.0, 1.0), vec3(1.0, 1.0, 1.0)),
];

// pub const INDICES: &[u16] = &[
//     // front (+Z)
//     0, 1, 2, 2, 3, 0,
//     // back (-Z)
//     5, 4, 7, 7, 6, 5,
//     // left (-X)
//     4, 0, 3, 3, 7, 4,
//     // right (+X)
//     1, 5, 6, 6, 2, 1,
//     // top (+Y)
//     3, 2, 6, 6, 7, 3,
//     // bottom (-Y)
//     4, 5, 1, 1, 0, 4,
// ];
// pub static mut VERTICES: [Vertex; 8] = [
//     Vertex::new(vec3(-1.0, -1.0,  1.0), vec3(1.0, 0.0, 0.0)), // 0
//     Vertex::new(vec3( 1.0, -1.0,  1.0), vec3(0.0, 1.0, 0.0)), // 1
//     Vertex::new(vec3( 1.0,  1.0,  1.0), vec3(0.0, 0.0, 1.0)), // 2
//     Vertex::new(vec3(-1.0,  1.0,  1.0), vec3(1.0, 1.0, 0.0)), // 3
//     Vertex::new(vec3(-1.0, -1.0, -1.0), vec3(1.0, 0.0, 1.0)), // 4
//     Vertex::new(vec3( 1.0, -1.0, -1.0), vec3(0.0, 1.0, 1.0)), // 5
//     Vertex::new(vec3( 1.0,  1.0, -1.0), vec3(1.0, 1.0, 1.0)), // 6
//     Vertex::new(vec3(-1.0,  1.0, -1.0), vec3(0.5, 0.5, 0.5)), // 7
// ];

pub unsafe fn update_vertex_buffer(
    instance: &Instance,
    device: &Device,    
    data: &mut AppData,
) -> Result<()> {

   let size = (size_of::<Vertex>() * VERTICES.len()) as u64;

    // 1. Create a temporary CPU-visible staging buffer
    let (staging_buffer, staging_buffer_memory) = create_buffer(
        instance,
        device,
        data,
        size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_VISIBLE
            | vk::MemoryPropertyFlags::HOST_COHERENT,
    )?;

    // 2. Map the staging memory
    let memory = device.map_memory(
        staging_buffer_memory,
        0,
        size,
        vk::MemoryMapFlags::empty(),
    )?;

    // 3. Copy the vertices into the staging buffer
    memcpy(
        VERTICES.as_ptr(),
        memory.cast(),
        size as usize,
    );

    // 4. Unmap
    device.unmap_memory(staging_buffer_memory);

    // 5. Copy staging buffer → EXISTING vertex buffer
    copy_buffer(
        device,
        data,
        staging_buffer,
        data.vertex_buffer,
        size,
    )?;

    // 6. Destroy temporary staging resources
    device.destroy_buffer(staging_buffer, None);
    device.free_memory(staging_buffer_memory, None);

    Ok(())
}


#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct UniformBufferObject {
    pub model: Mat4,
    pub view: Mat4,
    pub proj: Mat4,
}

// Creates one uniform buffer per swapchain image
pub unsafe fn create_uniform_buffers(
    instance: &Instance,
    device: &Device,
    data: &mut AppData,
) -> Result<()> {
    data.uniform_buffers.clear();
    data.uniform_buffers_memory.clear();

    for _ in 0..data.swapchain_images.len() {
        let (uniform_buffer, uniform_buffer_memory) = create_buffer(
            instance,
            device,
            data,
            size_of::<UniformBufferObject>() as u64,
            vk::BufferUsageFlags::UNIFORM_BUFFER,
            vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE
        )?;
        
        data.uniform_buffers.push(uniform_buffer);
        data.uniform_buffers_memory.push(uniform_buffer_memory);
    }

    Ok(())
}

