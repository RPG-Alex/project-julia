use bevy::{
  app::{App, Plugin, Startup, Update},
  asset::Assets,
  prelude::{default, Camera2dBundle, Commands, ResMut},
  render::{
    mesh::{Indices, Mesh, PrimitiveTopology},
    render_asset::RenderAssetUsages,
    render_resource::AsBindGroup,
  },
  sprite::{Material2dPlugin, MaterialMesh2dBundle},
};

use crate::traits::FractalMaterial2d;

#[derive(Debug, Clone, Default)]
pub struct FractalPlugin2d<M>
{
  _marker: std::marker::PhantomData<M>,
}

impl<M> Plugin for FractalPlugin2d<M>
where
  M: FractalMaterial2d + AsBindGroup,
  <M as AsBindGroup>::Data: PartialEq + Eq + std::hash::Hash + Clone,
{
  fn build(&self, app: &mut App)
  {
    let plugin = Material2dPlugin::<M>::default();
    app
      .add_plugins(plugin)
      .add_systems(Startup, create_screen_covering_triangle::<M>)
      .add_systems(
        Update,
        (
          crate::controll::update_fractal_material::<M>,
          crate::controll::zoom_with_mouse_wheel::<M>,
          crate::controll::click_and_drag_with_mouse::<M>,
        ),
      );
  }
}

/// Creates a triangle mesh that will cover the entire screen and attaches a
/// JuliaMaterial to it. The fractal animation will play on the triangle.
fn create_screen_covering_triangle<M>(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<M>>,
) where
  M: FractalMaterial2d,
{
  // The triangle that will cover the screen.
  let asset_usage = RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD;
  let mut triangle = Mesh::new(PrimitiveTopology::TriangleList, asset_usage);

  // This function produces the following triangle, once it is scaled correctly:
  //
  //  1 |  0-----x.....2
  //  0 |  |  s  |  . ´
  // -1 |  x_____x´
  // -2 |  :  .´
  // -3 |  1´
  //    +---------------
  //      -1  0  1  2  3
  //
  // The axes are clip-space x and y. The region marked s is the visible region.
  // The digits in the corners of the right-angled triangle are the vertex
  // indices.
  //
  // The top-left has UV 0,0, the bottom-left has 0,2, and the top-right has 2,0.
  // This means that the UV gets interpolated to 1,1 at the bottom-right corner
  // of the clip-space rectangle that is at 1,-1 in clip space.

  // Vertices positions relative to center, in pixels. The triangle will be
  // rescaled in `update_julia_triangle` to cover the screen.
  triangle.insert_attribute(
    Mesh::ATTRIBUTE_POSITION,
    vec![[-1.0, 1.0, 0.0], [-1.0, -3.0, 0.0], [3.0, 1.0, 0.0]],
  );

  // UVs for the vertices.
  triangle.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0], [0.0, 2.0], [2.0, 0.0]]);

  // Connection of the vertices to form triangles.
  triangle.insert_indices(Indices::U32(vec![0, 1, 2]));

  commands.spawn(Camera2dBundle::default());
  // Spawn a bundle that contains the julia material and the triangle all in one.
  commands.spawn(MaterialMesh2dBundle {
    mesh: meshes.add(triangle).into(),
    material: materials.add(M::default()),
    ..default()
  });
}
