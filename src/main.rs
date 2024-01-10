use bevy::prelude::*;
use bevy::render::color::Color;
use bevy::text::TextStyle;
use bevy::ui::{Style, Interaction};
use bevy::ui::node_bundles::{ButtonBundle, NodeBundle, TextBundle};
use bevy::ecs::system::Resource;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use nalgebra::{Complex, Normed};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, setup_ui))
        .add_systems(Update, (update_fractal, button_interaction_system))
        .insert_resource(FractalZoom { 
            scale: 3.0,
            center: (-0.8, 0.156),
        })
        .run();
}


#[derive(Component)]
struct FractalMaterial;

fn setup(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
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
        TextureFormat::Rgba8Unorm
    );

    let image_handle = images.add(image);
    let texture_atlas = TextureAtlas::from_grid(image_handle.clone(), size, 1, 1, None, None);
    let atlas_handle = texture_atlases.add(texture_atlas);

    // Camera
    commands.spawn(Camera2dBundle::default());

    // Create a sprite to render the fractal texture atlas
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: atlas_handle,
            ..Default::default()
        },
        FractalMaterial,
    ));

    commands.insert_resource(FractalTexture(image_handle));

    // Initialize it
    commands.insert_resource(FractalZoom { 
        scale: 3.0, // Initial scale
        center: (0.0, 0.0), // Initial center
    });

}


#[derive(Component)]
struct ZoomInButton;

#[derive(Component)]
struct ZoomOutButton;

fn setup_ui(
    mut commands: Commands,
) {

    commands
    .spawn(NodeBundle {
        style: Style {
            ..Default::default()
        },
        background_color: Color::NONE.into(), 
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn(ButtonBundle {
            style: Style {
               ..default()
            },
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
        parent.spawn(ButtonBundle {
            style: Style {
               ..default()
            },
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


fn button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children, Option<&ZoomInButton>, Option<&ZoomOutButton>),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut fractal_zoom: ResMut<FractalZoom>,
) {
    for (interaction, mut background_color, zoom_in, zoom_out) in interaction_query.iter_mut() {

        match *interaction {
            Interaction::Pressed => {
                if zoom_in.is_some() {
                    fractal_zoom.scale *= 0.9; // Zoom in
                } else if zoom_out.is_some() {
                    fractal_zoom.scale *= 1.1; // Zoom out
                } ;
                *background_color = BackgroundColor(Color::rgb(0.35, 0.75, 0.35));
            }
            _ => {
                *background_color = BackgroundColor(Color::rgb(0.15, 0.15, 0.15));
            }
        }
    }
}







#[derive(Resource)]
struct FractalTexture(Handle<Image>);

#[derive(Resource)]
struct FractalZoom {
    scale: f64,
    center: (f64, f64), 
}

fn update_fractal(
    mut images: ResMut<Assets<Image>>,
    fractal_texture: Res<FractalTexture>,
    fractal_zoom: Res<FractalZoom>,
) {
    if let Some(image) = images.get_mut(&fractal_texture.0) {
        let size = image.texture_descriptor.size;
        let width = size.width as usize;
        let height = size.height as usize;

        let scale = fractal_zoom.scale;
        let center_x = fractal_zoom.center.0;
        let center_y = fractal_zoom.center.1;

        // Generate fractal data for each pixel
        for y in 0..height {
            for x in 0..width {
                // Map pixel to fractal coordinate space
                let cx = (x as f64 * scale / width as f64) + center_x - scale / 2.0;
                let cy = (y as f64 * scale / height as f64) + center_y - scale / 2.0;
                let c = Complex::new(-0.8, 0.156);

                let value = julia(c, cx, cy);

                // Convert the fractal value to a color (RGBA)
                let color = map_value_to_color(value);

                // Update image data
                let offset = (y * width + x) * 4;
                image.data[offset] = color.0;
                image.data[offset + 1] = color.1;
                image.data[offset + 2] = color.2;
                image.data[offset + 3] = color.3;
            }
        }
    }
}


// Function to map fractal value to RGBA color
fn map_value_to_color(value: u16) -> (u8, u8, u8, u8) {
    let outter = u16::MAX as f64;
    if value == u16::MAX {
        (255, 255, 255, 255) // White for points inside the Julia set
    } else if value < (outter * 0.0005) as u16 {
        (0, 0, 0, 255) // Black for points outside the Julia set
    } else {
        let v = value as u16;
        let (r, g, b) = (
            ((v * 6) % 256) as u8, // Red 
            ((v * 10) % 256) as u8, // Green 
            ((v * 4) % 256) as u8, // Blue 
        );
        (r, g, b, 255) 
    }
}



fn julia(c: Complex<f64>, x: f64, y: f64) -> u16 {
    let mut z = Complex::new(x,y);

    for i in 0..255 {
        if z.norm() > 2.0 {
            return i as u16;
        }
        z = z * z + c;
    }
    u16::MAX
}
