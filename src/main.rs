use bevy::{
  prelude::*,
  render::{
    render_asset::RenderAssetUsages,
    render_resource::{Extent3d, TextureDimension, TextureFormat},
  },
};

mod color_gradient;
mod sets;
mod controll;
use controll::{click_and_drag_with_mouse, zoom_with_mouse_wheel};
use sets::julia;

fn main() {
  App::new()
    .add_plugins((DefaultPlugins, julia::PostProcessPlugin))
    .add_systems(Startup, (julia::setup, setup_ui /* setup */))
    // .add_systems(PostStartup, update_fractal)
    .add_systems(
      Update,
      (
        julia::update_settings,
       zoom_with_mouse_wheel,
       click_and_drag_with_mouse
      ),
    )
    .insert_resource(FractalZoom {
      scale: 3.0,
      center: (-0.8, 0.156),
    })
    .insert_resource(ZoomButtonClicked::default())
    .run();
}

#[derive(Component)]
struct FractalMaterial;

#[allow(dead_code)]
fn setup(
  mut commands: Commands,
  mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
  mut images: ResMut<Assets<Image>>,
) {
  let size = Vec2::new(1280.0, 1280.0);
  let extent = Extent3d {
    width: size.x as u32,
    height: size.y as u32,
    depth_or_array_layers: 1,
  };
  let image = Image::new_fill(
    extent,
    TextureDimension::D2,
    &[0, 0, 0, 0],
    TextureFormat::Rgba8Unorm,
    RenderAssetUsages::default(),
  );

  // Add image and check that it is successfully loaded
  let image_handle = images.add(image);
  // if image_handle.is_none() {
  //   eprintln!("Failed to load the image asset.");
  //   return;
  // }

  let texture_atlas = TextureAtlasLayout::from_grid(size, 1, 1, None, None);
  let atlas_handle = texture_atlases.add(texture_atlas);

  // Camera
  commands.spawn(Camera2dBundle::default());

  // Create a sprite to render the fractal texture atlas
  commands.spawn((
    SpriteSheetBundle {
      atlas: TextureAtlas {
        layout: atlas_handle,
        index: 0,
      },
      ..default()
    },
    FractalMaterial,
  ));
  let fractal_texture = FractalTexture(image_handle);
  let fractal_zoom = FractalZoom {
    scale: 3.0,
    center: (0.0, 0.0),
  };
  commands.insert_resource(fractal_texture);

  // Initialize it
  commands.insert_resource(fractal_zoom);
}

#[derive(Component)]
struct ZoomInButton;

#[derive(Component)]
struct ZoomOutButton;

#[derive(Resource, Default)]
struct ZoomButtonClicked(bool);

fn setup_ui(mut commands: Commands) {
  commands
    .spawn(NodeBundle {
      style: Style {
        ..Default::default()
      },
      background_color: Color::NONE.into(),
      ..Default::default()
    })
    .with_children(|parent| {
      parent
        .spawn(ButtonBundle {
          style: Style { ..default() },
          background_color: Color::rgb(0.15, 0.15, 0.15).into(),
          ..Default::default()
        })
        .insert(ZoomInButton)
        .with_children(|parent| {
          parent.spawn(TextBundle::from_section(
            "+",
            TextStyle {
              font_size: 40.0,
              color: Color::WHITE,
              ..default()
            },
          ));
        });
    })
    .with_children(|parent| {
      parent
        .spawn(ButtonBundle {
          style: Style { ..default() },
          background_color: Color::rgb(0.15, 0.15, 0.15).into(),
          ..Default::default()
        })
        .insert(ZoomOutButton)
        .with_children(|parent| {
          parent.spawn(TextBundle::from_section(
            "-",
            TextStyle {
              font_size: 40.0,
              color: Color::WHITE,
              ..default()
            },
          ));
        });
    });
}

#[allow(clippy::type_complexity, dead_code, unused_variables)]
fn button_interaction_system(
  mut interaction_query: Query<
    (
      &Interaction,
      &mut BackgroundColor,
      &Children,
      Option<&ZoomInButton>,
      Option<&ZoomOutButton>,
    ),
    (Changed<Interaction>, With<Button>),
  >,
  mut fractal_zoom: ResMut<FractalZoom>,
  images: ResMut<Assets<Image>>,
  fractal_texture: Res<FractalTexture>,
  mut zoom_buttom_clicked: ResMut<ZoomButtonClicked>,
) {
  if let Some((interaction, mut background_color, _, zoom_in, zoom_out)) =
    interaction_query.iter_mut().next()
  {
    match *interaction {
      Interaction::Pressed => {
        zoom_buttom_clicked.0 = true;
        if zoom_in.is_some() {
          fractal_zoom.scale *= 0.9; // Zoom in
        } else if zoom_out.is_some() {
          fractal_zoom.scale *= 1.1; // Zoom out
        };
        *background_color = BackgroundColor(Color::rgb(0.35, 0.75, 0.35));
        // update_fractal(images, fractal_texture, fractal_zoom);
      },
      _ => {
        *background_color = BackgroundColor(Color::rgb(0.15, 0.15, 0.15));
      },
    }
  }
}

