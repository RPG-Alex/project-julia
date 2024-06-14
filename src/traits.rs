use bevy::{math::Vec2, sprite::Material2d};

pub trait FractalMaterial2d: Material2d + Default
{
  fn zoom_in(&mut self) -> &mut Self;
  fn zoom_out(&mut self) -> &mut Self;
  fn translate(&mut self, direction: Vec2) -> &mut Self;
  fn resize_screen(&mut self, width: f32, height: f32) -> &mut Self;
  fn set_timer(&mut self, time: f32) -> &mut Self;
}
