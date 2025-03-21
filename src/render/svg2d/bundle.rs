//! Bevy [`Bundle`] representing an SVG entity.

use crate::{origin::Origin, render::SvgMesh3d, svg::Svg};
use bevy::{
    asset::{Handle, RenderAssetUsages},
    math::{Quat, Vec2},
    prelude::{default, Component},
};

/// A component that defines how a [Mesh2dHandle] should be generated for a given entity.
///
/// Ideally any computed values should be computed once and re-used throughout the
/// whole application, because [crate::render::resources::SvgMeshKey] will treat
/// floating-point values as meaningless bits.
///
/// Because [bevy::sprite::Mesh2dHandle] is a simple wrapper around `Handle<Mesh>`,
/// this type is converted to [crate::render::SvgMesh3d] for tesselation by setting
/// `depth` to `None`, and `rotation` using [Quat::from_rotation_z].
#[derive(Debug, Clone, Component)]
pub struct SvgMesh2d {
    /// Handle to the [Svg] asset.
    pub svg: Handle<Svg>,
    /// Modify the origin of the generated [Mesh].
    pub origin: Origin,
    /// Optionally override the computed size of the SVG by scaling vertices during tesselation.
    pub size: Option<Vec2>,
    /// Rotates all vertices around the origin before finalizing the mesh.
    pub rotation: f32,
    /// Tolerance passed to [lyon_tessellation::FillTessellator::tessellate].
    pub tolerance: f32,
    /// The [RenderAssetUsages] for the generated mesh.
    pub usages: RenderAssetUsages,
}

impl Default for SvgMesh2d {
    fn default() -> Self {
        Self {
            svg: default(),
            origin: default(),
            size: None,
            rotation: default(),
            tolerance: 0.001,
            usages: default(),
        }
    }
}

impl From<SvgMesh2d> for SvgMesh3d {
    fn from(value: SvgMesh2d) -> Self {
        Self {
            svg: value.svg,
            origin: value.origin,
            size: value.size,
            depth: None,
            rotation: Quat::from_rotation_z(value.rotation),
            tolerance: value.tolerance,
            usages: value.usages,
        }
    }
}
