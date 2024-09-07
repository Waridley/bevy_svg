use bevy::prelude::*;
use bevy_svg::prelude::*;

#[path = "../common/lib.rs"]
mod common;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "origin_check".to_string(),
                resolution: (600., 600.).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((common::CommonPlugin, bevy_svg::prelude::SvgPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let svg = asset_server.load("box.svg");
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SvgMesh2dBundle {
        svg: svg.clone(),
        mesh_settings: SvgMesh2d {
            origin: Origin::Center,
            ..default()
        },
        ..Default::default()
    });
    commands.spawn((
        SvgMesh2dBundle {
            svg,
            mesh_settings: SvgMesh2d {
                origin: Origin::TopLeft,
                ..default()
            },
            ..Default::default()
        },
        common::DontChange,
    ));
}
