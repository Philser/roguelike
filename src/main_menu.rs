use bevy::{
    prelude::{
        App, AssetServer, BuildChildren, Button, ButtonBundle, Changed, Color, Commands, Component,
        DespawnRecursiveExt, Entity, Input, KeyCode, NodeBundle, Plugin, Query, Res, ResMut, Size,
        State, SystemSet, TextBundle, With,
    },
    text::TextStyle,
    ui::{
        AlignItems, BackgroundColor, FlexDirection, Interaction, JustifyContent, PositionType,
        Style, UiRect, Val,
    },
};

use crate::GameState;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct MainMenuPlugin {}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::MainMenu)
                .with_system(generate_main_menu)
                .label("main_menu_setup"),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::MainMenu)
                .with_system(exit_main_menu)
                .after("main_menu_setup"),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::MainMenu)
                .with_system(main_menu_interaction)
                .after("main_menu_setup"),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::MainMenu)
                .with_system(main_menu_selection)
                .after("main_menu_setup"),
        );
    }
}

#[derive(Component, EnumIter, Debug, Clone)]
enum MainMenuButtonAction {
    NewGame,
    Save,
    Load,
    Quit,
}

const HOVER_BUTTON_COLOR: Color = Color::rgb(0.750, 0.0975, 0.717);
const DEFAULT_BUTTON_COLOR: Color = Color::rgb(0.925, 0.0564, 0.940);

#[derive(Component)]
pub struct MainMenuUI {}

fn generate_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_font = asset_server.load("fonts/EduVICWANTBeginner-Regular.ttf");
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.7).into(),
            ..Default::default()
        })
        .insert(MainMenuUI {})
        .with_children(|parent| {
            for (i, menu_point) in MainMenuButtonAction::iter().enumerate() {
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Percent(33.3), Val::Percent(10.0)),
                                position_type: PositionType::Absolute,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                position: UiRect {
                                    bottom: Val::Percent(
                                        (MainMenuButtonAction::iter().len() - i) as f32 * 20.0,
                                    ),
                                    left: Val::Percent(33.3),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            background_color: DEFAULT_BUTTON_COLOR.into(),
                            ..Default::default()
                        },
                        menu_point.clone(),
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            format!("{:?}", menu_point),
                            TextStyle {
                                font: text_font.clone(),
                                font_size: 27.0,
                                color: Color::WHITE,
                            },
                        ));
                    });
            }
        });
}

fn exit_main_menu(
    mut app_state: ResMut<State<GameState>>,
    mut commands: Commands,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    main_menu_ui: Query<Entity, With<MainMenuUI>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        let main_menu = main_menu_ui
            .get_single()
            .expect("Fetching the main menu UI");
        commands.entity(main_menu).despawn_recursive();

        app_state.pop().expect("Popping main menu game state");
    }
    keyboard_input.clear();
}

fn main_menu_interaction(
    interaction_query: Query<
        (&Interaction, &MainMenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, action) in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            bevy::log::info!("Click!");
            match action {
                MainMenuButtonAction::NewGame => bevy::log::info!("New Game!"),
                MainMenuButtonAction::Save => todo!(),
                MainMenuButtonAction::Load => todo!(),
                MainMenuButtonAction::Quit => todo!(),
            }
        }
    }
}

fn main_menu_selection(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<Button>,
            With<MainMenuButtonAction>,
        ),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        *color = match *interaction {
            Interaction::Clicked | Interaction::Hovered => HOVER_BUTTON_COLOR.into(),
            Interaction::None => DEFAULT_BUTTON_COLOR.into(),
        };
    }
}
