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

/// A Bevy [`Bundle`] representing an SVG entity.
#[allow(missing_docs)]
#[derive(Bundle)]
pub struct SvgMesh3dBundle<M: Material = StandardMaterial> {
    pub svg: Handle<Svg>,
    pub mesh_settings: SvgMesh3d,
    pub mesh: Handle<Mesh>,
    pub material: Handle<M>,
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
            svg: Default::default(),
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
