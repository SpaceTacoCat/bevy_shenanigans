use bevy::core_pipeline::Transparent3d;
use bevy::pbr::{
    DrawMesh, MeshPipeline, MeshPipelineKey, MeshUniform, SetMeshBindGroup, SetMeshViewBindGroup,
};
use bevy::prelude::*;
use bevy::render::render_phase::{AddRenderCommand, DrawFunctions, RenderPhase, SetItemPipeline};
use bevy::render::render_resource::{
    PrimitiveTopology, RenderPipelineCache, RenderPipelineDescriptor, SpecializedPipeline,
    SpecializedPipelines,
};
use bevy::render::renderer::RenderDevice;
use bevy::render::view::ExtractedView;
use bevy::render::{RenderApp, RenderStage};
use std::iter;

type SkyboxDrawCustom = (
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMeshBindGroup<1>,
    DrawMesh,
);

pub struct SkyboxPlugin;

#[derive(Component)]
pub struct SkyboxMaterial;

struct SkyboxPipeline {
    shader: Handle<Shader>,
    mesh_pipeline: MeshPipeline,
}

impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .add_render_command::<Transparent3d, SkyboxDrawCustom>()
            .init_resource::<SkyboxPipeline>()
            .init_resource::<SpecializedPipelines<SkyboxPipeline>>()
            .add_system_to_stage(RenderStage::Extract, extract_skybox_material)
            .add_system_to_stage(RenderStage::Queue, queue_custom);
    }
}

impl FromWorld for SkyboxPipeline {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let shader = asset_server.load("shaders/skybox.wgsl");

        let mesh_pipeline = world.get_resource::<MeshPipeline>().unwrap();

        SkyboxPipeline {
            shader,
            mesh_pipeline: mesh_pipeline.clone(),
        }
    }
}

impl SpecializedPipeline for SkyboxPipeline {
    type Key = MeshPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        let mut descriptor = self.mesh_pipeline.specialize(key);
        descriptor.vertex.shader = self.shader.clone();
        descriptor.fragment.as_mut().unwrap().shader = self.shader.clone();
        descriptor.layout = Some(vec![
            self.mesh_pipeline.view_layout.clone(),
            self.mesh_pipeline.mesh_layout.clone(),
        ]);
        descriptor
    }
}

fn extract_skybox_material(mut commands: Commands, query: Query<Entity, With<SkyboxMaterial>>) {
    commands
        .get_or_spawn(query.get_single().expect("Only one skybox entity allowed"))
        .insert_bundle((SkyboxMaterial,));
}

fn queue_custom(
    transparent_3d_draw_functions: Res<DrawFunctions<Transparent3d>>,
    custom_pipeline: Res<SkyboxPipeline>,
    msaa: Res<Msaa>,
    mut pipelines: ResMut<SpecializedPipelines<SkyboxPipeline>>,
    mut pipeline_cache: ResMut<RenderPipelineCache>,
    material_meshes: Query<(Entity, &MeshUniform), (With<Handle<Mesh>>, With<SkyboxMaterial>)>,
    mut views: Query<(&ExtractedView, &mut RenderPhase<Transparent3d>)>,
) {
    let draw_function = transparent_3d_draw_functions
        .read()
        .get_id::<SkyboxDrawCustom>()
        .unwrap();
    let key = MeshPipelineKey::from_msaa_samples(msaa.samples)
        | MeshPipelineKey::from_primitive_topology(PrimitiveTopology::TriangleList);
    let pipeline = pipelines.specialize(&mut pipeline_cache, &custom_pipeline, key);

    for (view, mut transparent_phase) in views.iter_mut() {
        let view_matrix = view.transform.compute_matrix();
        let view_row_2 = view_matrix.row(2);
        for (entity, mesh) in material_meshes.iter() {
            transparent_phase.add(Transparent3d {
                entity,
                pipeline,
                draw_function,
                distance: view_row_2.dot(mesh.transform.col(3)),
            });
        }
    }
}
