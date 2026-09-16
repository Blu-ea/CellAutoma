#version 450

const float cell_size = 10.0f;
const float half_cell_size = cell_size * 0.5f;

const float subcell_size = 1.0f;
const float half_subcell_size = subcell_size * 0.5f;

const float cell_line_thickness    = 0.05f;
const float subcell_line_thickness = 0.005f;

const vec4 cell_colour    = vec4( 0.75, 0.75, 0.75, 0.5 );
const vec4 subcell_colour = vec4(  0.5,  0.0,  0.0, 0.5 );


layout(location = 0) in vec2 coords;

vec2 cell_coords    = mod( coords + half_cell_size , cell_size    );
vec2 subcell_coords = mod( coords + half_subcell_size , subcell_size );
vec2 tmp = fwidth(coords);

vec2 distance_to_cell    = abs( cell_coords    - half_cell_size    );
vec2 distance_to_subcell = abs( subcell_coords - half_subcell_size );

vec2 d = fwidth(coords);
vec2 adjusted_cell_line_thickness    = ( cell_line_thickness    + d );
vec2 adjusted_subcell_line_thickness = ( subcell_line_thickness + d );



layout(location = 0) out vec4 outColor;

void main() {
    if (any(lessThan(distance_to_cell,    vec2(adjusted_cell_line_thickness    * 0.5)))) outColor = cell_colour;
    else if (any(lessThan(distance_to_subcell, vec2(adjusted_subcell_line_thickness * 0.5)))) outColor = subcell_colour;
    else outColor = vec4(0);
}

