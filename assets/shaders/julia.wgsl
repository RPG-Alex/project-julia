// This shader computes the color of each pixel on the screen to render a julia set.
// It is mostly copied from the bevy post_processing example.
// Available here: https://github.com/bevyengine/bevy/blob/main/assets/shaders/post_processing.wgsl

// It is written in WGSL, a minimalist Rust-like language that can be read by the GPU.
// It is also possible to use the more popular C++-like GLSL language if needed.

// Guide:
// 1. #import a struct we will need
// 2. Define some constants
// 3. Define and retrieve data structures from Rust
// 4. Define some utility functions
// 5. Define the main function (fragment)

// Note: complex numbers are represented as vec2<f32> in this shader.

/* Imports */

// This struct that the main function will use contains the UV position of the pixel.
// It specifically contains a uv field which is a vec2<f32> that contains 
// the position of the pixel on the screen. The top left corner of the screen is (0, 0)
// and the bottom right corner is (1, 1).
#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

/* Constants */

const MAX_COLORS_GRADIENT: u32 = 12; // matches color_gradient::MAX_COLORS_GRADIENT

// Matches color_gradient::ColorGradient
struct ColorGradient 
{
  colors: array<vec4<f32>, MAX_COLORS_GRADIENT>,
  tresholds: array<vec4<f32>, (MAX_COLORS_GRADIENT / 4)>,
  size: u32,
}

// matches sets::julia::PostProcessSettings
struct PostProcessSettings 
{
  gradient: ColorGradient,
  view: vec4<f32>,
  time: f32,
  pulse: f32,
  max_iter: u32,
}

// Retrieves the settings from Rust
@group(0) @binding(0) var<uniform> settings: PostProcessSettings;

/* Utility functions */

// Outputs the index-th treshold of the gradient
fn get_tres(index: u32) -> f32
{
  // Not gonna lie, this is stupid, but it's the only way to do it
  // WGSL does not support indexing an array by a non const variable
  switch(index)
  {
    case 0u: {return settings.gradient.tresholds[0u][0];}
    case 1u: {return settings.gradient.tresholds[0u][1];}
    case 2u: {return settings.gradient.tresholds[0u][2];}
    case 3u: {return settings.gradient.tresholds[0u][3];}
    case 4u: {return settings.gradient.tresholds[1u][0];}
    case 5u: {return settings.gradient.tresholds[1u][1];}
    case 6u: {return settings.gradient.tresholds[1u][2];}
    case 7u: {return settings.gradient.tresholds[1u][3];}
    case 8u: {return settings.gradient.tresholds[2u][0];}
    case 9u: {return settings.gradient.tresholds[2u][1];}
    case 10u: {return settings.gradient.tresholds[2u][2];}
    case 11u: {return settings.gradient.tresholds[2u][3];}
    default: {return 0.;}
  }
}

// Outputs the index-th color of the gradient in RGBA format
fn get_col(index: u32) -> vec4<f32>
{
  // Same as above
  switch(index)
  {
    case 0u: {return settings.gradient.colors[0u];}
    case 1u: {return settings.gradient.colors[1u];}
    case 2u: {return settings.gradient.colors[2u];}
    case 3u: {return settings.gradient.colors[3u];}
    case 4u: {return settings.gradient.colors[4u];}
    case 5u: {return settings.gradient.colors[5u];}
    case 6u: {return settings.gradient.colors[6u];}
    case 7u: {return settings.gradient.colors[7u];}
    case 8u: {return settings.gradient.colors[8u];}
    case 9u: {return settings.gradient.colors[9u];}
    case 10u: {return settings.gradient.colors[10u];}
    case 11u: {return settings.gradient.colors[11u];}
    default: {return vec4<f32>(0., 0., 0., 1.);}
  }
}

// Computes the squared modulus of a complex number
fn mod2(a: vec2<f32>) -> f32
{
  return a.x * a.x + a.y * a.y;
}

// Inputs a float between 0 and 1 and outputs the corresponding color in RGBA format
fn interpolate_color(val: f32) -> vec4<f32>
{
  let size = settings.gradient.size;
  let tresholds = settings.gradient.tresholds;
  let colors = settings.gradient.colors;
  if (val < tresholds[0u][0u]) {return colors[0u];}
  for (var i = 0u; i < size - 1u; i++)
  {
    if (get_tres(i) > val)
    {
      let t = (val - get_tres(i - 1u)) / (get_tres(i) - get_tres(i - 1u));
      return mix(get_col(i - 1u), get_col(i), t);
    }
  }
  return get_col(size - 1u);
}

// Smooths the color transition
fn smoother(iter: u32, z: vec2<f32>) -> f32
{
  return clamp((f32(iter) - log2(max(1., log2(mod2(z))))) / f32(settings.max_iter), 0., 1.);
}

// Computes z² + c
fn julia_next(z: vec2<f32>, c: vec2<f32>) -> vec2<f32>
{
  return vec2(z.x * z.x - z.y * z.y, 2. * z.x * z.y) + c;
}

// This function takes the pixel's UV as well as other 
// information contained in FullscreenVertexOutput
// and returns the color of the pixel in a RGBA format.
@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> 
{
  // uv.y should be between 0 and 1 and be 0 at the bottom
  // in.uv.y is currently 0 at the top, so we need to invert it
  var uv = vec2<f32>(in.uv.x, 1.0 - in.uv.y);

  // complex number parameter:
  // (rotates on the circle of radius 0.8 for the animation effect):
  let c = vec2(
    0.8 * cos(settings.time * settings.pulse), 
    0.8 * sin(settings.time * settings.pulse)
  );

  // compute border parameters
  let center = settings.view.xy;
  let width = settings.view.z;
  let height = settings.view.w;

  let top = center.y + height / 2.0;
  let bottom = center.y - height / 2.0;
  let left = center.x - width / 2.0;
  let right = center.x + width / 2.0;

  // now, compute the complex number corresponding to the pixel
  var z = uv * vec2(right - left, top - bottom) + vec2(left, bottom);

  // compute the julia set
  var iter = 0u;
  while (iter < settings.max_iter && mod2(z) < 4.0)
  {
    z = julia_next(z, c);
    iter++;
  }
  let val = smoother(iter, z);
  let color = interpolate_color(val);
  
  return color;
}
