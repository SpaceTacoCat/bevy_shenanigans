#import bevy_pbr::mesh_view_bind_group
#import bevy_pbr::mesh_struct

struct Vertex {
    [[location(0)]] position: vec3<f32>;
};

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
};

fn inverse(in: mat4x4<f32>) -> mat4x4<f32> {
    let Azwzw = in.z.z * in.w.w - in.z.w * in.w.z ;
    let Aywzw = in.z.y * in.w.w - in.z.w * in.w.y ;
    let Ayzzw = in.z.y * in.w.z - in.z.z * in.w.y ;
    let Axwzw = in.z.x * in.w.w - in.z.w * in.w.x ;
    let Axzzw = in.z.x * in.w.z - in.z.z * in.w.x ;
    let Axyzw = in.z.x * in.w.y - in.z.y * in.w.x ;
    let Azwyw = in.y.z * in.w.w - in.y.w * in.w.z ;
    let Aywyw = in.y.y * in.w.w - in.y.w * in.w.y ;
    let Ayzyw = in.y.y * in.w.z - in.y.z * in.w.y ;
    let Azwyz = in.y.z * in.z.w - in.y.w * in.z.z ;
    let Aywyz = in.y.y * in.z.w - in.y.w * in.z.y ;
    let Ayzyz = in.y.y * in.z.z - in.y.z * in.z.y ;
    let Axwyw = in.y.x * in.w.w - in.y.w * in.w.x ;
    let Axzyw = in.y.x * in.w.z - in.y.z * in.w.x ;
    let Axwyz = in.y.x * in.z.w - in.y.w * in.z.x ;
    let Axzyz = in.y.x * in.z.z - in.y.z * in.z.x ;
    let Axyyw = in.y.x * in.w.y - in.y.y * in.w.x ;
    let Axyyz = in.y.x * in.z.y - in.y.y * in.z.x ;

    let det = determinant(in);
    let det = 1.0 / det;

    return mat4x4<f32>(
       det *   ( in.y.y * Azwzw - in.y.z * Aywzw + in.y.w * Ayzzw ),
       det * - ( in.x.y * Azwzw - in.x.z * Aywzw + in.x.w * Ayzzw ),
       det *   ( in.x.y * Azwyw - in.x.z * Aywyw + in.x.w * Ayzyw ),
       det * - ( in.x.y * Azwyz - in.x.z * Aywyz + in.x.w * Ayzyz ),
       det * - ( in.y.x * Azwzw - in.y.z * Axwzw + in.y.w * Axzzw ),
       det *   ( in.x.x * Azwzw - in.x.z * Axwzw + in.x.w * Axzzw ),
       det * - ( in.x.x * Azwyw - in.x.z * Axwyw + in.x.w * Axzyw ),
       det *   ( in.x.x * Azwyz - in.x.z * Axwyz + in.x.w * Axzyz ),
       det *   ( in.y.x * Aywzw - in.y.y * Axwzw + in.y.w * Axyzw ),
       det * - ( in.x.x * Aywzw - in.x.y * Axwzw + in.x.w * Axyzw ),
       det *   ( in.x.x * Aywyw - in.x.y * Axwyw + in.x.w * Axyyw ),
       det * - ( in.x.x * Aywyz - in.x.y * Axwyz + in.x.w * Axyyz ),
       det * - ( in.y.x * Ayzzw - in.y.y * Axzzw + in.y.z * Axyzw ),
       det *   ( in.x.x * Ayzzw - in.x.y * Axzzw + in.x.z * Axyzw ),
       det * - ( in.x.x * Ayzyw - in.x.y * Axzyw + in.x.z * Axyyw ),
       det *   ( in.x.x * Ayzyz - in.x.y * Axzyz + in.x.z * Axyyz ),
    );
}

[[stage(vertex)]]
fn vertex(in: Vertex) -> VertexOutput {
    var out: VertexOutput;

    out.position = vec4<f32>(in.position.xy, 1.0, 1.0);

    return out;
}


[[stage(fragment)]]
fn fragment(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
