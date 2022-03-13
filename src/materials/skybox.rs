use bevy::core_pipeline::Transparent3d;
use bevy::ecs::system::lifetimeless::{Read, SQuery};
use bevy::ecs::system::SystemParamItem;
use bevy::pbr::{
    DrawMesh, MeshPipeline, MeshPipelineKey, MeshUniform, SetMeshBindGroup, SetMeshViewBindGroup,
};
use bevy::prelude::{
    shape, App, AssetServer, Assets, Commands, Component, ComputedVisibility, Entity, FromWorld,
    GlobalTransform, Handle, Image, Mesh, Msaa, Plugin, Query, Res, ResMut, Shader, Visibility,
    With, World,
};
use bevy::render::render_resource::{
    SpecializedMeshPipeline, SpecializedMeshPipelineError, SpecializedMeshPipelines,
};

use bevy::render::mesh::MeshVertexBufferLayout;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_phase::{
    AddRenderCommand, DrawFunctions, EntityRenderCommand, RenderCommandResult, RenderPhase,
    SetItemPipeline, TrackedRenderPass,
};
use bevy::render::render_resource::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, CompareFunction, PrimitiveTopology,
    RenderPipelineCache, RenderPipelineDescriptor, SamplerBindingType, ShaderStages,
    TextureSampleType, TextureViewDimension,
};
use bevy::render::renderer::RenderDevice;
use bevy::render::view::{ExtractedView, NoFrustumCulling};
use bevy::render::{RenderApp, RenderStage};

pub struct SkyboxPlugin;

#[derive(Component, Clone)]
pub struct SkyboxMaterial {
    pub texture: Handle<Image>,
}

#[derive(Default)]
pub struct SkyboxTextureConversionQueue {
    queue: Vec<Handle<Image>>,
}

pub type SkyboxDrawCustom = (
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMeshBindGroup<1>,
    SetMaterialBindGroup<2>,
    DrawMesh,
);

pub struct SkyboxPipeline {
    shader: Handle<Shader>,
    pub mesh_pipeline: MeshPipeline,
    pub material_bind_group_layout: BindGroupLayout,
}

pub struct SetMaterialBindGroup<const I: usize>;

#[derive(Clone, Component)]
pub struct MaterialBindGroup {
    pub value: BindGroup,
}

impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SkyboxTextureConversionQueue>()
            .add_startup_system(setup)
            .add_system(process_skybox_texture_conversion_queue);

        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .add_render_command::<Transparent3d, SkyboxDrawCustom>()
            .init_resource::<SkyboxPipeline>()
            .init_resource::<SpecializedMeshPipelines<SkyboxPipeline>>()
            .add_system_to_stage(RenderStage::Extract, extract_skybox_material)
            .add_system_to_stage(RenderStage::Queue, queue_skybox_pipeline)
            .add_system_to_stage(RenderStage::Queue, queue_view_extra_bind_group);
    }
}

impl SkyboxTextureConversionQueue {
    pub fn add(&mut self, handle: Handle<Image>) {
        self.queue.push(handle);
    }
}

impl FromWorld for SkyboxPipeline {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let shader = asset_server.load("shaders/skybox.wgsl");

        let render_device = world.get_resource_mut::<RenderDevice>().unwrap();
        let extra_bind_group_layout =
            render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("view extra uniforms bind group"),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            sample_type: TextureSampleType::Float { filterable: true },
                            view_dimension: TextureViewDimension::D2Array,
                            multisampled: false,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let mesh_pipeline = world.get_resource::<MeshPipeline>().unwrap();

        SkyboxPipeline {
            shader,
            mesh_pipeline: mesh_pipeline.clone(),
            material_bind_group_layout: extra_bind_group_layout,
        }
    }
}

impl SpecializedMeshPipeline for SkyboxPipeline {
    type Key = MeshPipelineKey;

    fn specialize(
        &self,
        key: Self::Key,
        layout: &MeshVertexBufferLayout,
    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
        let mut descriptor = self.mesh_pipeline.specialize(key, layout)?;
        descriptor.vertex.shader = self.shader.clone();
        descriptor.fragment = descriptor.fragment.map(|mut fragment| {
            fragment.shader = self.shader.clone();
            fragment
        });
        descriptor.layout = Some(vec![
            self.mesh_pipeline.view_layout.clone(),
            self.mesh_pipeline.mesh_layout.clone(),
            self.material_bind_group_layout.clone(),
        ]);
        descriptor.depth_stencil = descriptor.depth_stencil.map(|mut depth_stencil| {
            depth_stencil.depth_compare = CompareFunction::GreaterEqual;
            depth_stencil
        });
        Ok(descriptor)
    }
}

