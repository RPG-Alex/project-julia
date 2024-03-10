// This shader computes the color of each pixel on the screen to render a julia set.
// It is mostly copied from the bevy post_processing example.
// Available here: https://github.com/bevyengine/bevy/blob/main/assets/shaders/post_processing.wgsl


// Since post processing is a fullscreen effect, we use the fullscreen vertex shader provided by bevy.
// This will import a vertex shader that renders a single fullscreen triangle.
//
// A fullscreen triangle is a single triangle that covers the entire screen.
// The box in the top left in that diagram is the screen. The 4 x are the corner of the screen
//
// Y axis
//  1 |  x-----x......
//  0 |  |  s  |  . ´
// -1 |  x_____x´
// -2 |  :  .´
// -3 |  :´
//    +---------------  X axis
//      -1  0  1  2  3
//
// As you can see, the triangle ends up bigger than the screen.
//
// You don't need to worry about this too much since bevy will compute the correct UVs for you.
// Specifically, the top left corner of the triangle is (0, 0) and the bottom right corner is (1, 1).

// This struct that the fragment function will use contains the UV position of the pixel.
#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

struct PostProcessSettings 
{
// #ifdef SIXTEEN_BYTE_ALIGNMENT
    // WebGL2 structs must be 16 byte aligned.
    colors: vec4<f32>
// #endif
}
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

// This function takes the pixel's UV as well as other 
// information contained in FullscreenVertexOutput
// and returns the color of the pixel in a RGBA format.
@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> 
{
  return mix(vec4(in.uv, 0.0, 1.0), settings.colors, 0.5);
}
