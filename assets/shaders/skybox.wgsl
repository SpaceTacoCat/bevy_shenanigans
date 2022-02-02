#import bevy_pbr::mesh_view_bind_group
#import bevy_pbr::mesh_struct

struct ViewExtra {
    untranslated_view: mat4x4<f32>;
};

struct Vertex {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] uv: vec3<f32>;
};

[[group(1), binding(0)]]
var<uniform> g_mesh: Mesh;

[[group(2), binding(0)]]
var<uniform> g_extra: ViewExtra;

[[stage(vertex)]]
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    out.position = (view.projection * g_extra.untranslated_view * g_mesh.model * vec4<f32>(1000.0 * vertex.position, 1.0)).xyww;
    out.uv = vertex.position;

    return out;
}

[[stage(fragment)]]
fn fragment() -> [[location(0)]] vec4<f32> {
    var color = vec4<f32>(1.0, 1.0, 1.0, 1.0);

    return color;
}
