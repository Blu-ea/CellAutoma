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
    vec3 cam_pos;
} ubo;

layout(location = 0) out vec2 coords;

invariant gl_Position;

void main()
{
    vec4 world_pos = positions[gl_VertexIndex];
    world_pos.xy *= ubo.cam_pos.z * 5.0; // grid size
    world_pos.xy += ubo.cam_pos.xy; // Mouving it under the cam

    gl_Position = ubo.proj * ubo.view * world_pos;
    coords = world_pos.xy;
}
