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

fn inverse(m: mat4x4<f32>) -> mat4x4<f32> {
    let det = determinant(m);
    var result = mat4x4<f32>(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

    result.x.x = m.y.y  * m.z.z * m.w.w -
                 m.y.y  * m.z.w * m.w.z -
                 m.z.y  * m.y.z  * m.w.w +
                 m.z.y  * m.y.w  * m.w.z +
                 m.w.y * m.y.z  * m.z.w -
                 m.w.y * m.y.w  * m.z.z;

    result.y.x = -m.y.x  * m.z.z * m.w.w +
                  m.y.x  * m.z.w * m.w.z +
                  m.z.x  * m.y.z  * m.w.w -
                  m.z.x  * m.y.w  * m.w.z -
                  m.w.x * m.y.z  * m.z.w +
                  m.w.x * m.y.w  * m.z.z;

    result.z.x = m.y.x  * m.z.y * m.w.w -
                 m.y.x  * m.z.w * m.w.y -
                 m.z.x  * m.y.y * m.w.w +
                 m.z.x  * m.y.w * m.w.y +
                 m.w.x * m.y.y * m.z.w -
                 m.w.x * m.y.w * m.z.y;

    result.w.x = -m.y.x  * m.z.y * m.w.z +
                   m.y.x  * m.z.z * m.w.y +
                   m.z.x  * m.y.y * m.w.z -
                   m.z.x  * m.y.z * m.w.y -
                   m.w.x * m.y.y * m.z.z +
                   m.w.x * m.y.z * m.z.y;

    result.x.y = -m.x.y  * m.z.z * m.w.w +
                    m.x.y  * m.z.w * m.w.z +
                    m.z.y  * m.x.z * m.w.w -
                    m.z.y  * m.x.w * m.w.z -
                    m.w.y * m.x.z * m.z.w +
                    m.w.y * m.x.w * m.z.z;

    result.y.y = m.x.x  * m.z.z * m.w.w -
                 m.x.x  * m.z.w * m.w.z -
                 m.z.x  * m.x.z * m.w.w +
                 m.z.x  * m.x.w * m.w.z +
                 m.w.x * m.x.z * m.z.w -
                 m.w.x * m.x.w * m.z.z;

    result.z.y = -m.x.x  * m.z.y * m.w.w +
                  m.x.x  * m.z.w * m.w.y +
                  m.z.x  * m.x.y * m.w.w -
                  m.z.x  * m.x.w * m.w.y -
                  m.w.x * m.x.y * m.z.w +
                  m.w.x * m.x.w * m.z.y;

    result.w.y = m.x.x  * m.z.y * m.w.z -
                  m.x.x  * m.z.z * m.w.y -
                  m.z.x  * m.x.y * m.w.z +
                  m.z.x  * m.x.z * m.w.y +
                  m.w.x * m.x.y * m.z.z -
                  m.w.x * m.x.z * m.z.y;

    result.x.z = m.x.y  * m.y.z * m.w.w -
                 m.x.y  * m.y.w * m.w.z -
                 m.y.y  * m.x.z * m.w.w +
                 m.y.y  * m.x.w * m.w.z +
                 m.w.y * m.x.z * m.y.w -
                 m.w.y * m.x.w * m.y.z;

    result.y.z = -m.x.x  * m.y.z * m.w.w +
                  m.x.x  * m.y.w * m.w.z +
                  m.y.x  * m.x.z * m.w.w -
                  m.y.x  * m.x.w * m.w.z -
                  m.w.x * m.x.z * m.y.w +
                  m.w.x * m.x.w * m.y.z;

    result.z.z = m.x.x  * m.y.y * m.w.w -
                  m.x.x  * m.y.w * m.w.y -
                  m.y.x  * m.x.y * m.w.w +
                  m.y.x  * m.x.w * m.w.y +
                  m.w.x * m.x.y * m.y.w -
                  m.w.x * m.x.w * m.y.y;

    result.w.z = -m.x.x  * m.y.y * m.w.z +
                   m.x.x  * m.y.z * m.w.y +
                   m.y.x  * m.x.y * m.w.z -
                   m.y.x  * m.x.z * m.w.y -
                   m.w.x * m.x.y * m.y.z +
                   m.w.x * m.x.z * m.y.y;

    result.x.w = -m.x.y * m.y.z * m.z.w +
                  m.x.y * m.y.w * m.z.z +
                  m.y.y * m.x.z * m.z.w -
                  m.y.y * m.x.w * m.z.z -
                  m.z.y * m.x.z * m.y.w +
                  m.z.y * m.x.w * m.y.z;

    result.y.w = m.x.x * m.y.z * m.z.w -
                 m.x.x * m.y.w * m.z.z -
                 m.y.x * m.x.z * m.z.w +
                 m.y.x * m.x.w * m.z.z +
                 m.z.x * m.x.z * m.y.w -
                 m.z.x * m.x.w * m.y.z;

    result.z.w = -m.x.x * m.y.y * m.z.w +
                   m.x.x * m.y.w * m.z.y +
                   m.y.x * m.x.y * m.z.w -
                   m.y.x * m.x.w * m.z.y -
                   m.z.x * m.x.y * m.y.w +
                   m.z.x * m.x.w * m.y.y;

    result.w.w = m.x.x * m.y.y * m.z.z -
                  m.x.x * m.y.z * m.z.y -
                  m.y.x * m.x.y * m.z.z +
                  m.y.x * m.x.z * m.z.y +
                  m.z.x * m.x.y * m.y.z -
                  m.z.x * m.x.z * m.y.y;

    return 1.0/det * result;
}

[[stage(vertex)]]
fn vertex(vertex: Vertex) -> VertexOutput {
    var model = r_mesh.model;
    model.w = vec4<f32>(0.0, 0.0, 0.0, 1.0);

    var new_view = inverse(view.inverse_view);
    new_view.w = vec4<f32>(0.0, 0.0, 0.0, 1.0);

    var out: VertexOutput;

    out.position = (view.projection * inverse(new_view) * model * vec4<f32>(vertex.position, 1.0)).xyww;
    out.uv = vec3<f32>(vertex.position.xy, -vertex.position.z);

    return out;
}

[[stage(fragment)]]
fn fragment() -> [[location(0)]] vec4<f32> {
    var color = vec4<f32>(1.0, 1.0, 1.0, 1.0);

    return color;
}
