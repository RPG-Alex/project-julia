use bevy::prelude::*;

type Rgba = Color;

const DEFAULT_RGBA_COLORS: [Rgba; 8] = [
  Color::rgba(0., 0., 0., 1.),
  Color::rgba(0.016, 0.137, 0.231, 1.),
  Color::rgba(0.145, 0.514, 0.8, 1.),
  Color::rgba(0.886, 0.91, 0.557, 1.),
  Color::rgba(0.82, 0.502, 0.165, 1.),
  Color::rgba(0.839, 0.059, 0.059, 1.),
  Color::rgba(0.549, 0.024, 0.024, 1.),
  Color::rgba(0., 0., 0., 1.),
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
        let color = self.colors[i] * (1.0 - nx) + self.colors[i - 1] * nx;
        return color;
      }
    }
    *self.colors.last().unwrap()
  }
}

