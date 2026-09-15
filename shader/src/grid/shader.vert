#version 450 core


const vec4 positions[4] = vec4[4](
    vec4(-0.5,  0.5, 0.0, 1.0),
    vec4( 0.5,  0.5, 0.0, 1.0),
    vec4(-0.5, -0.5, 0.0, 1.0),
    vec4( 0.5, -0.5, 0.0, 1.0)
);

layout(set = 0, binding = 0) uniform UniformBufferObject {
    mat4 model;
    mat4 view;
    mat4 proj;
} ubo;

layout(location = 0) out vec2 coords;

invariant gl_Position;

void main()
{
    vec4 world_pos = positions[gl_VertexIndex];
    world_pos.xy *= 100.0f; // grid size

    gl_Position = ubo.proj * ubo.view * world_pos;
    coords = world_pos.xy;
}
