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

#[derive(Debug, Clone, Component)]
pub struct SvgMesh3d {
    pub origin: Origin,
    pub size: Option<Vec2>,
    pub depth: Option<f32>,
    pub rotation: Quat,
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
