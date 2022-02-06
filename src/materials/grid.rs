use bevy::ecs::system::lifetimeless::SRes;
use bevy::ecs::system::SystemParamItem;
use bevy::pbr::{MaterialPipeline, SpecializedMaterial};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_asset::{PrepareAssetError, RenderAsset};
use bevy::render::render_resource::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, Buffer, CompareFunction, RenderPipelineDescriptor};
use bevy::render::renderer::RenderDevice;
use bevy::render::view::NoFrustumCulling;

pub struct GridPlugin;

#[derive(Clone, Component, Debug, TypeUuid)]
#[uuid = "0541ce52-293a-49c2-9694-11e5ffdd9204"]
pub struct GridMaterial;

pub struct GpuGridMaterial {
    bind_group: BindGroup,
}

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MaterialPlugin::<GridMaterial>::default())
            .add_startup_system(setup);
    }
}

impl RenderAsset for GridMaterial {
    type ExtractedAsset = GridMaterial;
    type PreparedAsset = GpuGridMaterial;
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
        Ok(GpuGridMaterial { bind_group })
    }
}

impl SpecializedMaterial for GridMaterial {
    type Key = ();

    fn key(_: &<Self as RenderAsset>::PreparedAsset) -> Self::Key {
        ()
    }

    fn specialize(key: Self::Key, descriptor: &mut RenderPipelineDescriptor) {
        let depth_stencil = descriptor.depth_stencil.as_mut().unwrap();
        depth_stencil.depth_compare = CompareFunction::GreaterEqual;
        depth_stencil.depth_write_enabled = true;
    }

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
        Some(asset_server.load("shaders/grid.wgsl"))
    }

    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/grid.wgsl"))
    }

    fn alpha_mode(_: &<Self as RenderAsset>::PreparedAsset) -> AlphaMode {
        AlphaMode::Blend
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GridMaterial>>,
) {
    commands.spawn_bundle(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Quad {
            size: Vec2::new(2.0, 2.0),
            flip: false,
        })),
        material: materials.add(GridMaterial),
        ..Default::default()
    }).insert(NoFrustumCulling);
}
