use crate::{AssetServer, Entity, FromWorld, Handle, Mat4, Shader, World};
use bevy::ecs::system::lifetimeless::{Read, SQuery};
use bevy::ecs::system::SystemParamItem;
use bevy::pbr::{DrawMesh, MeshPipeline, MeshPipelineKey, SetMeshBindGroup, SetMeshViewBindGroup};
use bevy::prelude::*;
use bevy::render::render_phase::{
    EntityRenderCommand, RenderCommandResult, SetItemPipeline, TrackedRenderPass,
};
use bevy::render::render_resource::std140::AsStd140;
use bevy::render::render_resource::{BindGroup, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BufferBindingType, BufferSize, CompareFunction, DepthBiasState, DynamicUniformVec, PrimitiveState, RenderPipelineDescriptor, ShaderStages, SpecializedPipeline, StencilFaceState, StencilState, TextureFormat};
use bevy::render::renderer::RenderDevice;

pub type SkyboxDrawCustom = (
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMeshBindGroup<1>,
    SetViewExtraBindGroup<2>,
    DrawMesh,
);

pub struct SkyboxPipeline {
    vertex: Handle<Shader>,
    fragment: Handle<Shader>,
    mesh_pipeline: MeshPipeline,
    pub view_extra_uniforms_bind_group_layout: BindGroupLayout,
}

#[derive(Clone, AsStd140)]
pub struct ViewExtraUniform {
    pub untranslated_view: Mat4,
}

#[derive(Default)]
pub struct ViewExtraUniforms {
    pub uniforms: DynamicUniformVec<ViewExtraUniform>,
}

#[derive(Component)]
pub struct ViewExtraUniformOffset {
    pub offset: u32,
}

pub struct SetViewExtraBindGroup<const I: usize>;

#[derive(Clone, Component)]
pub struct ViewExtraBindGroup {
    pub value: BindGroup,
}

impl FromWorld for SkyboxPipeline {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let vertex = asset_server.load("shaders/skybox.vert");
        let fragment = asset_server.load("shaders/skybox.frag");

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
            vertex,
            fragment,
            mesh_pipeline: mesh_pipeline.clone(),
            view_extra_uniforms_bind_group_layout,
        }
    }
}

impl SpecializedPipeline for SkyboxPipeline {
    type Key = MeshPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        let mut descriptor = self.mesh_pipeline.specialize(key);
        descriptor.vertex.shader = self.vertex.clone();
        descriptor.vertex.entry_point = "main".into();
        descriptor.fragment = descriptor.fragment.map(|mut fragment| {
            fragment.entry_point = "main".into();
            fragment.shader = self.fragment.clone();
            fragment
        });
        descriptor.layout = Some(vec![
            self.mesh_pipeline.view_layout.clone(),
            self.mesh_pipeline.mesh_layout.clone(),
            self.view_extra_uniforms_bind_group_layout.clone(),
        ]);
        descriptor.depth_stencil = descriptor.depth_stencil.map(|mut depth_stencil| {
            depth_stencil.depth_write_enabled = true;
            depth_stencil.depth_compare = CompareFunction::LessEqual;
            depth_stencil
        });
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
