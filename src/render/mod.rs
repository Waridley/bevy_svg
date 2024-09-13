mod plugin;
pub(crate) mod tessellation;
mod vertex_buffer;

mod resources;
#[cfg(feature = "2d")]
mod svg2d;
#[cfg(feature = "3d")]
mod svg3d;

use bevy::prelude::*;
#[cfg(feature = "2d")]
pub use svg2d::{SvgMesh2d, SvgMesh2dBundle};
#[cfg(feature = "3d")]
pub use svg3d::SvgMesh3dBundle;

use crate::origin::Origin;
pub use plugin::SvgPlugin;
pub use resources::{FillTessellator, StrokeTessellator};

/// A component that defines how to generate a [Mesh] handle for a given entity.
///
/// Ideally any computed values should be computed once and re-used throughout the
/// whole application, because [crate::render::resources::SvgMeshKey] will treat
/// floating-point values as meaningless bits.
#[derive(Debug, Clone, Component)]
pub struct SvgMesh3d {
    /// Modify the origin of the generated [Mesh].
    pub origin: Origin,
    /// Optionally override the computed size of the SVG by scaling vertices during tesselation.
    pub size: Option<Vec2>,
    /// Optionally extrude the mesh along the `z` axis.
    // TODO: Add a way to define the z-axis origin, or which direction the mesh is extruded.
    pub depth: Option<f32>,
    /// Rotates all vertices around the origin before finalizing the mesh.
    pub rotation: Quat,
    /// Tolerance passed to [lyon_tessellation::FillTessellator::tessellate].
    // TODO: Replace with `FillOptions`? Would require extra enums for `SvgMeshKey`
    //   or a PR to lyon because `FillRule` and `Orientation` do not implement `Hash`.
    pub tolerance: f32,
}

impl Default for SvgMesh3d {
    fn default() -> Self {
        Self {
            origin: default(),
            size: None,
            depth: None,
            rotation: default(),
            tolerance: 0.001,
        }
    }
}
