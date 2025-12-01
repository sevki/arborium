#version 330 core

uniform mat4 modelViewProjection;
uniform vec3 lightPosition;
uniform sampler2D diffuseTexture;

in vec3 vertexPosition;
in vec3 vertexNormal;
in vec2 texCoord;

out vec4 fragColor;

void main() {
    vec3 normal = normalize(vertexNormal);
    vec3 lightDir = normalize(lightPosition - vertexPosition);
    float diff = max(dot(normal, lightDir), 0.0);
    vec4 texColor = texture(diffuseTexture, texCoord);
    fragColor = texColor * diff;
}
