use bevy::{prelude::*, render::render_resource::ShaderType};
pub const MAX_COLORS_GRADIENT: usize = 12;
// Make it a multiple of 4 to satisfy alignment rules easily
const N: usize = MAX_COLORS_GRADIENT; // Alias for simpler code below

const DEFAULT_RGBA_COLORS: [Color; N] = [
  Color::rgba(0., 0., 0., 1.),
  Color::rgba(0.016, 0.137, 0.231, 1.),
  Color::rgba(0.145, 0.514, 0.8, 1.),
  Color::rgba(0.886, 0.91, 0.557, 1.),
  Color::rgba(0.82, 0.502, 0.165, 1.),
  Color::rgba(0.839, 0.059, 0.059, 1.),
  Color::rgba(0.549, 0.024, 0.024, 1.),
  Color::rgba(0., 0., 0., 1.),
  Color::BLACK,
  Color::BLACK,
  Color::BLACK,
  Color::BLACK,
];

// It is an array Vec4 because it is easier to pass to the shader.
// The exact reason lies in byte alignment rules. This is why N is a multiple of 4.
// However, the code has to be a bit more verbose to handle this.
const DEFAULT_TRESHOLDS: [Vec4; N / 4] = [
  Vec4::new(0.03, 0.05, 0.08, 0.1),
  Vec4::new(0.13, 0.18, 0.25, 1.),
  Vec4::new(1., 1., 1., 1.),
];

pub const DEFAULT_COLOR_GRADIENT: ColorGradient = ColorGradient {
  colors: DEFAULT_RGBA_COLORS,
  tresholds: DEFAULT_TRESHOLDS,
  size: 8,
};
impl Default for ColorGradient {
  fn default() -> Self {
    DEFAULT_COLOR_GRADIENT
  }
}

#[derive(Resource, Copy, Clone, Debug, ShaderType)]
pub struct ColorGradient {
  pub colors: [Color; N],       // Colors buffer
  pub tresholds: [Vec4; N / 4], // Tresholds buffer
  pub size: u32,                // Number of colors
}
// size without padding is N * 4 * 4 + N * 4 + 4 = 204 bytes
// need a padding of 4 bytes to make it a multiple of 16, which webgl2 requires

impl ColorGradient {
  #[inline]
  #[allow(dead_code)]
  pub fn new(input_colors: &[Color], input_tresholds: &[f32]) -> Self {
    assert_eq!(input_colors.len(), input_tresholds.len());
    assert!(input_colors.len() <= MAX_COLORS_GRADIENT);
    let mut colors = [Color::default(); N];
    let mut tresholds = [Vec4::ZERO; N / 4];
    colors[0..input_colors.len()].clone_from_slice(input_colors);
    // The verbose part comes here to wrap the tresholds in Vec4's
    let mut index = 0;
    for (i, &treshold) in input_tresholds.iter().enumerate() {
      tresholds[index][i % 4] = treshold;
      if i % 4 == 3 {
        index += 1;
      }
    }
    Self {
      colors,
      tresholds,
      size: input_colors.len() as u32,
    }
  }
}
