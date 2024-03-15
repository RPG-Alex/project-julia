//! This code is mostly copied from the bevy example `post_processing.rs` and
//! modified to use a custom shader. This example is available at <https://github.com/bevyengine/bevy/blob/main/examples/shader/post_processing.rs>
//! Most of the unecessary boilerplate code as well as comments have been
//! removed for brevity. Some comments were added for parts that were modified
//! or for some parts that seem to be important, such as the shader loading and
//! the `PostProcessSettings` component.

use bevy::{
  core_pipeline::{
    core_2d::graph::{Core2d, Node2d},
    fullscreen_vertex_shader::fullscreen_shader_vertex_state,
  },
  ecs::query::QueryItem,
  prelude::*,
  render::{
    extract_component::{
      ComponentUniforms, ExtractComponent, ExtractComponentPlugin, UniformComponentPlugin,
    },
    render_graph::{
      NodeRunError, RenderGraphApp, RenderGraphContext, RenderLabel, ViewNode, ViewNodeRunner,
    },
    render_resource::{binding_types::uniform_buffer, *},
    renderer::{RenderContext, RenderDevice},
    texture::BevyDefault,
    view::ViewTarget,
    RenderApp,
  },
  window::WindowResized,
};

use crate::color_gradient;

pub struct PostProcessPlugin;

impl Plugin for PostProcessPlugin
{
  fn build(&self, app: &mut App)
  {
    app.add_plugins((
      ExtractComponentPlugin::<PostProcessSettings>::default(),
      UniformComponentPlugin::<PostProcessSettings>::default(),
    ));
    let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
      return;
    };
    render_app
      .add_render_graph_node::<ViewNodeRunner<PostProcessNode>>(Core2d, PostProcessLabel)
      .add_render_graph_edges(
        Core2d,
        (Node2d::Tonemapping, PostProcessLabel, Node2d::EndMainPassPostProcessing),
      );
  }

  fn finish(&self, app: &mut App)
  {
    let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
      return;
    };
    render_app.init_resource::<PostProcessPipeline>();
  }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct PostProcessLabel;

#[derive(Default)]
struct PostProcessNode;

impl ViewNode for PostProcessNode
{
  type ViewQuery = (&'static ViewTarget, &'static PostProcessSettings);

  fn run(
    &self,
    _graph: &mut RenderGraphContext,
    render_context: &mut RenderContext,
    (view_target, _post_process_settings): QueryItem<Self::ViewQuery>,
    world: &World,
  ) -> Result<(), NodeRunError>
  {
    let post_process_pipeline = world.resource::<PostProcessPipeline>();
    let pipeline_cache = world.resource::<PipelineCache>();
    let Some(pipeline) = pipeline_cache.get_render_pipeline(post_process_pipeline.pipeline_id)
    else {
      return Ok(());
    };
    let settings_uniform = world.resource::<ComponentUniforms<PostProcessSettings>>();
    let Some(settings_binding) = settings_uniform.binding() else {
      return Ok(());
    };
    let post_process = view_target.post_process_write();
    let bind_group = render_context.render_device().create_bind_group(
      "post_process_bind_group",
      &post_process_pipeline.layout,
      // To send more data to the shader, add more entries here and in the
      // PostProcessPipeline::from_world function.
      &BindGroupEntries::sequential((settings_binding.clone(),)),
    );
    let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
      label:                    Some("post_process_pass"),
      color_attachments:        &[Some(RenderPassColorAttachment {
        view:           post_process.destination,
        resolve_target: None,
        ops:            Operations::default(),
      })],
      depth_stencil_attachment: None,
      timestamp_writes:         None,
      occlusion_query_set:      None,
    });
    render_pass.set_render_pipeline(pipeline);
    render_pass.set_bind_group(0, &bind_group, &[]);
    render_pass.draw(0..3, 0..1);
    Ok(())
  }
}

#[derive(Resource)]
struct PostProcessPipeline
{
  layout:      BindGroupLayout,
  pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for PostProcessPipeline
{
  fn from_world(world: &mut World) -> Self
  {
    let render_device = world.resource::<RenderDevice>();
    let layout = render_device.create_bind_group_layout(
      "post_process_bind_group_layout",
      &BindGroupLayoutEntries::sequential(
        ShaderStages::FRAGMENT,
        // To send more data to the shader, add more entries here
        // and in the PostProcessNode::run function.
        (uniform_buffer::<PostProcessSettings>(false),),
      ),
    );
    // Load the shader here
    let shader = world.resource::<AssetServer>().load("shaders/julia.wgsl");
    let pipeline_id =
      world
        .resource_mut::<PipelineCache>()
        .queue_render_pipeline(RenderPipelineDescriptor {
          label:                Some("post_process_pipeline".into()),
          layout:               vec![layout.clone()],
          vertex:               fullscreen_shader_vertex_state(),
          fragment:             Some(FragmentState {
            shader,
            shader_defs: vec![],
            entry_point: "fragment".into(),
            targets: vec![Some(ColorTargetState {
              format:     TextureFormat::bevy_default(),
              blend:      None,
              write_mask: ColorWrites::ALL,
            })],
          }),
          primitive:            PrimitiveState::default(),
          depth_stencil:        None,
          multisample:          MultisampleState::default(),
          push_constant_ranges: vec![],
        });

    Self {
      layout,
      pipeline_id,
    }
  }
}

/// This is the component that will get passed to the shader.
/// The WGSL script contains a struct with the same name and fields.
#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
pub struct PostProcessSettings
{
  /// The color gradient to use for coloring the julia set.
  pub gradient: color_gradient::ColorGradient,
  // The view is a vec4 with the x and y being the position of the camera
  // and the z and w being the width and height of the camera on the complex plane.
  pub view:     Vec4,
  // time in seconds since the start of the program.
  pub time:     f32,
  // defines the speed of the animation
  pub pulse:    f32,
  // The maximum number of iterations to calculate the julia set.
  // Should change with the zoom level.
  pub max_iter: u32,
}

/// Setup the camera and the settings
pub fn setup(mut commands: Commands)
{
  // camera
  commands.spawn((
    Camera2dBundle::default(),
    // Add the setting to the camera.
    // This component is also used to determine on which camera
    // to run the post processing effect.
    PostProcessSettings {
      gradient: color_gradient::DEFAULT_COLOR_GRADIENT,
      view:     Vec4::new(0.0, 0.0, 2.0 * 16.0 / 9.0, 2.0),
      time:     0.0,
      pulse:    0.1,
      max_iter: 100,
    },
  ));
}

/// Updates the settings every frame. This function can be used as a template
/// for interactive communication with the shader and therefore implementing the
/// UI.
pub fn update_settings(
  mut settings: Query<&mut PostProcessSettings>,
  time: Res<Time>,
  mut resize_reader: EventReader<WindowResized>,
)
{
  for mut settings in settings.iter_mut() {
    settings.time = time.elapsed_seconds();
    // The following triggers on window resize and updates the aspect ratio
    for e in resize_reader.read() {
      // Adapt to the new aspect ratio with fixed height
      settings.view.z = settings.view.w * e.width / e.height;
    }
  }
}
