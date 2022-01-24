use bevy::ecs::system::lifetimeless::SRes;
use bevy::ecs::system::SystemParamItem;
use bevy::pbr::{MaterialPipeline, MeshUniform, SpecializedMaterial};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_asset::{PrepareAssetError, RenderAsset};
use bevy::render::render_resource::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BufferInitDescriptor, BufferUsages, RenderPipelineDescriptor};
use bevy::render::renderer::RenderDevice;

#[derive(Debug, Clone, TypeUuid)]
#[uuid = "3c671922-a3fa-42dd-bd5e-ea486fa7d472"]
pub struct SkyMaterial;

pub struct GpuSkyMaterial {
    bind_group: BindGroup,
}

impl RenderAsset for SkyMaterial {
    type ExtractedAsset = SkyMaterial;
    type PreparedAsset = GpuSkyMaterial;
    type Param = (SRes<RenderDevice>, SRes<MaterialPipeline<Self>>);

    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        _: Self::ExtractedAsset,
        (render_device, material_pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &material_pipeline.material_layout,
            entries: &[],
        });
        Ok(GpuSkyMaterial { bind_group })
    }
}

impl Material for SkyMaterial {
    fn bind_group(material: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
        &material.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[],
        })
    }

    fn vertex_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/skybox.wgsl"))
    }

    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/skybox.wgsl"))
    }
}

pub fn setup_quad(
    mut commands: Commands,
    mut materials: ResMut<Assets<SkyMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {

    commands.spawn_bundle(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())),
        material: materials.add(SkyMaterial),
        ..Default::default()
    });
}
