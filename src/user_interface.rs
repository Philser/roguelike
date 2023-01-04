use std::collections::HashSet;

use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::{
    components::{combat_stats::CombatStats, item::AreaOfEffect, position::Position},
    inventory::components::WantsToUseItem,
    map::{game_map::GameMap, MainCamera, Tile},
    player::Player,
    utils::render::map_pos_to_screen_pos,
    viewshed::{generate_viewshed, Viewshed},
    GameConfig, GameState, ScreenDimensions, TileProperties,
};

const ACTION_LOG_MAX_LINES: usize = 7;

const TARGETING_MODE_TILE_COLOR: Color = Color::rgba(242.0, 36.0, 139.0, 0.05);

const TARGETING_MODE_SELECTION_COLOR: Color = Color::BEIGE;
#[derive(Clone)]
pub struct ActionLog {
    pub entries: Vec<String>,
}
pub struct UIPlugin {}

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system_set(
                SystemSet::on_update(GameState::Render)
                    .with_system(render_ui)
                    .after("render_map"),
            )
            .add_system_set(
                SystemSet::on_enter(GameState::AwaitingActionInput).with_system(render_ui),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Targeting).with_system(render_target_mode),
            );
    }
}

#[derive(Component)]
pub struct HealthBar {}

#[derive(Component)]
pub struct HealthText {}

#[derive(Component)]
pub struct ActionLogText {}

///
#[derive(Component)]
pub struct TargetingModeContext {
    pub range: u32,
    pub item: Entity,
}

#[derive(Component)]
pub struct TargetingTile {}

pub struct UIFont(Handle<Font>);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, game_config: Res<GameConfig>) {
    commands.spawn_bundle(UiCameraBundle::default());

    let font_handle: Handle<Font> = asset_server.load("fonts/EduVICWANTBeginner-Regular.ttf");
    commands.insert_resource(UIFont(font_handle.clone()));

    commands.insert_resource(ActionLog {
        entries: vec!["Adventure awaits!".to_owned()],
    });

    let mut commands_builder = commands.spawn_bundle(NodeBundle {
        style: Style {
            justify_content: JustifyContent::SpaceBetween,
            size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
            ..default()
        },
        color: Color::PURPLE.into(),
        ..default()
    });
    spawn_health_bar(
        &mut commands_builder,
        font_handle.clone(),
        game_config.gameplay_settings.player_starting_health as f32,
    );
    spawn_action_log(&mut commands_builder, font_handle);
}

fn spawn_health_bar(
    commands: &mut EntityCommands,
    text_font: Handle<Font>,
    player_starting_health: f32,
) {
    commands.with_children(|parent| {
        parent
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(player_starting_health * 3.0), Val::Percent(10.0)),
                    position_type: PositionType::Absolute,
                    position: Rect {
                        left: Val::Percent(50.0),
                        top: Val::Percent(25.0),
                        ..default()
                    },
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                color: Color::RED.into(),
                ..default()
            })
            .insert(HealthBar {})
            .with_children(|parent| {
                parent
                    .spawn_bundle(TextBundle {
                        style: Style {
                            margin: Rect::all(Val::Px(5.0)),
                            ..default()
                        },
                        text: Text::with_section(
                            format!("{}/{}", player_starting_health, player_starting_health),
                            TextStyle {
                                font: text_font,
                                font_size: 27.0,
                                color: Color::WHITE,
                            },
                            Default::default(),
                        ),
                        ..default()
                    })
                    .insert(HealthText {});
            });
    });
}

fn spawn_action_log(commands: &mut EntityCommands, text_font: Handle<Font>) {
    commands.with_children(|parent| {
        parent
            .spawn_bundle(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::ColumnReverse,
                    size: Size::new(Val::Percent(40.0), Val::Percent(100.0)),
                    ..default()
                },
                color: Color::rgb(0.8, 0.8, 1.0).into(),
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn_bundle(TextBundle {
                        style: Style {
                            margin: Rect {
                                left: Val::Percent(1.0),
                                right: Val::Percent(1.0),
                                bottom: Val::Percent(1.0),
                                ..default()
                            },
                            size: Size::new(Val::Px(300.0), Val::Px(400.0)),
                            ..default()
                        },
                        text: Text::with_section(
                            "",
                            TextStyle {
                                font: text_font,
                                font_size: 23.0,
                                color: Color::WHITE,
                            },
                            Default::default(),
                        ),
                        ..default()
                    })
                    .insert(ActionLogText {});
            });
    });
}

