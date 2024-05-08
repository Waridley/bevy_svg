//! Bevy [`Bundle`] representing an SVG entity.

use bevy::{
    asset::Handle,
    ecs::bundle::Bundle,
    render::view::{InheritedVisibility, ViewVisibility, Visibility},
    sprite::{ColorMaterial, Material2d, Mesh2dHandle},
    transform::components::{GlobalTransform, Transform},
};

use crate::svg::Svg;

/// A Bevy [`Bundle`] representing an SVG entity.
#[allow(missing_docs)]
#[derive(Bundle)]
pub struct Svg2dBundle<M: Material2d = ColorMaterial> {
    pub svg: Handle<Svg>,
    pub mesh_2d: Mesh2dHandle,
    pub material_2d: Handle<M>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl<M: Material2d> Default for Svg2dBundle<M> {
    /// Creates a default [`Svg2dBundle`].
    fn default() -> Self {
        Self {
            svg: Default::default(),
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
