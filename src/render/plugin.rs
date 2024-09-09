use crate::render::resources::{FillTessellator, StrokeTessellator, SvgMeshCache};
use bevy::app::{App, Plugin};

#[cfg(feature = "2d")]
use crate::render::svg2d;
#[cfg(feature = "3d")]
use crate::render::svg3d;

/// Plugin that renders [`Svg`](crate::svg::Svg)s in 2D
pub struct SvgPlugin;

impl Plugin for SvgPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(any(feature = "3d", feature = "2d"))]
        app.insert_resource(SvgMeshCache::default())
            .insert_resource(FillTessellator::default())
            .insert_resource(StrokeTessellator::default());

        #[cfg(feature = "2d")]
        app.add_plugins(svg2d::RenderPlugin);

        #[cfg(feature = "3d")]
        app.add_plugins(svg3d::RenderPlugin);
    }
}
