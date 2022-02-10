use crate::{App, World};
use bevy::app::Plugin;
use bevy::core_pipeline::draw_3d_graph;
use bevy::render::render_graph::{NodeRunError, RenderGraph, RenderGraphContext};
use bevy::render::renderer::RenderContext;
use bevy::render::{render_graph, RenderApp};

// TODO: Wait for https://github.com/bevyengine/bevy/pull/3552/ before implementing post-process shaders

pub struct VignetteShaderPlugin;
pub struct PostProcessPassNode;

const POST_PROCESS_PASS: &str = "post_process_pass";

impl Plugin for VignetteShaderPlugin {
    fn build(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        let mut render_graph = render_app.world.get_resource_mut::<RenderGraph>().unwrap();
        let main_pass = render_graph.get_sub_graph(draw_3d_graph::NAME);
        render_graph.add_node(POST_PROCESS_PASS, PostProcessPassNode);
        render_graph.add_node_edge(draw_3d_graph::node::MAIN_PASS, POST_PROCESS_PASS);
    }
}

impl render_graph::Node for PostProcessPassNode {
    fn run(
        &self,
        graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        todo!()
    }
}
