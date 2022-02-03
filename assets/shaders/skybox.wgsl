#import bevy_pbr::mesh_view_bind_group
#import bevy_pbr::mesh_struct

struct ExtraData {
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
var<uniform> g_extra: ExtraData;

[[group(2), binding(1)]]
var g_texture: texture_2d_array<f32>;

[[group(2), binding(2)]]
var g_sampler: sampler;

[[stage(vertex)]]
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    out.position = view.view_proj * g_mesh.model * vec4<f32>(vertex.position, 1.0);
    out.position.z = 0.0;
    out.uv = vertex.position;

    return out;
}

struct Face {
    uv: vec2<f32>;
    idx: i32;
};

fn face(ray: vec3<f32>) -> Face {
    let ray_abs = abs(ray);
    var max_adj: f32;
    var idx: i32;
    var uv: vec2<f32>;

    if (ray_abs.z >= ray_abs.x && ray_abs.z >= ray_abs.y) {
        if (ray.z < 0.0) { idx = 5; } else { idx = 4; }
        max_adj = 0.5 / ray_abs.z;
        uv = vec2<f32>(ray.x * -sign(ray.z), -ray.y);
    } else if (ray_abs.y >= ray_abs.x) {
        if (ray.y < 0.0) { idx = 3; } else { idx = 2; }
        max_adj = 0.5 / ray.y;
        uv = vec2<f32>(ray.x * sign(ray.y), -ray.z);
    } else {
        if (ray.x < 0.0) { idx = 1; } else { idx = 0; }
        max_adj = 0.5 / ray.x;
        uv = vec2<f32>(ray.z, ray.y * -sign(ray.x));
    }

    var out: Face;

    out.idx = idx;
    out.uv = uv * max_adj + 0.5;

    return out;
}

[[stage(fragment)]]
fn fragment(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let face_info = face(in.uv);

    var color = textureSample(g_texture, g_sampler, face_info.uv, face_info.idx);

    return color;
}
