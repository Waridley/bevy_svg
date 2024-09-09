use crate::plugin::Set;
use crate::render::resources::SvgMeshCache;
use crate::render::{FillTessellator, StrokeTessellator, SvgMesh2d, SvgMesh3d};
use crate::svg::Svg;
use bevy::asset::{AssetEvent, Assets, Handle};
use bevy::log::{debug, warn};
use bevy::prelude::{
    Changed, Commands, Entity, EventReader, IntoSystemConfigs, Last, Mesh, Query, Res, ResMut, Vec3,
};
use bevy::render::mesh::PrimitiveTopology;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::sprite::Mesh2dHandle;
use bevy::utils::HashMap;
use bevy::{
    app::{App, Plugin},
    asset::AssetApp,
};

/// Plugin that renders [`Svg`](crate::svg::Svg)s in 2D
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_reflect::<Svg>()
            .add_systems(Last, svg_mesh_2d_generator.in_set(Set::SVG));
    }
}

pub fn svg_mesh_2d_generator(
    mut cmds: Commands,
    mut svg_events: EventReader<AssetEvent<Svg>>,
    svgs: Res<Assets<Svg>>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(Entity, &SvgMesh2d, &Handle<Svg>, Option<&Mesh2dHandle>), Changed<SvgMesh2d>>,
    mut cache: ResMut<SvgMeshCache>,
    mut fill_tess: ResMut<FillTessellator>,
    mut stroke_tess: ResMut<StrokeTessellator>,
) {
    for (id, settings, svg, existing_mesh) in &query {
        let mesh = cache
            .entry(svg.clone_weak())
            .or_insert_with(HashMap::default)
            .entry(settings.clone().into())
            .or_insert_with(|| meshes.reserve_handle());
        if existing_mesh != Some(&Mesh2dHandle(mesh.clone())) {
            cmds.entity(id).insert(Mesh2dHandle(mesh.clone()));
        }
        let mesh = meshes.get_or_insert_with(mesh.id(), || {
            let mut mesh = Mesh::new(
                PrimitiveTopology::TriangleList,
                RenderAssetUsages::RENDER_WORLD,
            );
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, Vec::<Vec3>::new());
            mesh
        });
        if let Some(svg) = svgs.get(svg) {
            *mesh = svg.tessellate(
                &settings.clone().into(),
                &mut **fill_tess,
                &mut **stroke_tess,
            );
            debug!(?mesh);
        } else {
            debug!(?mesh, ?settings, "Svg asset not ready yet");
        }
    }

    for event in svg_events.read() {
        match event {
            AssetEvent::Added { id }
            | AssetEvent::LoadedWithDependencies { id }
            | AssetEvent::Modified { id } => {
                let Some(svg) = svgs.get(*id) else {
                    warn!(?id, "Svg asset is already missing");
                    continue;
                };
                let cache = cache
                    .entry(Handle::Weak(*id))
                    .or_insert_with(HashMap::default);
                for (key, mesh) in cache {
                    let settings = SvgMesh3d::from(key.clone());
                    let mut mesh = meshes.get_or_insert_with(mesh.id(), || {
                        let mut mesh = Mesh::new(
                            PrimitiveTopology::TriangleList,
                            RenderAssetUsages::RENDER_WORLD,
                        );
                        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, Vec::<Vec3>::new());
                        mesh
                    });
                    *mesh = svg.tessellate(&settings, &mut **fill_tess, &mut **stroke_tess);
                    debug!(?mesh);
                }
            }
            AssetEvent::Removed { id } => {
                cache.remove(&Handle::Weak(*id));
            }
            AssetEvent::Unused { .. } => {}
        }
    }
}
