use crate::{
    plugin::Set,
    render::{resources::SvgMeshCache, FillTessellator, StrokeTessellator, SvgMesh3d},
    svg::Svg,
};
use bevy::{
    app::{App, Plugin},
    asset::AssetApp,
    prelude::*,
    utils::{HashMap, HashSet},
};

/// Plugin that renders [`Svg`](crate::svg::Svg)'s in 2D
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_reflect::<Svg>()
            .add_systems(Last, svg_mesh_3d_generator.in_set(Set::SVG));
    }
}

/// System that generates and inserts [`Mesh3d`]'s for entities with [`SvgMesh3d`].
pub fn svg_mesh_3d_generator(
    mut cmds: Commands,
    mut svg_events: EventReader<AssetEvent<Svg>>,
    svgs: Res<Assets<Svg>>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(Entity, &SvgMesh3d, Option<&Mesh3d>), Changed<SvgMesh3d>>,
    mut cache: ResMut<SvgMeshCache>,
    mut fill_tess: ResMut<FillTessellator>,
    mut stroke_tess: ResMut<StrokeTessellator>,
) {
    for (id, settings, existing_mesh) in &query {
        let mesh = cache
            .entry(settings.svg.clone_weak())
            .or_insert_with(HashMap::default)
            .entry(settings.clone().into())
            .or_insert_with(|| meshes.reserve_handle());
        if existing_mesh.map(|m3d| &m3d.0) != Some(mesh) {
            cmds.entity(id).insert(Mesh3d(mesh.clone()));
        }
        let mesh = meshes.get_or_insert_with(mesh.id(), || {
            debug!("Mesh does not yet exist. Inserting default rectangle to prevent panics on WASM.");
            // Empty meshes panic in WASM in bevy 0.15
            Rectangle::default().mesh().build()
        });
        if let Some(svg) = svgs.get(&settings.svg) {
            *mesh = svg.tessellate(settings, &mut fill_tess, &mut stroke_tess);
            debug!(?mesh);
        } else {
            debug!(?mesh, ?settings, "Svg asset not ready yet");
        }
    }

    let to_update = svg_events.read().filter_map(|event| {
        match event {
            AssetEvent::Added { id }
            | AssetEvent::LoadedWithDependencies { id }
            | AssetEvent::Modified { id } => {
                Some(*id)
            }
            AssetEvent::Removed { id } => {
                cache.remove(&Handle::Weak(*id));
                None
            }
            AssetEvent::Unused { .. } => None,
        }
    }).collect::<HashSet<_>>();
    
    for id in to_update {
        let Some(svg) = svgs.get(id) else {
            warn!(?id, "Svg asset is already unloaded");
            continue;
        };
        let handle = Handle::Weak(id);
        let cache = cache.entry(handle.clone()).or_insert_with(HashMap::default);
        for (key, mesh) in cache {
            let settings = SvgMesh3d::from((handle.clone(), key.clone()));
            let mesh = meshes
              .get_or_insert_with(mesh.id(), || Rectangle::default().mesh().build());
            *mesh = svg.tessellate(&settings, &mut fill_tess, &mut stroke_tess);
            if mesh.count_vertices() == 0 {
                warn!(?id, "Failed to tessellate Svg. Using default rectangle to prevent panics on WASM.");
                // Empty meshes panic in WASM in bevy 0.15
                *mesh = Rectangle::default().mesh().build();
            }
            debug!(?mesh);
        }
    }
}
