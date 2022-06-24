#![allow(unused_variables, unused_mut, /*dead_code*/)]

use bevy::prelude::*;

pub(crate) struct LevelPlugin;

#[derive(Default)]
struct LevelState {
    visible: bool,
    scene_path_override: Option<String>,
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

#[cfg(ignore)]
const DEFAULT_LEVEL_SCENE_PATH: &str = "gltf/levels/level_zero.gltf#Scene0";
const DEFAULT_LEVEL_SCENE_PATH: &str = "gltf/levels/level_zero_alt.gltf#Scene0";

const DEFAULT_LEVEL_CAMERA_HEIGHT: f32 = 30.0;

fn level_plugin_startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut level_state: ResMut<LevelState>,
    asset_server: Res<AssetServer>,
) {
    //

    info!("System setup...");

    if let Some(scene_path) = std::env::var_os("QLAD_SECRET_LEVEL") {
        level_state.scene_path_override = scene_path.into_string().ok();
        info!(
            "Found environment scene path override: {:?}",
            &level_state.scene_path_override
        );
    }

    commands
        .spawn_bundle(TransformBundle::default())
        .insert(Name::new("GameLevel"))
        .with_children(|parent| {
            //

            parent
                .spawn_bundle(PointLightBundle {
                    point_light: PointLight {
                        shadows_enabled: true,
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 2.0, 2.0),
                    ..default()
                })
                .insert(Visibility {
                    #[cfg(ignore)]
                    is_visible: level_state.scene_path_override.is_none(),
                    is_visible: false,
                });

            parent.spawn_bundle(PerspectiveCameraBundle {
                transform: Transform::from_xyz(0.0, 0.0, DEFAULT_LEVEL_CAMERA_HEIGHT)
                    .looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            });

            parent
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
                    material: materials.add(Color::LIME_GREEN.into()),
                    ..default()
                })
                .insert(Visibility {
                    #[cfg(ignore)]
                    is_visible: level_state.scene_path_override.is_none(),
                    is_visible: false,
                });

            //
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TransformBundle::default())
                .insert(Name::new("LoadedGltf"))
                .with_children(|gltf_parent| {
                    //
                    gltf_parent.spawn_scene(
                        asset_server.load(
                            level_state
                                .scene_path_override
                                .as_ref()
                                .unwrap_or(&DEFAULT_LEVEL_SCENE_PATH.to_string()),
                        ),
                    );
                    //
                });
        });
    //
}
