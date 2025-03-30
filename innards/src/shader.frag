#version 300 es

precision highp float;

out vec4 out_color;

uniform float u_time;
uniform float u_phase;
uniform vec2 u_resolution;
uniform float u_wavelength;
// uniform float u_frequency;

const float PI = 3.1415926536f;
const float TWO_PI = 6.2831853072f;
const float c = 299792458.f;
const uint MAX_ARRAY_POINTS = 200u;

uniform uint u_num_array_elements;
uniform vec2 u_array_elements[MAX_ARRAY_POINTS];

void main() {
    vec2 uv = gl_FragCoord.xy / u_resolution.xy;

    // display array points
    uint i;
    float d_to_array_point = 10000.f;
    for(i = 0u; i < u_num_array_elements; i++) {
        d_to_array_point = min(d_to_array_point, distance(uv, u_array_elements[i]));
    }
    d_to_array_point = smoothstep(0.005f, 0.0051f, d_to_array_point);
    vec3 source_color = vec3(1.0f);

    // each source is a sine wave, red = high, blue = low
    float amplitude = 0.f;
    float phase_offset = 0.f;
    for(i = 0u; i < u_num_array_elements; i++) {
        phase_offset = u_phase * float(i);
        amplitude += sin(TWO_PI * u_time - TWO_PI / u_wavelength * distance(uv, u_array_elements[i]) - phase_offset);
    }

    amplitude /= float(u_num_array_elements);

    // at this point amplitude is in [-1,1]

    float positive = step(0.0f, amplitude);

    vec3 point_color = mix( //choose between blue and red, black in between
    mix(vec3(0.f, 0.f, 1.f), vec3(0.1f), amplitude + 1.f), // blue
    mix(vec3(0.1f), vec3(1.f, 0.f, 0.f), amplitude), // red
    positive // red if positive, blue if negative
    );

    vec3 col = mix(source_color, point_color, d_to_array_point);
    out_color = vec4(col, 1.0f);
}