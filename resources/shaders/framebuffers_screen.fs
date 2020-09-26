#version 330 core
out vec4 FragColor;

in vec2 TexCoords;

uniform float screen_width;
uniform float screen_height;
uniform sampler2D screenTexture;

void main()
{
    vec2 uv = TexCoords.xy;
    uv *=  1.0 - uv.yx;
    float vig = uv.x*uv.y * 15.0;
    vig = pow(vig, 0.3);

    
    FragColor = texture(screenTexture, TexCoords) * vig;
}
