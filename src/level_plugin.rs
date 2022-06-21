#![allow(unused_variables, unused_mut, /*dead_code*/)]

use bevy::prelude::*;

pub(crate) struct LevelPlugin;

#[derive(Default)]
struct LevelState {
    visible: bool,
}

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        //

        info!("Building `{}`...", "LevelPlugin");

        app.init_resource::<LevelState>()
            .add_startup_system(level_plugin_startup_system);

        //
    }
}

const DEFAULT_LEVEL_SCENE_PATH: &str = "gltf/levels/level_zero.gltf#Scene0";

fn level_plugin_startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    //

    info!("System setup...");

    commands
        .spawn_bundle(TransformBundle::default())
        .insert(Name::new("GameLevel"))
        .with_children(|parent| {
            //

            parent.spawn_bundle(PointLightBundle {
                transform: Transform::from_xyz(0.0, 2.0, 2.0),
                ..default()
            });

            parent.spawn_bundle(PerspectiveCameraBundle {
                transform: Transform::from_xyz(0.0, 0.0, 7.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            });

            parent.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
                material: materials.add(Color::LIME_GREEN.into()),
                ..default()
            });

            //
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TransformBundle::default())
                .insert(Name::new("LoadedGltf"))
                .with_children(|gltf_parent| {
                    //
                    gltf_parent
                        .spawn_scene(asset_server.load(DEFAULT_LEVEL_SCENE_PATH));
                    //
                });
        });
    //
}
