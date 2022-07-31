use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_asset::RenderAssets,
        render_resource::{
            AsBindGroup, AsBindGroupError, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
            BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType,
            BufferBindingType, BufferInitDescriptor, BufferSize, BufferUsages,
            OwnedBindingResource, PreparedBindGroup, SamplerBindingType, ShaderRef, ShaderStages,
            TextureAspect, TextureFormat, TextureViewDescriptor, TextureViewDimension,
        },
        renderer::RenderDevice,
    },
};

// This is the struct that will be passed to your shader
#[derive(Debug, Clone, TypeUuid)]
#[uuid = "4ee9c361-1124-4113-890e-197d82b00321"]
pub struct CubemapMaterial {
    pub normal: Vec4,
    pub texture: Handle<Image>,
    pub needs_conversion: bool,
}

impl Material for CubemapMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/emissive_material.wgsl".into()
    }
}

impl AsBindGroup for CubemapMaterial {
    type Data = ();

    fn as_bind_group(
        &self,
        layout: &BindGroupLayout,
        render_device: &RenderDevice,
        images: &RenderAssets<Image>,
        _fallback_image: &bevy::render::texture::FallbackImage,
    ) -> Result<
        bevy::render::render_resource::PreparedBindGroup<Self>,
        bevy::render::render_resource::AsBindGroupError,
    > {
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            contents: bytemuck::cast_slice(self.normal.as_ref()),
            label: None,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let image = images
            .get(&self.texture)
            .ok_or(AsBindGroupError::RetryNextUpdate)?;

        let texture_view = image.texture.create_view(&TextureViewDescriptor {
            label: None,
            format: Some(TextureFormat::Rgba8UnormSrgb),
            dimension: Some(TextureViewDimension::Cube),
            aspect: TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None,
        });

        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&texture_view),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::Sampler(&image.sampler),
                },
            ],
            label: None,
            layout,
        });

        Ok(PreparedBindGroup {
            bind_group,
            bindings: vec![
                OwnedBindingResource::TextureView(image.texture_view.clone()),
                OwnedBindingResource::Sampler(image.sampler.clone()),
            ],
            data: (),
        })
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(0),
                    },
                    count: None,
                },
                // Emissive Texture
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        multisampled: false,
                        sample_type: bevy::render::render_resource::TextureSampleType::Float {
                            filterable: true,
                        },
                        view_dimension: bevy::render::render_resource::TextureViewDimension::Cube,
                    },
                    count: None,
                },
                // Emissive Texture Sampler
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: None,
        })
    }
}