impl<const I: usize> EntityRenderCommand for SetMaterialBindGroup<I> {
    type Param = SQuery<Read<MaterialBindGroup>>;

    fn render<'w>(
        view: Entity,
        _item: Entity,
        query: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let Ok(bind_group) = query.get(view) else { return RenderCommandResult::Failure };
        pass.set_bind_group(I, &bind_group.value, &[]);
        RenderCommandResult::Success
    }
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut conversion_queue: ResMut<SkyboxTextureConversionQueue>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let skybox_texture = asset_server.load("textures/sky.png");
    conversion_queue.add(skybox_texture.clone());

    commands.spawn().insert_bundle((
        meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
        GlobalTransform::default(),
        SkyboxMaterial {
            texture: skybox_texture,
        },
        Visibility::default(),
        ComputedVisibility::default(),
        NoFrustumCulling,
    ));
}

fn process_skybox_texture_conversion_queue(
    mut conversion_queue: ResMut<SkyboxTextureConversionQueue>,
    mut images: ResMut<Assets<Image>>,
) {
    let mut i = 0;
    loop {
        match conversion_queue.queue.get(i) {
            None => break,
            Some(item) => match images.get_mut(item) {
                None => {
                    i += 1;
                }
                Some(image) => {
                    conversion_queue.queue.remove(i);
                    image.reinterpret_stacked_2d_as_array(6);
                }
            },
        }
    }
}

fn queue_view_extra_bind_group(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    skybox_pipeline: Res<SkyboxPipeline>,
    views: Query<Entity, With<ExtractedView>>,
    gpu_images: Res<RenderAssets<Image>>,
    skybox: Query<(Entity, &SkyboxMaterial)>,
) {
    let (_, skybox) = skybox.get_single().unwrap();

    let Some((texture_view, sampler)) = skybox_pipeline
        .mesh_pipeline
        .get_image_texture(gpu_images.as_ref(), &Some(skybox.texture.clone()))
        else { return };

    let group = MaterialBindGroup {
        value: render_device.create_bind_group(&BindGroupDescriptor {
            label: Some("view_extra_bind_group"),
            layout: &skybox_pipeline.material_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(texture_view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(sampler),
                },
            ],
        }),
    };
    for entity in views.iter() {
        commands.entity(entity).insert(group.clone());
    }
}

fn extract_skybox_material(mut commands: Commands, query: Query<(Entity, &SkyboxMaterial)>) {
    let (entity, material) = query.get_single().unwrap();
    commands.get_or_spawn(entity).insert(SkyboxMaterial {
        texture: material.texture.clone(),
    });
}

#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
fn queue_skybox_pipeline(
    transparent_3d_draw_functions: Res<DrawFunctions<Transparent3d>>,
    custom_pipeline: Res<SkyboxPipeline>,
    msaa: Res<Msaa>,
    render_meshes: Res<RenderAssets<Mesh>>,
    mut pipelines: ResMut<SpecializedMeshPipelines<SkyboxPipeline>>,
    mut pipeline_cache: ResMut<RenderPipelineCache>,
    material_meshes: Query<
        (Entity, &Handle<Mesh>, &MeshUniform),
        (With<Handle<Mesh>>, With<SkyboxMaterial>),
    >,
    mut views: Query<(&ExtractedView, &mut RenderPhase<Transparent3d>)>,
) {
    let draw_function = transparent_3d_draw_functions
        .read()
        .get_id::<SkyboxDrawCustom>()
        .unwrap();
    let key = MeshPipelineKey::from_msaa_samples(msaa.samples)
        | MeshPipelineKey::from_primitive_topology(PrimitiveTopology::TriangleList);

    for (view, mut transparent_phase) in views.iter_mut() {
        let view_matrix = view.transform.compute_matrix();
        let view_row_2 = view_matrix.row(2);
        for (entity, mesh_handle, mesh) in material_meshes.iter() {
            let render_mesh = render_meshes.get(mesh_handle).unwrap();
            let pipeline = pipelines
                .specialize(
                    &mut pipeline_cache,
                    &custom_pipeline,
                    key,
                    &render_mesh.layout,
                )
                .unwrap();

            transparent_phase.add(Transparent3d {
                entity,
                pipeline,
                draw_function,
                distance: view_row_2.dot(mesh.transform.col(3)),
            });
        }
    }
}
