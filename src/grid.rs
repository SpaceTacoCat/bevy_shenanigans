use bevy::core_pipeline::Transparent3d;
use bevy::pbr::{
    DrawMesh, MeshPipeline, MeshPipelineKey, MeshUniform, SetMeshBindGroup, SetMeshViewBindGroup,
};
use bevy::prelude::*;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_component::ExtractComponentPlugin;
use bevy::render::render_phase::{AddRenderCommand, DrawFunctions, RenderPhase, SetItemPipeline};
use bevy::render::render_resource::{
    RenderPipelineCache, RenderPipelineDescriptor, SpecializedPipeline, SpecializedPipelines,
};
use bevy::render::view::ExtractedView;
use bevy::render::{RenderApp, RenderStage};

pub type DrawEndless = (
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMeshBindGroup<1>,
    DrawMesh,
);

pub struct EndlessGridPipeline {
    mesh_pipeline: MeshPipeline,
    shader: Handle<Shader>,
}

pub struct EndlessGridPlugin;

#[derive(Component, Hash< PartialEq, Eq, Copy, Clone)]
pub struct GridPlane;

impl FromWorld for EndlessGridPipeline {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let mesh_pipeline = world.get_resource::<MeshPipeline>().unwrap();
        EndlessGridPipeline {
            mesh_pipeline: mesh_pipeline.clone(),
            shader: asset_server.load("shaders/static_grid.wgsl"),
        }
    }
}

impl SpecializedPipeline for EndlessGridPipeline {
    type Key = MeshPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        let mut descriptor = self.mesh_pipeline.specialize(key);
        descriptor.vertex.shader = self.shader.clone();
        let mut fragment = descriptor.fragment.as_mut().unwrap();
        fragment.shader = self.shader.clone();
        descriptor.layout = Some(vec![
            self.mesh_pipeline.view_layout.clone(),
            self.mesh_pipeline.mesh_layout.clone(),
        ]);
        descriptor
    }
}

impl Plugin for EndlessGridPlugin {
    fn build(&self, app: &mut App) {
        app.sub_app_mut(RenderApp)
            .add_render_command::<Transparent3d, DrawEndless>()
            .init_resource::<EndlessGridPipeline>()
            .init_resource::<SpecializedPipelines<EndlessGridPipeline>>()
            .add_system_to_stage(RenderStage::Queue, queue_custom);
    }
}

pub fn queue_custom(
    t3ddf: Res<DrawFunctions<Transparent3d>>,
    rm: Res<RenderAssets<Mesh>>,
    custom_pipeline: Res<EndlessGridPipeline>,
    msaa: Res<Msaa>,
    mut pipelines: ResMut<SpecializedPipelines<EndlessGridPipeline>>,
    mut pipeline_cache: ResMut<RenderPipelineCache>,
    mm: Query<(Entity, &Handle<Mesh>, &MeshUniform, &GridPlane)>,
    mut views: Query<(&ExtractedView, &mut RenderPhase<Transparent3d>)>,
) {
    let draw_custom = t3ddf.read().get_id::<DrawEndless>().unwrap();

    let key = MeshPipelineKey::from_msaa_samples(msaa.samples);

    for (view, mut tp) in views.iter_mut() {
        let vm = view.transform.compute_matrix();
        let vr2 = vm.row(2);
        for (entity, mh, mu) in mm.iter() {
            if let Some(mesh) = rm.get(mh) {
                let key = key | MeshPipelineKey::from_primitive_topology(mesh.primitive_topology);
                let pipeline = pipelines.specialize(&mut pipeline_cache, &custom_pipeline, key);
                tp.add(Transparent3d {
                    distance: vr2.dot(mu.transform.col(3)),
                    pipeline,
                    entity,
                    draw_function: draw_custom,
                })
            }
        }
    }
}
