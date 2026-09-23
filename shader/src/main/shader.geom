#version 450

layout (points) in;
layout (triangle_strip, max_vertices = 4) out;

layout(location = 0) in vec3 vertColor[];

layout(location = 0) out vec3 outColor;

layout(set = 0, binding = 0) uniform UniformBufferObject {
    mat4 model;
    mat4 view;
    mat4 proj;
} ubo;

void main(){
    vec4 base = gl_in[0].gl_Position;
    vec3 color = vertColor[0];
    mat4 mvp = ubo.proj * ubo.view * ubo.model;

    vec4 offsets[4] = vec4[](
        vec4(0.0, 0.0, 0.0, 0.0),
        vec4(1.0, 0.0, 0.0, 0.0),
        vec4(0.0, 1.0, 0.0, 0.0),
        vec4(1.0, 1.0, 0.0, 0.0)
    );

    for (int i = 0; i < 4; i++){
        outColor = color;
        gl_Position = mvp * (base + offsets[i]);
        EmitVertex();
    }

    EndPrimitive();
}