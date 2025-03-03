//! Contains the plugin and its helper types.
//!
//! The [`Svg2dBundle`](crate::bundle::Svg2dBundle) provides a way to display an `SVG`-file
//! with minimal boilerplate.
//!
//! ## How it works
//! The user creates/loades a [`Svg2dBundle`](crate::bundle::Svg2dBundle) in a system.
//!
//! Then, in the [`Set::SVG`](Set::SVG), a mesh is created for each loaded [`Svg`] bundle.
//! Each mesh is then extracted in the [`RenderSet::Extract`](bevy::render::RenderSet) and added to the
//! [`RenderWorld`](bevy::render::RenderWorld).
//! Afterwards it is queued in the [`RenderSet::Queue`](bevy::render::RenderSet) for actual drawing/rendering.


#[cfg(feature = "2d")]
use bevy::render::mesh::Mesh2d;
use bevy::{
    app::{App, Plugin},
    asset::Handle,
    ecs::{
        entity::Entity,
        schedule::SystemSet,
    },
    render::mesh::Mesh,
};

use crate::{render, svg::Svg};

/// Sets for this plugin.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum Set {
    /// Set in which [`Svg2dBundle`](crate::bundle::Svg2dBundle)s get drawn.
    SVG,
}

/// A plugin that makes sure your [`Svg`]s get rendered
pub struct SvgRenderPlugin;

impl Plugin for SvgRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(render::SvgPlugin);
    }
}

#[cfg(feature = "2d")]
#[cfg(not(feature = "3d"))]
type SvgMeshComponents = (
    Entity,
    &'static Handle<Svg>,
    Option<&'static mut Mesh2dHandle>,
    Option<()>,
    Option<()>,
);
#[cfg(not(feature = "2d"))]
#[cfg(feature = "3d")]
type SvgMeshComponents = (
    Entity,
    &'static Handle<Svg>,
    Option<()>,
    Option<&'static mut Handle<Mesh>>,
    Option<render::SvgMesh3d>,
);
#[cfg(all(feature = "2d", feature = "3d"))]
type SvgMeshComponents = (
    Entity,
    &'static Handle<Svg>,
    Option<&'static mut Mesh2d>,
    Option<&'static mut Handle<Mesh>>,
    Option<render::SvgMesh3d>,
);
