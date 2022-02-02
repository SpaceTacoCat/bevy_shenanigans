use crate::skybox::pipeline::{
    SkyboxDrawCustom, SkyboxPipeline, ViewExtraBindGroup, ViewExtraUniform, ViewExtraUniformOffset,
    ViewExtraUniforms,
};
use crate::MainCamera;
use bevy::core_pipeline::Transparent3d;
use bevy::pbr::{MeshPipelineKey, MeshUniform};
use bevy::prelude::*;
use bevy::render::render_phase::{AddRenderCommand, DrawFunctions, RenderPhase};
use bevy::render::render_resource::{
    BindGroupDescriptor, BindGroupEntry, PrimitiveTopology, RenderPipelineCache,
    SpecializedPipelines,
};
use bevy::render::renderer::{RenderDevice, RenderQueue};
use bevy::render::view::ExtractedView;
use bevy::render::{RenderApp, RenderStage};

pub mod pipeline;
pub mod shape;

pub struct SkyboxPlugin;

#[derive(Component)]
pub struct SkyboxMaterial;

impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::Update, move_skybox_with_camera);

        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .add_render_command::<Transparent3d, SkyboxDrawCustom>()
            .init_resource::<ViewExtraUniforms>()
            .init_resource::<SkyboxPipeline>()
            .init_resource::<SpecializedPipelines<SkyboxPipeline>>()
            .add_system_to_stage(RenderStage::Extract, extract_skybox_material)
            .add_system_to_stage(RenderStage::Prepare, prepare_view_extra_uniforms)
            .add_system_to_stage(RenderStage::Queue, queue_skybox_pipeline)
            .add_system_to_stage(RenderStage::Queue, queue_view_extra_bind_group);
    }
}

fn move_skybox_with_camera(
    mut q_skybox: Query<&mut Transform, With<SkyboxMaterial>>,
    q_camera: Query<&Transform, (With<MainCamera>, Without<SkyboxMaterial>)>,
) {
    if let Ok(camera) = q_camera.get_single() {
        for mut skybox in q_skybox.iter_mut() {
            skybox.translation = camera.translation;
        }
    }
}

fn queue_view_extra_bind_group(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    skybox_pipeline: Res<SkyboxPipeline>,
    view_extra_uniforms: Res<ViewExtraUniforms>,
    views: Query<Entity, With<ExtractedView>>,
) {
    if let Some(binding) = view_extra_uniforms.uniforms.binding() {
        let group = ViewExtraBindGroup {
            value: render_device.create_bind_group(&BindGroupDescriptor {
                label: Some("view_extra_bind_group"),
                layout: &skybox_pipeline.view_extra_uniforms_bind_group_layout,
                entries: &[BindGroupEntry {
                    binding: 0,
                    resource: binding.clone(),
                }],
            }),
        };
        for entity in views.iter() {
            commands.entity(entity).insert(group.clone());
        }
    }
}

fn prepare_view_extra_uniforms(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    mut view_extra_uniforms: ResMut<ViewExtraUniforms>,
    views: Query<(Entity, &ExtractedView)>,
) {
    view_extra_uniforms.uniforms.clear();
    for (entity, camera) in views.iter() {
        let view = camera.transform.compute_matrix();
        let (scale, rotation, _) = view.to_scale_rotation_translation();
        let untranslated_cam = Mat4::from_scale_rotation_translation(scale, rotation, Vec3::ZERO);
        let view_extra_uniforms = ViewExtraUniformOffset {
            offset: view_extra_uniforms.uniforms.push(ViewExtraUniform {
                untranslated_view: untranslated_cam.inverse(),
            }),
        };

        commands.entity(entity).insert(view_extra_uniforms);
    }

    view_extra_uniforms
        .uniforms
        .write_buffer(&render_device, &render_queue);
}

fn extract_skybox_material(mut commands: Commands, query: Query<Entity, With<SkyboxMaterial>>) {
    commands
        .get_or_spawn(query.get_single().expect("Only one skybox entity allowed"))
        .insert_bundle((SkyboxMaterial,));
}

#[allow(clippy::type_complexity)]
fn queue_skybox_pipeline(
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
