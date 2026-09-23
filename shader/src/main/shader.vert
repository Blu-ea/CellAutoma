#version 450

layout(set = 0, binding = 0) uniform UniformBufferObject {
    mat4 model;
    mat4 view;
    mat4 proj;
} ubo;

layout(location = 0) in vec2 inPosition;
layout(location = 1) in vec3 inColor;

layout(location = 0) out vec3 fragColor;

void main() {
    gl_Position = vec4(inPosition, 0.0, 1.0); // Do not multiply with UBO, this is done in the geom shader !
    // gl_PointSize = 10.0; // Use for POINT_LIST instead of TRIANGLE_LIST
    fragColor = inColor;
}
