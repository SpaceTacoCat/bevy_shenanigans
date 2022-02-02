#version 450

layout(location = 0) in vec3 Vertex_Position;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
    mat4 InverseView;
    mat4 Projection;
};
layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};
layout(set = 2, binding = 0) uniform Extras {
    mat4 UntranslatedView;
};

void main() {
    vec4 pos = ViewProj * Model * vec4(Vertex_Position, 1.0);
    gl_Position = pos.xyzw;
}
