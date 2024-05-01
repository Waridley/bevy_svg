use bevy::{
    app::{App, Plugin},
    asset::AssetApp,
};

use crate::svg::Svg;

/// Plugin that renders [`Svg`](crate::svg::Svg)s in 2D
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_reflect::<Svg>();
    }
}
