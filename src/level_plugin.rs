#![allow(unused_variables, unused_mut, non_snake_case, /*dead_code*/)]

use std::time::Instant;

use bevy::{gltf::GltfExtras, prelude::*};

#[cfg(ignore)]
use heron::prelude::*;

pub(crate) struct LevelPlugin;

#[derive(Default)]
struct LevelState {
    visible: bool,
    scene_path_override: Option<String>,
    duration__hack: Option<Instant>,
}

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        //

        info!("Building `{}`...", "LevelPlugin");

        app.init_resource::<LevelState>()
            .add_startup_system(level_plugin_startup_system);

        app.add_system(configure_named_entities);
        app.add_system(configure_player_character);

        app.add_system(debug_print_gltfextras);

        #[cfg(ignore)]
        app.add_system(log_player_character);

        app.add_system(move_player_character_on_title_screen__hack);

        //
    }
}

#[cfg(ignore)]
const DEFAULT_LEVEL_SCENE_PATH: &str = "gltf/levels/level_zero.gltf#Scene0";
const DEFAULT_LEVEL_SCENE_PATH: &str = "gltf/levels/level_zero_alt.gltf#Scene0";

const DEFAULT_LEVEL_CAMERA_HEIGHT: f32 = 15.0;

fn level_plugin_startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut level_state: ResMut<LevelState>,
    asset_server: Res<AssetServer>,
) {
    //

    info!("System setup...");

    level_state.duration__hack = Some(Instant::now());

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

//
// Initial hacky attempt to integrate the Blender-produced
// GLTF level files into the Bevy universe a bit more, automatically.
//
// TODO: Move this somewhere else?
//

#[derive(Component)]
struct PlayerCharacterMarker;

fn configure_named_entities(
    mut commands: Commands,
    mut query: Query<(Entity, &Name, &mut Transform), Added<Name>>, // Note: The `Added<>` filter needs to be *outside* query tuple to actually _filter_ as intended!
) {
    //

    for (mut entity, name, mut transform) in query.iter_mut() {
        //

        dbg!(entity, name, &transform);

        match name.as_str() {
            "PlayerCharacter" => {
                // TODO: When more marker components are added just return
                //       the marker component from this matcher?
                commands.entity(entity).insert(PlayerCharacterMarker);
            }
            _ => {}
        }

        //
    }

    //
}

#[cfg(ignore)]
const PLAYER_INITIAL_ACCEL: Vec3 = Vec3::X; // Vec3::X * 0.0;
#[cfg(ignore)]
const PLAYER_INITIAL_VELOCITY: Vec3 = Vec3::X; // Vec3::X * 0.5;

fn configure_player_character(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), Added<PlayerCharacterMarker>>, // Note: The `Added<>` filter needs to be *outside* query tuple to actually _filter_ as intended!
) {
    //

    for mut result in query.iter_mut() {
        //

        dbg!(&result);

        let (entity, mut transform) = result;

        // TODO: Remove this hardcoded test code.
        transform.translation.x = -4.0;
        transform.translation.y = -2.2;

        #[cfg(ignore)]
        commands
            .entity(entity)
            .insert(RigidBody::Dynamic)
            .insert(Acceleration::from_linear(PLAYER_INITIAL_ACCEL))
            .insert(Velocity::from_linear(PLAYER_INITIAL_VELOCITY));

        //
    }

    //
}

fn debug_print_gltfextras(
    mut query: Query<(Entity, &GltfExtras), Added<GltfExtras>>, // Note: The `Added<>` filter needs to be *outside* query tuple to actually _filter_ as intended!
) {
    //

    for result in query.iter_mut() {
        dbg!(result);
    }

    //
}

#[cfg(ignore)]
fn log_player_character(
    mut query: Query<(Entity, &mut Transform), (Changed<Transform>, With<PlayerCharacterMarker>)>,
) {
    //

    for mut result in query.iter_mut() {
        //

        let (entity, mut transform) = result;

        info!("  x: {:?}", transform.translation.x);

        //
    }

    //
}

fn move_player_character_on_title_screen__hack(
    mut query: Query<(Entity, &mut Transform), (With<PlayerCharacterMarker>,)>,
    mut level_state: ResMut<LevelState>,
) {
    //

    for mut result in query.iter_mut() {
        //

        let (entity, mut transform) = result;

        transform.translation.x = -4.0
            + ((level_state
                .duration__hack
                .expect("huh?")
                .elapsed()
                .as_secs_f32()
                / 2.0)
                % 2.0)
                * 4.0;

        //
    }

    //
}
