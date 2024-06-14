use bevy::{math::Vec2, sprite::Material2d};

/// This trait is used to define the behavior of a fractal material. It is used
/// to control the fractal animation. The coder will implement this trait for
/// their fractal material and create a plugin from it.
pub trait FractalMaterial2d: Material2d + Default
{
  /// Zooms in on the fractal, e.g. when the user rolls in with their mouse.
  fn zoom_in(&mut self) -> &mut Self;
  /// Zooms out on the fractal, e.g. when the user rolls out with their mouse.
  fn zoom_out(&mut self) -> &mut Self;
  /// Translates the fractal, e.g. when the user clicks and drags with their
  /// mouse. The direction is given in screen pixels coordinates.
  fn translate(&mut self, direction: Vec2) -> &mut Self;
  /// Resizes the fractal rendering screen, e.g. when the window is resized. The
  /// width and height are given in screen pixels coordinates.
  fn resize_screen(&mut self, width: f32, height: f32) -> &mut Self;
  /// Sets the time of the fractal animation. This is used to animate the
  /// fractal, as the time is given to the fragment shader. The time is given in
  /// seconds.
  fn set_timer(&mut self, time: f32) -> &mut Self;
}