fn render_ui(
    player_query: Query<&CombatStats, With<Player>>,
    mut healthbar_query: Query<&mut Style, With<HealthBar>>,
    mut healthtext_query: Query<&mut Text, (With<HealthText>, Without<ActionLogText>)>,
    mut actionlogtext_query: Query<&mut Text, (With<ActionLogText>, Without<HealthText>)>,
    action_log: Res<ActionLog>,
    default_font: Res<UIFont>,
) {
    let player = player_query
        .get_single()
        .expect("Got more or less than exactly one Player entity while rendering UI");

    let mut healthbar = healthbar_query
        .get_single_mut()
        .expect("Found more or less than exactly one Healthbar entity while rendering UI");

    healthbar.size.width = Val::Px(player.hp as f32 * 3.0);

    let mut healthtext = healthtext_query
        .get_single_mut()
        .expect("Found more or less than exactly one Healthtext entity while rendering UI");

    // We only care for the first section
    healthtext.sections[0].value = format!("{}/{}", player.hp, player.max_hp);

    let actionlogtext = actionlogtext_query
        .get_single_mut()
        .expect("Found more or less than exactly one Action log text entity while rendering UI");

    render_action_log(actionlogtext, action_log, default_font);
}

// TODO: Instead of creating new text sections on every rendering
// we could just manipulate the existing text sections.
// This might save some computing power.
fn render_action_log(
    mut action_log_text: Mut<Text>,
    action_log: Res<ActionLog>,
    font: Res<UIFont>,
) {
    let mut sections: Vec<TextSection> = vec![];
    let len = &action_log.entries.len();
    let mut to_iterate = action_log.entries.to_vec();
    if *len > ACTION_LOG_MAX_LINES {
        let entries = &action_log.entries.as_slice()[*len - ACTION_LOG_MAX_LINES..];
        to_iterate = entries.to_vec();
    }

    for entry in to_iterate {
        sections.push(TextSection {
            value: format!("{}\n", entry.clone()),
            style: TextStyle {
                font: font.0.clone(),
                font_size: 23.0,
                color: Color::BLACK,
            },
        })
    }
    action_log_text.sections = sections;
}

fn render_target_mode(
    mut commands: Commands,
    app_state: ResMut<State<GameState>>,
    viewshed_player_query: Query<(&Position, &Viewshed, With<Player>)>,
    target_mode_query: Query<(Entity, &TargetingModeContext)>,
    tiles_query: Query<(&Position, With<Tile>)>,
    mut targeting_tiles_query: ParamSet<(
        Query<(&GlobalTransform, &mut Sprite, &Position), With<TargetingTile>>,
        Query<Entity, With<TargetingTile>>,
    )>,
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    game_config: Res<GameConfig>,
    mouse_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    aoe_items: Query<&AreaOfEffect>,
    game_map: Res<GameMap>,
) {
    let (target_ctx_entity, target_ctx) = target_mode_query
        .get_single()
        .expect("Expected a single TargetModeContext component in render_target_mode");

    if keyboard_input.just_pressed(KeyCode::Escape) {
        finish_targeting_mode(
            &mut commands,
            app_state,
            target_ctx,
            target_ctx_entity,
            targeting_tiles_query.p1(),
            None,
            None,
            &game_map,
        );
        return;
    }

    if targeting_tiles_query.p0().is_empty() {
        // We just entered targeting mode
        let (player_pos, viewshed, _) = viewshed_player_query
            .get_single()
            .expect("Expected a single player viewshed in render_target_mode");

        create_targeting_tiles(
            commands,
            player_pos,
            viewshed,
            target_ctx,
            tiles_query,
            &game_config.screen_dimensions,
            &game_config.tile_properties,
        );
    } else {
        // We are waiting for the player to pick a target
        let window = windows.get_primary().unwrap();

        let aoe = aoe_items.get(target_ctx.item).ok();

        if let Some(mouse_pos) = window.cursor_position() {
            let target_pos =
                draw_cursor_pos(window, mouse_pos, q_camera, targeting_tiles_query.p0());

            if mouse_input.just_pressed(MouseButton::Left) && target_pos.is_some() {
                finish_targeting_mode(
                    &mut commands,
                    app_state,
                    target_ctx,
                    target_ctx_entity,
                    targeting_tiles_query.p1(),
                    target_pos,
                    aoe,
                    &game_map,
                );
                return;
            }
        }
    }

    fn finish_targeting_mode(
        commands: &mut Commands,
        mut app_state: ResMut<State<GameState>>,
        target_ctx: &TargetingModeContext,
        target_ctx_entity: Entity,
        targeting_tiles_entity_query: Query<Entity, With<TargetingTile>>,
        target_pos: Option<Position>,
        aoe: Option<&AreaOfEffect>,
        game_map: &GameMap,
    ) {
        let mut perfomed_action = false;

        // If we have no target, just exit target mode
        if let Some(pos) = target_pos {
            let mut targets: Vec<Position> = vec![];
            if aoe.is_some() {
                // Fetch all targets from area
                let viewshed = generate_viewshed(&pos, game_map, target_ctx.range as usize, true);
                for x in 0..viewshed.width {
                    for y in 0..viewshed.height {
                        if viewshed.is_in_fov(x, y) {
                            let pos = Position {
                                x: x as i32,
                                y: y as i32,
                            };
                            targets.push(pos);
                        }
                    }
                }
            } else {
                targets.push(pos)
            }

            commands.spawn().insert(WantsToUseItem {
                entity: target_ctx.item,
                targets: Some(targets),
            });
            perfomed_action = true;
        }

        // Delete Targeting Tiles
        let entities: Vec<Entity> = targeting_tiles_entity_query.iter().collect();
        for entity in entities {
            commands.entity(entity).despawn();
        }

        commands.entity(target_ctx_entity).despawn();

        let mut next_state = GameState::AwaitingActionInput;
        if perfomed_action {
            next_state = GameState::PlayerTurn;
        }
        app_state
            .set(next_state)
            .expect("Setting GameState in targeting mode");
    }
}

