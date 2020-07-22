#version 300 es        
precision mediump float;
in vec2 LineEnabled;
out vec4 FragColor;
const vec3 background = vec3(0.2, 0.3, 0.3);

void main() {         
    if (LineEnabled.x < 0.9 && LineEnabled.y < 0.9) {
        discard;
    } 
    else if (LineEnabled.x < 0.9 && LineEnabled.y > 0.9) {
        FragColor = vec4(1.0, 0.0, 0.0, 1.0);
    }
    else {
        FragColor = vec4(0.0, 1.0, 0.0, 1.0);
    }

};