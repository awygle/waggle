#version 300 es            
in vec4 VertexPosition;      
uniform vec2 Dimensions;

layout (std140) uniform bitstring
{
    uint data[4];
} bs;

out vec2 LineEnabled;

const uint LOW = 0u;
const uint HIGH = 3u;
const uint RISING = 2u;
const uint FALLING = 1u;

uint mode(uint data[4], uint index) {
    uint word = index / 32u;
    uint bit = index % 32u;
    uint curr = (data[word] >> bit) & 0x1u;

    uint next_word = (index + 1u) / 32u;
    if (next_word >= 4u) {
        return curr | (curr << 1);
    }

    uint next_bit = (index + 1u) % 32u;
    uint next = (data[next_word] >> next_bit) & 0x1u;

    return curr | (next << 1);
}

void main() {
    vec4 trans = vec4(VertexPosition.x + (64.0 * float(gl_InstanceID % 32)), VertexPosition.y + (96.0 * float(gl_InstanceID / 32)), 0.0, 1.0);
    /*if ((gl_InstanceID % 2) == 1 && gl_VertexID == 0) {

    }*/

    uint m = mode(bs.data, uint(gl_InstanceID));

    if ((m & 0x01u) == 1u && gl_VertexID == 0) { // currently high, flip to roof
        trans.y += 64.0;
    }

    switch (m) {
        case LOW:
            if (gl_VertexID == 0 || gl_VertexID == 1) {
                LineEnabled.x = 1.0;
            }
            else {
                LineEnabled.x = 0.0;
            }
            LineEnabled.y = 0.0;
            break;
        case HIGH:
            if (gl_VertexID == 0 || gl_VertexID == 2) {
                LineEnabled.x = 1.0;
            }
            else {
                LineEnabled.x = 0.0;
            }
            LineEnabled.y = 0.0;
            break;
        case RISING:            
            if (gl_VertexID == 0 || gl_VertexID == 1) {
                LineEnabled.x = 1.0;
            }
            else {
                LineEnabled.x = 0.0;
            }
            if (gl_VertexID == 1 || gl_VertexID == 2) {
                LineEnabled.y = 1.0;
            }
            else {
                LineEnabled.y = 0.0;
            }
            break;
        case FALLING:
            if (gl_VertexID == 0 || gl_VertexID == 2) {
                LineEnabled.x = 1.0;
            }
            else {
                LineEnabled.x = 0.0;
            }
            if (gl_VertexID == 1 || gl_VertexID == 2) {
                LineEnabled.y = 1.0;
            }
            else {
                LineEnabled.y = 0.0;
            }
            break;
    }

    mat4 ortho = mat4(
        2.0/Dimensions.x, 0.0, 0.0, 0.0,
        0.0, 2.0/Dimensions.y, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        -1.0, -1.0, 0.0, 1.0);
    vec4 udc = ortho * trans;
    gl_Position = udc;
}