fn create_targeting_tiles(
    mut commands: Commands,
    player_pos: &Position,
    player_viewshed: &Viewshed,
    target_ctx: &TargetingModeContext,
    tiles_query: Query<(&Position, With<Tile>)>,
    screen_dimensions: &ScreenDimensions,
    tile_properties: &TileProperties,
) {
    let visible_positions = &player_viewshed.visible_tiles;

    let mut pos_in_range: HashSet<&Position> = HashSet::new();
    for visible_pos in visible_positions {
        if player_pos.get_airline_distance(visible_pos) <= target_ctx.range as i32 {
            pos_in_range.insert(visible_pos);
        }
    }

    let scaled_tile_size = tile_properties.get_scaled_tile_size();
    for (pos, _) in tiles_query.iter() {
        if pos_in_range.contains(pos) {
            commands
                .spawn()
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: TARGETING_MODE_TILE_COLOR,
                        custom_size: Some(Vec2::new(scaled_tile_size, scaled_tile_size)),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: map_pos_to_screen_pos(
                            pos,
                            10.0,
                            tile_properties.tile_size,
                            screen_dimensions,
                        ),
                        scale: Vec3::new(
                            tile_properties.tile_scale,
                            tile_properties.tile_scale,
                            1.0,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Position { x: pos.x, y: pos.y })
                .insert(TargetingTile {});
        }
    }
}

fn draw_cursor_pos(
    window: &Window,
    mouse_pos: Vec2,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut targeting_tiles_query: Query<
        (&GlobalTransform, &mut Sprite, &Position),
        With<TargetingTile>,
    >,
) -> Option<Position> {
    let (camera, camera_transform) = camera_query.single();
    // get the size of the window
    let window_size = Vec2::new(window.width() as f32, window.height() as f32);

    // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
    let ndc = (mouse_pos / window_size) * 2.0 - Vec2::ONE;

    // matrix for undoing the projection and camera transform
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix.inverse();

    // use it to convert ndc to world-space coordinates
    let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

    // reduce it to a 2D value
    let world_pos: Vec2 = world_pos.truncate();
    // cursor is in the screen
    let mut target_position: Option<Position> = None;
    for (transform, mut sprite, position) in targeting_tiles_query.iter_mut() {
        if transform.translation.x <= world_pos.x
            && transform.translation.x + sprite.custom_size.unwrap().x >= world_pos.x
            && transform.translation.y <= world_pos.y
            && transform.translation.y + sprite.custom_size.unwrap().y >= world_pos.y
        {
            sprite.color = TARGETING_MODE_SELECTION_COLOR;
            target_position = Some(position.clone());
        } else {
            sprite.color = TARGETING_MODE_TILE_COLOR
        }
    }

    return target_position;
}
