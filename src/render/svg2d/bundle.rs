//! Bevy [`Bundle`] representing an SVG entity.

use crate::origin::Origin;
use crate::render::SvgMesh3d;
use crate::svg::Svg;
use bevy::math::{Quat, Vec2};
use bevy::prelude::{default, Component, Mesh};
use bevy::{
    asset::Handle,
    ecs::bundle::Bundle,
    render::view::{InheritedVisibility, ViewVisibility, Visibility},
    sprite::{ColorMaterial, Material2d, Mesh2dHandle},
    transform::components::{GlobalTransform, Transform},
};

/// A Bevy [`Bundle`] for generating a [Mesh2dHandle] from an [Svg] asset.
#[allow(missing_docs)]
#[derive(Bundle)]
pub struct SvgMesh2dBundle<M: Material2d = ColorMaterial> {
    pub svg: Handle<Svg>,
    pub mesh_settings: SvgMesh2d,
    /// This placeholder will be replaced by the generated mesh handle.
    pub mesh_2d: Mesh2dHandle,
    pub material_2d: Handle<M>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl<M: Material2d> Default for SvgMesh2dBundle<M> {
    /// Creates a default [`SvgMesh2dBundle`].
    fn default() -> Self {
        Self {
            svg: Default::default(),
            mesh_settings: Default::default(),
            mesh_2d: Default::default(),
            material_2d: Default::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
        }
    }
}

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
    /// Modify the origin of the generated [Mesh].
    pub origin: Origin,
    /// Optionally override the computed size of the SVG by scaling vertices during tesselation.
    pub size: Option<Vec2>,
    /// Rotates all vertices around the origin before finalizing the mesh.
    pub rotation: f32,
    /// Tolerance passed to [lyon_tessellation::FillTessellator::tessellate].
    pub tolerance: f32,
}

impl Default for SvgMesh2d {
    fn default() -> Self {
        Self {
            origin: default(),
            size: None,
            rotation: default(),
            tolerance: 0.001,
        }
    }
}

impl From<SvgMesh2d> for SvgMesh3d {
    fn from(value: SvgMesh2d) -> Self {
        Self {
            origin: value.origin,
            size: value.size,
            depth: None,
            rotation: Quat::from_rotation_z(value.rotation),
            tolerance: value.tolerance,
        }
    }
}
