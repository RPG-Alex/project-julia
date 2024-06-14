use bevy::{
  asset::Asset,
  math::{Vec2, Vec4},
  prelude::Component,
  reflect::TypePath,
  render::render_resource::{AsBindGroup, ShaderRef},
  sprite::Material2d,
};

use crate::{color_gradient, traits::FractalMaterial2d};

/// This is the component that will get passed to the shader.
/// The WGSL script contains the same fields.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Component)]
pub struct JuliaMaterial
{
  // The color gradient to use for coloring the julia set.
  #[uniform(0)]
  pub gradient:      color_gradient::ColorGradient,
  // The view is a vec4 with the x and y being the position of the camera on
  // the complex plane and the z and w being the width and height of the
  // camera on the complex plane.
  #[uniform(1)]
  pub view:          Vec4,
  // The screen is a vec2 with the x and y being the width and height of the
  // screen.
  #[uniform(2)]
  pub screen:        Vec2,
  // Time in seconds since the start of the program.
  #[uniform(3)]
  pub time:          f32,
  // Defines the speed of the animation
  #[uniform(4)]
  pub pulse:         f32,
  // The maximum number of iterations to calculate the julia set.
  // Should change with the zoom level.
  #[uniform(5)]
  pub max_iter:      u32,
  // Square root of the number of substeps to reduce the aliasing.
  #[uniform(6)]
  pub substeps_sqrt: u32,
}

impl Default for JuliaMaterial
{
  fn default() -> Self
  {
    Self {
      gradient:      color_gradient::DEFAULT_COLOR_GRADIENT,
      view:          Vec4::new(0.0, 0.0, 2.0, 2.0),
      screen:        Vec2::new(800.0, 800.0),
      time:          0.0,
      pulse:         0.1,
      max_iter:      150,
      substeps_sqrt: 4,
    }
  }
}

impl Material2d for JuliaMaterial
{
  fn fragment_shader() -> ShaderRef { "shaders/julia.wgsl".into() }
}

impl FractalMaterial2d for JuliaMaterial
{
  fn zoom_in(&mut self) -> &mut Self
  {
    self.view.w *= 0.9;
    self.view.z *= 0.9;
    self.max_iter += 3;
    self
  }

  fn zoom_out(&mut self) -> &mut Self
  {
    self.view.w *= 1.1;
    self.view.z *= 1.1;
    self.max_iter -= 3;
    self.max_iter = self.max_iter.max(100);
    self
  }

  fn translate(&mut self, direction: Vec2) -> &mut Self
  {
    let complex_dir = Vec2::new(
      direction.x / self.screen.x * self.view.z,
      -direction.y / self.screen.y * self.view.w,
    );
    self.view.x += complex_dir.x;
    self.view.y += complex_dir.y;
    self
  }

  fn resize_screen(&mut self, width: f32, height: f32) -> &mut Self
  {
    self.screen = Vec2::new(width, height);
    self.view.z = self.view.w * width / height;
    self
  }

  fn set_timer(&mut self, time: f32) -> &mut Self
  {
    self.time = time;
    self
  }
}
