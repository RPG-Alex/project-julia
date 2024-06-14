use bevy::{
  math::Vec4,
  prelude::Resource,
  render::{color::Color, render_resource::ShaderType},
};

/// The maximum number of colors in a gradient. This has to be a multiple of 4
/// to satisfy WGSL byte alignment rules more easily.
pub const MAX_COLORS_GRADIENT: usize = 12;
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
  // The rest does not matter, as the size of the gradient will be set to 8
  Color::BLACK,
  Color::BLACK,
  Color::BLACK,
  Color::BLACK,
];

// It is an array Vec4 because it is easier to pass to the shader.
// The exact reason lies in byte alignment rules. This is why N is a multiple of
// 4. However, the code has to be a bit more verbose to handle this.
const DEFAULT_TRESHOLDS: [Vec4; N / 4] = [
  Vec4::new(0.03, 0.05, 0.08, 0.1),
  Vec4::new(0.13, 0.18, 0.25, 1.),
  // Same here, the rest does not matter
  Vec4::new(1., 1., 1., 1.),
];

/// The default color gradient.
pub const DEFAULT_COLOR_GRADIENT: ColorGradient = ColorGradient {
  colors:    DEFAULT_RGBA_COLORS,
  tresholds: DEFAULT_TRESHOLDS,
  size:      8,
};

impl Default for ColorGradient
{
  fn default() -> Self { DEFAULT_COLOR_GRADIENT }
}

/// A color gradient to be used in a shader. It contains n colors and n
/// tresholds, with n being at most [`MAX_COLORS_GRADIENT`]. Between 0 and the
/// first treshold, the first color is used. After the last treshold, the last
/// color is used. Between two tresholds, the color is interpolated.
#[derive(Resource, Copy, Clone, Debug, ShaderType)]
pub struct ColorGradient
{
  /// The colors to use in the gradient. These are RGBA colors contained in a
  /// buffer with at most [`MAX_COLORS_GRADIENT`] elements.
  pub colors:    [Color; N],
  /// The tresholds to use in the gradient. These are floating points contained
  /// in a buffer with at most [`MAX_COLORS_GRADIENT`] elements. To satisfy
  /// byte alignment rules, the floating points are wrapped in Vec4's, so the
  /// buffer has at most [`MAX_COLORS_GRADIENT`] / 4 vectors. This explains why
  /// [`MAX_COLORS_GRADIENT`] has to be a multiple of 4.
  pub tresholds: [Vec4; N / 4],
  /// Effective size of the gradient.
  pub size:      u32,
}

impl ColorGradient
{
  #[inline]
  #[allow(dead_code)]
  /// Create a new color gradient from a slice of colors and a slice of
  /// tresholds. Automatically wraps the tresholds in Vec4's to satisfy byte
  /// alignment rules. **THIS FUNCTION IS NOT TESTED**.
  ///
  /// # Arguments
  /// * `input_colors` - The colors to use in the gradient.
  /// * `input_tresholds` - The tresholds to use in the gradient.
  ///
  /// # Panics
  /// Panics if the length of the input colors and tresholds is not the same, or
  /// if the length of the input colors is greater than [`MAX_COLORS_GRADIENT`].
  pub fn new(input_colors: &[Color], input_tresholds: &[f32]) -> Self
  {
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
