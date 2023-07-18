//! A shader that uses the GLSL shading language.

use bevy::{
  pbr::{MaterialPipeline, MaterialPipelineKey},
  prelude::*,
  reflect::{TypePath, TypeUuid},
  render::{
      mesh::MeshVertexBufferLayout,
      render_resource::{
          AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
      },
  },
};

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, Clone, TypeUuid, TypePath)]
#[uuid = "4ee9c363-1124-4113-890e-199d81b00281"]
pub struct CustomMaterial {
  #[uniform(0)]
  pub color: Color,
  #[texture(1)]
  #[sampler(2)]
  pub color_texture: Option<Handle<Image>>,
  #[uniform(3)]
  pub color_uv: Vec2,
  pub alpha_mode: AlphaMode,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
/// When using the GLSL shading language for your shader, the specialize method must be overridden.
impl Material for CustomMaterial {
  fn vertex_shader() -> ShaderRef {
      "shaders/custom_material.vert".into()
  }

  fn fragment_shader() -> ShaderRef {
      "shaders/custom_material.frag".into()
  }

  fn alpha_mode(&self) -> AlphaMode {
      self.alpha_mode
  }

  // Bevy assumes by default that vertex shaders use the "vertex" entry point
  // and fragment shaders use the "fragment" entry point (for WGSL shaders).
  // GLSL uses "main" as the entry point, so we must override the defaults here
  fn specialize(
      _pipeline: &MaterialPipeline<Self>,
      descriptor: &mut RenderPipelineDescriptor,
      _layout: &MeshVertexBufferLayout,
      _key: MaterialPipelineKey<Self>,
  ) -> Result<(), SpecializedMeshPipelineError> {
      descriptor.vertex.entry_point = "main".into();
      descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
      Ok(())
  }
}
