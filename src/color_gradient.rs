use bevy::prelude::*;

type Rgba = UVec4;

const DEFAULT_RGBA_COLORS: [Rgba; 8] = [
  UVec4::new(0, 0, 0, 255),
  UVec4::new(4, 35, 59, 255),
  UVec4::new(37, 131, 204, 255),
  UVec4::new(226, 232, 142, 255),
  UVec4::new(209, 128, 42, 255),
  UVec4::new(214, 15, 15, 255),
  UVec4::new(140, 6, 6, 255),
  UVec4::new(0, 0, 0, 255),
];

const DEFAULT_TRESHOLDS: [f32; 8] = [0.03, 0.05, 0.08, 0.1, 0.13, 0.18, 0.25, 1.];

#[derive(Resource)]
pub struct ColorGradient<const N: usize>
{
  colors:    [Rgba; N],
  tresholds: [f32; N],
}

pub const DEFAULT_COLOR_GRADIENT: ColorGradient<8> = ColorGradient {
  colors:    DEFAULT_RGBA_COLORS,
  tresholds: DEFAULT_TRESHOLDS,
};

impl Default for ColorGradient<8>
{
  fn default() -> Self { DEFAULT_COLOR_GRADIENT }
}

impl<const N: usize> ColorGradient<N>
{
  #[inline]
  pub fn new(colors: [Rgba; N], tresholds: [f32; N]) -> Self { Self { colors, tresholds } }

  #[inline]
  pub fn get_color(&self, x: f32) -> Rgba
  {
    if x <= self.tresholds[0] {
      return self.colors[0];
    }
    for i in 0..N {
      if self.tresholds[i] > x {
        let nx = (self.tresholds[i] - x) / (self.tresholds[i] - self.tresholds[i - 1]);
        let float_color = self.colors[i].as_vec4() * (1.0 - nx) + self.colors[i - 1].as_vec4() * nx;
        let color = float_color.as_uvec4();
        return color;
      }
    }
    *self.colors.last().unwrap()
  }
}
