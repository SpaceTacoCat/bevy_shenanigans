use crate::{AssetServer, Entity, FromWorld, Handle, Mat4, Shader, World};
use bevy::ecs::system::lifetimeless::{Read, SQuery, SRes};
use bevy::ecs::system::SystemParamItem;
use bevy::pbr::{
    DrawMesh, MaterialPipeline, MeshPipeline, MeshPipelineKey, SetMeshBindGroup,
    SetMeshViewBindGroup,
};
use bevy::prelude::*;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_phase::{
    EntityRenderCommand, RenderCommandResult, SetItemPipeline, TrackedRenderPass,
};
use bevy::render::render_resource::std140::AsStd140;
use bevy::render::render_resource::{
    BindGroup, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType,
    BufferBindingType, BufferSize, CompareFunction, DynamicUniformVec, PrimitiveState,
    RenderPipelineDescriptor, SamplerBindingType, ShaderStages, SpecializedPipeline, Texture,
    TextureSampleType, TextureView, TextureViewDimension,
};
use bevy::render::renderer::RenderDevice;

pub type SkyboxDrawCustom = (
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMeshBindGroup<1>,
    SetExtraBindGroup<2>,
    DrawMesh,
);

pub struct SkyboxPipeline {
    shader: Handle<Shader>,
    pub mesh_pipeline: MeshPipeline,
    pub extra_bind_group_layout: BindGroupLayout,
}

#[derive(Clone, AsStd140)]
pub struct Extras {
    pub untranslated_view: Mat4,
}

#[derive(Default)]
pub struct ViewExtraUniforms {
    pub uniforms: DynamicUniformVec<Extras>,
}

#[derive(Component)]
pub struct ViewExtraUniformOffset {
    pub offset: u32,
}

pub struct SetExtraBindGroup<const I: usize>;

#[derive(Clone, Component)]
pub struct ExtraBindGroup {
    pub value: BindGroup,
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
                        visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: true,
                            min_binding_size: BufferSize::new(Extras::std140_size_static() as u64),
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            sample_type: TextureSampleType::Float { filterable: true },
                            view_dimension: TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 2,
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
            extra_bind_group_layout,
        }
    }
}

impl SpecializedPipeline for SkyboxPipeline {
    type Key = MeshPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        let mut descriptor = self.mesh_pipeline.specialize(key);
        descriptor.vertex.shader = self.shader.clone();
        descriptor.fragment = descriptor.fragment.map(|mut fragment| {
            fragment.shader = self.shader.clone();
            fragment
        });
        descriptor.layout = Some(vec![
            self.mesh_pipeline.view_layout.clone(),
            self.mesh_pipeline.mesh_layout.clone(),
            self.extra_bind_group_layout.clone(),
        ]);
        descriptor.primitive = PrimitiveState {
            unclipped_depth: true,
            ..Default::default()
        };
        descriptor.depth_stencil = descriptor.depth_stencil.map(|mut depth_stencil| {
            depth_stencil.depth_compare = CompareFunction::GreaterEqual;
            depth_stencil
        });
        descriptor
    }
}

impl<const I: usize> EntityRenderCommand for SetExtraBindGroup<I> {
    type Param = SQuery<(Read<ViewExtraUniformOffset>, Read<ExtraBindGroup>)>;

    fn render<'w>(
        view: Entity,
        _item: Entity,
        query: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let Ok((view_extra_uniform, bind_group)) = query.get(view) else { return RenderCommandResult::Failure };
        pass.set_bind_group(I, &bind_group.value, &[view_extra_uniform.offset]);
        RenderCommandResult::Success
    }
}
