#version 330 core
out vec4 FragColor;

in vec2 TexCoords;

uniform float screen_width;
uniform float screen_height;
uniform sampler2D screenTexture;

uniform float pixelWidth = 3.0;
uniform float pixelHeight = 3.0;

void main()
{
    vec2 uv = TexCoords.xy;
    uv *=  1.0 - uv.yx;
    float vig = uv.x*uv.y * 15.0;
    vig = pow(vig, 0.3);

    float dx = pixelWidth*(1.0/screen_width);
    float dy = pixelHeight*(1.0/screen_height);

    vec2 coord = vec2(dx*floor(TexCoords.x/dx), dy*floor(TexCoords.y/dy));

    vec3 tc = texture(screenTexture, coord).rgb;
    
    FragColor = vec4(tc,1.0) * vig;
}
