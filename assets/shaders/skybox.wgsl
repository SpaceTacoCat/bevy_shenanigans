#import bevy_pbr::mesh_view_bind_group
#import bevy_pbr::mesh_struct

[[group(1), binding(0)]]
var<uniform> r_mesh: Mesh;

struct Vertex {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] uv: vec3<f32>;
};

[[stage(vertex)]]
fn vertex(vertex: Vertex) -> VertexOutput {
    var view_proj = view.view_proj;
    view_proj.w = vec4<f32>(0.0, 0.0, 0.0, 1.0);

    var out: VertexOutput;

    out.position = (view.view_proj * vec4<f32>(vertex.position, 1.0)).xyww;
    out.uv = vec3<f32>(vertex.position.xy, -vertex.position.z);

    return out;
}

[[stage(fragment)]]
fn fragment() -> [[location(0)]] vec4<f32> {
    var color = vec4<f32>(1.0, 1.0, 1.0, 1.0);

    return color;
}