#[allow(dead_code, unused_variables)]
fn click_to_center(
  mut fractal_zoom: ResMut<FractalZoom>,
  windows: Query<&Window>,
  mouse_click: Res<ButtonInput<MouseButton>>,
  images: ResMut<Assets<Image>>,
  fractal_texture: Res<FractalTexture>,
  mut zoom_button_clicked: ResMut<ZoomButtonClicked>,
) {
  if zoom_button_clicked.0 {
    zoom_button_clicked.0 = false; // Reset the flag and return early
    return;
  }
  if let Some(window) = windows.iter().next() {
    if mouse_click.just_pressed(MouseButton::Left) {
      if let Some(cursor_position) = window.cursor_position() {
        // Convert cursor position to fractal coordinates
        let size = Vec2::new(window.width(), window.height());
        let fractal_x =
          (cursor_position.x - size.x / 2.0) * fractal_zoom.scale / size.x + fractal_zoom.center.0;
        let fractal_y =
          (cursor_position.y - size.y / 2.0) * fractal_zoom.scale / size.y + fractal_zoom.center.1;

        // Update fractal center
        fractal_zoom.center = (fractal_x, fractal_y);
        // update_fractal(images, fractal_texture, fractal_zoom);
      }
    }
  }
}

#[derive(Resource)]
struct FractalTexture(Handle<Image>);

#[derive(Resource)]
struct FractalZoom {
  scale: f32,
  center: (f32, f32),
}

// #[allow(dead_code, unused_variables)]
// fn zoom_with_mouse_wheel(
//   mut scroll_events: EventReader<MouseWheel>,
//   mut fractal_zoom: ResMut<FractalZoom>,
//   images: ResMut<Assets<Image>>,
//   fractal_texture: Res<FractalTexture>,
// ) {
//   for event in scroll_events.read() {
//     match event.y {
//       // Positive y value means scrolling up (zoom in)
//       // Negative y value means scrolling down (zoom out)
//       _ if event.y > 0.0 => fractal_zoom.scale *= 0.9, // Zoom in
//       _ => fractal_zoom.scale *= 1.1,                  // Zoom out
//     }
//   }
  // update_fractal(images, fractal_texture, fractal_zoom);
// }

// I let the following code even though it is mostly irrelevant now
// // Increasing the number of substeps will reduce the aliasing at the cost of
// // performance.
// const SUBSTEPS: f32 = 1.;
//
// fn update_fractal(
//   mut images: ResMut<Assets<Image>>,
//   fractal_texture: Res<FractalTexture>,
//   fractal_zoom: ResMut<FractalZoom>,
// ) {
//   if let Some(image) = images.get_mut(&fractal_texture.0) {
//     let size = image.texture_descriptor.size;
//     let width = size.width as usize;
//     let height = size.height as usize;
//
//     let scale = fractal_zoom.scale;
//     let center_x = fractal_zoom.center.0;
//     let center_y = fractal_zoom.center.1;
//
//     // Process in chunks using rayon in parallel!
//     image
//       .data
//       .par_chunks_mut(width * 4)
//       .enumerate()
//       .for_each(|(y, row)| {
//         for x in 0..width {
//           // Map pixel to fractal coordinate space
//           let cx = (x as f32 * scale / width as f32) + center_x - scale / 2.0;
//           let cy = (y as f32 * scale / height as f32) + center_y - scale / 2.0;
//           let c = Complex::new(-0.8, 0.156);
//           let mut total_color = Vec4::ZERO;
//           // Compute the mean color from mutiple points chosen randomly in the pixel
//           for _ in 0..SUBSTEPS as i32 {
//             let value = julia(
//               c,
//               cx + (random::<f32>() - 0.5) / width as f32,
//               cy + (random::<f32>() - 0.5) / height as f32,
//             );
//             total_color += color_gradient::DEFAULT_COLOR_GRADIENT
//               .get_color(value)
//               .rgba_to_vec4();
//           }
//
//           // Update image data
//           let offset = x * 4;
//           (0..4).for_each(|i| row[offset + i] = (total_color[i] / SUBSTEPS * 255.).round() as u8);
//         }
//       });
//   }
// }
//
// // Transforms the [0, 1] value to another value using a smoothstep-like function
// #[inline]
// fn smoother(iter: u8, z: Complex<f32>) -> f32 {
//   (iter as f32 - z.norm_squared().log2().max(1.).log2())
//     .max(0.)
//     .min(u8::MAX as f32)
//     / u8::MAX as f32
// }
//
// // Returns a f32 between 0 and 1 that represents the color of the pixel
// fn julia(c: Complex<f32>, x: f32, y: f32) -> f32 {
//   let mut z = Complex::new(x, y);
//
//   for i in 0..u8::MAX {
//     if z.norm_squared() > 4.0 {
//       return smoother(i, z);
//     }
//     z = z * z + c;
//   }
//   smoother(u8::MAX, z)
// }
