use bevy::core_pipeline::Transparent3d;
use bevy::ecs::system::lifetimeless::{Read, SQuery, SRes};
use bevy::ecs::system::SystemParamItem;
use bevy::pbr::{
    DrawMesh, MeshPipeline, MeshPipelineKey, MeshUniform, SetMeshBindGroup, SetMeshViewBindGroup,
};
use bevy::prelude::*;
use bevy::render::render_component::DynamicUniformIndex;
use bevy::render::render_phase::{
    AddRenderCommand, DrawFunctions, EntityRenderCommand, RenderCommandResult, RenderPhase,
    SetItemPipeline, TrackedRenderPass,
};
use bevy::render::render_resource::std140::AsStd140;
use bevy::render::render_resource::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BufferBindingType, BufferSize, CompareFunction, DynamicUniformVec, FrontFace, PrimitiveTopology, RenderPipelineCache, RenderPipelineDescriptor, ShaderStages, SpecializedPipeline, SpecializedPipelines, TextureFormat};
use bevy::render::renderer::{RenderDevice, RenderQueue};
use bevy::render::view::ExtractedView;
use bevy::render::{RenderApp, RenderStage};
use std::iter;

type SkyboxDrawCustom = (
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMeshBindGroup<1>,
    SetViewExtraBindGroup<2>,
    DrawMesh,
);

pub struct SkyboxPlugin;

#[derive(Component)]
pub struct SkyboxMaterial;

struct SkyboxPipeline {
    shader: Handle<Shader>,
    mesh_pipeline: MeshPipeline,
    view_extra_uniforms_bind_group_layout: BindGroupLayout,
}

#[derive(Clone, AsStd140)]
struct ViewExtraUniform {
    view_proj: Mat4,
}

#[derive(Default)]
struct ViewExtraUniforms {
    pub uniforms: DynamicUniformVec<ViewExtraUniform>,
}

#[derive(Component)]
struct ViewExtraUniformOffset {
    pub offset: u32,
}

struct SetViewExtraBindGroup<const I: usize>;

#[derive(Clone, Component)]
struct ViewExtraBindGroup {
    value: BindGroup,
}

impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut App) {
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

impl FromWorld for SkyboxPipeline {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let shader = asset_server.load("shaders/skybox.wgsl");

        let render_device = world.get_resource_mut::<RenderDevice>().unwrap();
        let view_extra_uniforms_bind_group_layout =
            render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("view extra uniforms bind group"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        min_binding_size: BufferSize::new(
                            ViewExtraUniform::std140_size_static() as u64
                        ),
                    },
                    count: None,
                }],
            });

        let mesh_pipeline = world.get_resource::<MeshPipeline>().unwrap();

        SkyboxPipeline {
            shader,
            mesh_pipeline: mesh_pipeline.clone(),
            view_extra_uniforms_bind_group_layout,
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
            self.view_extra_uniforms_bind_group_layout.clone(),
        ]);
        let depth_stencil = descriptor.depth_stencil.as_mut().unwrap();
        depth_stencil.depth_compare = CompareFunction::LessEqual;
        depth_stencil.depth_write_enabled = false;
        depth_stencil.format = TextureFormat::Depth32Float;
        descriptor.primitive.front_face = FrontFace::Cw;
        descriptor
    }
}

impl<const I: usize> EntityRenderCommand for SetViewExtraBindGroup<I> {
    type Param = SQuery<(Read<ViewExtraUniformOffset>, Read<ViewExtraBindGroup>)>;

    fn render<'w>(
        view: Entity,
        _item: Entity,
        query: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let (view_extra_uniform, bind_group) = query.get(view).unwrap();
        pass.set_bind_group(I, &bind_group.value, &[view_extra_uniform.offset]);
        RenderCommandResult::Success
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
        let mut view = camera.transform.compute_matrix();
        let view_extra_uniforms = ViewExtraUniformOffset {
            offset: view_extra_uniforms.uniforms.push(ViewExtraUniform {
                view_proj: camera.projection * view.inverse(),
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
