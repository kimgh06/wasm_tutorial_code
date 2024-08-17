use gltf::{buffer::Source, Gltf};
use std::io::{BufReader, Cursor};
use wgpu::util::DeviceExt;

use crate::{
    model::{self, ModelVertex},
    resources::{load_binary, load_string, load_texture},
    texture,
};

pub async fn load_model_gltf(
    file_name: &str,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> anyhow::Result<model::Model> {
    let gltf_text = load_string(file_name).await?;
    let gltf_cursor = Cursor::new(gltf_text);
    let gltf_reader = BufReader::new(gltf_cursor);
    let gltf = Gltf::from_reader(gltf_reader)?;

    // Load buffers
    let mut buffer_data = Vec::new();
    for buffer in gltf.buffers() {
        match buffer.source() {
            gltf::buffer::Source::Uri(uri) => {
                let bin = load_binary(uri).await?;
                buffer_data.push(bin);
            }
            gltf::buffer::Source::Bin => {
                // Handle embedded binary data
            }
        }
    }

    // Load meshes and materials
    let mut meshes = Vec::new();
    let mut materials = Vec::new();
    for scene in gltf.scenes() {
        for node in scene.nodes() {
            let mesh = node.mesh().expect("Got mesh");
            let primitives = mesh.primitives();
            for primitive in primitives {
                let reader = primitive.reader(|buffer| Some(&buffer_data[buffer.index()]));

                let mut vertices = Vec::new();
                if let Some(positions) = reader.read_positions() {
                    vertices.extend(positions.map(|pos| ModelVertex {
                        position: pos,
                        tex_coords: Default::default(),
                        normal: Default::default(),
                        tangent: Default::default(),
                        bitangent: Default::default(),
                    }));
                }
                // Handle normals and texture coordinates similarly

                let mut indices = Vec::new();
                if let Some(indices_raw) = reader.read_indices() {
                    indices.extend(indices_raw.into_u32());
                }

                let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(&vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                });

                let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    contents: bytemuck::cast_slice(&indices),
                    usage: wgpu::BufferUsages::INDEX,
                });

                meshes.push(model::Mesh {
                    name: file_name.to_string(),
                    vertex_buffer,
                    index_buffer,
                    num_elements: indices.len() as u32,
                    material: 0, // Placeholder
                });
            }
        }
    }
    // Load materials
    for material in gltf.materials() {
        let pbr = material.pbr_metallic_roughness();
        if let Some(texture) = pbr.base_color_texture() {
            let texture = texture.texture();
            let diffuse_texture = match texture.source() {
                gltf::image::Source::Uri { uri } => load_texture(&uri, true, device, queue).await?,
                gltf::image::Source::View { view, .. } => {
                    let buffer = &buffer_data[view.buffer().index()];
                    texture::Texture::from_bytes(device, queue, buffer, file_name, true)?
                }
            };

            materials.push(model::Material {
                name: material.name().unwrap_or("Default Material").to_string(),
                diffuse_texture,
                bind_group: None.unwrap(), // Adjust this according to your needs
                normal_texture: None.unwrap(), // Adjust this according to your needs
            });
        }
    }

    Ok(model::Model { meshes, materials })
}
