use bevy::core_pipeline::Transparent3d;
use bevy::ecs::query::QueryItem;
use bevy::ecs::system::lifetimeless::SRes;
use bevy::ecs::system::SystemParamItem;
use bevy::pbr::{
    DrawMesh, MaterialPipeline, MeshPipeline, MeshPipelineKey, MeshUniform, SetMeshBindGroup,
    SetMeshViewBindGroup, SpecializedMaterial,
};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_asset::{PrepareAssetError, RenderAsset, RenderAssets};
use bevy::render::render_component::{ExtractComponent, ExtractComponentPlugin};
use bevy::render::render_phase::{AddRenderCommand, DrawFunctions, RenderPhase, SetItemPipeline};
use bevy::render::render_resource::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BufferInitDescriptor, BufferUsages, CompareFunction, RenderPipelineCache, RenderPipelineDescriptor, SpecializedPipeline, SpecializedPipelines, StencilState};
use bevy::render::renderer::RenderDevice;
use bevy::render::view::ExtractedView;
use bevy::render::{RenderApp, RenderStage};
use crate::skybox::mesh::SkyboxMesh;

pub mod mesh;

type DrawSkybox = (
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMeshBindGroup<1>,
    DrawMesh,
);

pub struct SkyboxPlugin;

#[derive(Component, Copy, Clone)]
pub struct Skybox;

pub struct SkyPipeline {
    mesh_pipeline: MeshPipeline,
    shader: Handle<Shader>,
}

impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ExtractComponentPlugin::<Skybox>::default());
        app.sub_app_mut(RenderApp)
            .add_render_command::<Transparent3d, DrawSkybox>()
            .init_resource::<SkyPipeline>()
            .init_resource::<SpecializedPipelines<SkyPipeline>>()
            .add_system_to_stage(RenderStage::Queue, queue_custom);
    }
}

impl ExtractComponent for Skybox {
    type Query = &'static Skybox;
    type Filter = ();

    fn extract_component(item: QueryItem<Self::Query>) -> Self {
        *item
    }
}

impl FromWorld for SkyPipeline {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let mesh_pipeline = world.get_resource::<MeshPipeline>().unwrap();
        let shader = asset_server.load("shaders/skybox.wgsl");
        SkyPipeline {
            mesh_pipeline: mesh_pipeline.clone(),
            shader,
        }
    }
}

impl SpecializedPipeline for SkyPipeline {
    type Key = MeshPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        let mut descriptor = self.mesh_pipeline.specialize(key);

        descriptor.vertex.shader = self.shader.clone();
        let fragment = descriptor.fragment.as_mut().unwrap();
        fragment.shader = self.shader.clone();
        descriptor.layout = Some(vec![
            self.mesh_pipeline.view_layout.clone(),
            self.mesh_pipeline.mesh_layout.clone(),
        ]);
        let mut depth_stencil = descriptor.depth_stencil.as_mut().unwrap();
        depth_stencil.depth_compare = CompareFunction::LessEqual;
        descriptor
    }
}

pub fn queue_custom(
    transparent_3d_draw_functions: Res<DrawFunctions<Transparent3d>>,
    msaa: Res<Msaa>,
    mut views: Query<(&ExtractedView, &mut RenderPhase<Transparent3d>)>,
    material_meshes: Query<(Entity, &Handle<Mesh>, &MeshUniform), With<Skybox>>,
    render_meshes: Res<RenderAssets<Mesh>>,
    mut pipelines: ResMut<SpecializedPipelines<SkyPipeline>>,
    mut pipeline_cache: ResMut<RenderPipelineCache>,
    custom_pipeline: Res<SkyPipeline>,
) {
    let draw_custom = transparent_3d_draw_functions
        .read()
        .get_id::<DrawSkybox>()
        .unwrap();
    let key = MeshPipelineKey::from_msaa_samples(msaa.samples);
    for (view, mut transparent_phase) in views.iter_mut() {
        let view_matrix = view.transform.compute_matrix();
        let view_row_2 = view_matrix.row(2);
        for (entity, mesh_handle, mesh_uniform) in material_meshes.iter() {
            if let Some(mesh) = render_meshes.get(mesh_handle) {
                let key = key | MeshPipelineKey::from_primitive_topology(mesh.primitive_topology);
                let pipeline = pipelines.specialize(&mut pipeline_cache, &custom_pipeline, key);
                transparent_phase.add(Transparent3d {
                    entity,
                    pipeline,
                    draw_function: draw_custom,
                    distance: view_row_2.dot(mesh_uniform.transform.col(3)),
                })
            }
        }
    }
}
