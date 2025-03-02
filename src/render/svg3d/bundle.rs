//! Bevy [`Bundle`] representing an SVG entity.

use crate::render::SvgMesh3d;
use crate::svg::Svg;
use bevy::{
    asset::Handle,
    ecs::bundle::Bundle,
    pbr::{Material, StandardMaterial},
    render::{
        mesh::Mesh,
        view::{InheritedVisibility, ViewVisibility, Visibility},
    },
    transform::components::{GlobalTransform, Transform},
};
use bevy::prelude::{Mesh3d, MeshMaterial3d};

/// A Bevy [`Bundle`] for generating a [Mesh] from an [Svg] asset.
#[allow(missing_docs)]
#[derive(Bundle)]
pub struct SvgMesh3dBundle<M: Material = StandardMaterial> {
    pub mesh_settings: SvgMesh3d,
    /// This placeholder will be replaced by the generated mesh handle.
    pub mesh: Mesh3d,
    pub material: MeshMaterial3d<M>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl<M: Material> Default for SvgMesh3dBundle<M> {
    /// Creates a default [`SvgMesh3dBundle`].
    fn default() -> Self {
        Self {
            mesh_settings: Default::default(),
            mesh: Default::default(),
            material: Default::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
        }
    }
}
