// This is a fragment shader, a shader that computes the color of each pixel 
// on the screen to render a julia set. 

// To be exact, a triangle's data is first sent to a vertex shader (handled 
// by bevy in this case). For each vertex, the v-shader's main function outputs
// a struct that will come as an input here. In fact, the values of each outputed field
// of the struct are interpolated between the vertices. This is done under the 
// hood by graphics APIs. Here, what we will have is a VertexOutput, defined by bevy
// in <https://github.com/bevyengine/bevy/blob/main/crates/bevy_pbr/src/render/forward_io.wgsl>.

// It is written in WGSL, a minimalist Rust-like language that can be read by the GPU.
// It is also possible to use the more popular C++-like GLSL language if needed.

// Guide:
// 1. #import the struct outputed by the vertex shader (VertexOutput)
// 2. Define some constants
// 3. Define and retrieve data from Rust
// 4. Define some utility functions
// 5. Define the main function (fragment)

// Note: complex numbers are represented as vec2<f32> in this shader.

/* Imports */

#import bevy_pbr::forward_io::VertexOutput;

/* Constants */

const MAX_COLORS_GRADIENT: u32 = 12; // matches color_gradient::MAX_COLORS_GRADIENT

// Matches color_gradient::ColorGradient
struct ColorGradient 
{
  colors: array<vec4<f32>, MAX_COLORS_GRADIENT>,
  tresholds: array<vec4<f32>, (MAX_COLORS_GRADIENT / 4)>,
  size: u32,
}

// Retrieves the settings from Rust. @group(2) is necessary when
// dealing with bevy's MaterialPlugins. @binding(x) corresponds
// to #[uniform(x)] in Rust.
@group(2) @binding(0) var<uniform> gradient: ColorGradient;
@group(2) @binding(1) var<uniform> view: vec4<f32>;
@group(2) @binding(2) var<uniform> screen: vec2<f32>;
@group(2) @binding(3) var<uniform> time: f32;
@group(2) @binding(4) var<uniform> pulse: f32;
@group(2) @binding(5) var<uniform> max_iter: u32;
@group(2) @binding(6) var<uniform> substeps_sqrt: u32;

/* Utility functions */

// Outputs the index-th treshold of the gradient
fn get_tres(index: u32) -> f32
{
  // Not gonna lie, this is stupid, but it's the only way to do it
  // WGSL does not support indexing an array by a non const variable
  switch(index)
  {
    case 0u: {return gradient.tresholds[0u][0];}
    case 1u: {return gradient.tresholds[0u][1];}
    case 2u: {return gradient.tresholds[0u][2];}
    case 3u: {return gradient.tresholds[0u][3];}
    case 4u: {return gradient.tresholds[1u][0];}
    case 5u: {return gradient.tresholds[1u][1];}
    case 6u: {return gradient.tresholds[1u][2];}
    case 7u: {return gradient.tresholds[1u][3];}
    case 8u: {return gradient.tresholds[2u][0];}
    case 9u: {return gradient.tresholds[2u][1];}
    case 10u: {return gradient.tresholds[2u][2];}
    case 11u: {return gradient.tresholds[2u][3];}
    default: {return 0.;}
  }
}

// Outputs the index-th color of the gradient in RGBA format
fn get_col(index: u32) -> vec4<f32>
{
  // Same as above
  switch(index)
  {
    case 0u: {return gradient.colors[0u];}
    case 1u: {return gradient.colors[1u];}
    case 2u: {return gradient.colors[2u];}
    case 3u: {return gradient.colors[3u];}
    case 4u: {return gradient.colors[4u];}
    case 5u: {return gradient.colors[5u];}
    case 6u: {return gradient.colors[6u];}
    case 7u: {return gradient.colors[7u];}
    case 8u: {return gradient.colors[8u];}
    case 9u: {return gradient.colors[9u];}
    case 10u: {return gradient.colors[10u];}
    case 11u: {return gradient.colors[11u];}
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
  let size = gradient.size;
  let tresholds = gradient.tresholds;
  let colors = gradient.colors;
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
  return clamp((f32(iter) - log2(max(1., log2(mod2(z))))) / f32(max_iter), 0., 1.);
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
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> 
{
  // uv.y should be between 0 and 1 and be 0 at the bottom
  // in.uv.y is currently 0 at the top, so we need to invert it
  let uv = vec2<f32>(in.uv.x, 1.0 - in.uv.y);

  // complex number parameter:
  // (rotates on the circle of radius 0.8 for the animation effect):
  let c = vec2(
   0.8 * cos(time * pulse), 
   0.8 * sin(time * pulse)
  );

  // compute border parameters
  let center = view.xy;
  let width = view.z;
  let height = view.w;
  let pixel_width = width / screen.x;
  let pixel_height = height / screen.y;

  let top = center.y + height / 2.0;
  let bottom = center.y - height / 2.0;
  let left = center.x - width / 2.0;
  let right = center.x + width / 2.0;

  // now, compute the complex number corresponding to the pixel
  let z = uv * vec2(right - left, top - bottom) + vec2(left, bottom);
  
  // compute the julia set
  var total_color = vec4<f32>(0.0, 0.0, 0.0, 0.0);
  for (var i = 0u; i < substeps_sqrt; i++) {
    for (var j = 0u; j < substeps_sqrt; j++) {
      var sub_z = z + vec2(
        pixel_width * (f32(i) / f32(substeps_sqrt) - 0.5),
        pixel_height * (f32(j) / f32(substeps_sqrt) - 0.5)
      );
      var iter = 0u;
      while (iter < max_iter && mod2(sub_z) < 4.0)
      {
        sub_z = julia_next(sub_z, c);
        iter++;
      }
      let val = smoother(iter, sub_z);
      let color = interpolate_color(val);
      total_color = total_color + color;
    }
  }
    
  return total_color / f32(substeps_sqrt * substeps_sqrt);
}
