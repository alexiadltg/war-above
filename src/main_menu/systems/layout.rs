use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::main_menu::components::*;
use crate::main_menu::styles::*;

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    build_main_menu(&mut commands, &asset_server);

    let window = window_query.get_single().unwrap();

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        },
        CameraMenu {},
    ));
}

pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>>,
    camera_query: Query<Entity, With<CameraMenu>>,
) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
    if let Ok(camera_entity) = camera_query.get_single() {
        commands.entity(camera_entity).despawn();
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
        .spawn((
            NodeBundle {
                style: main_menu_style,
                ..default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            //Title
            parent
                .spawn(NodeBundle {
                    style: title_style,
                    ..default()
                })
                .with_children(|parent| {
                    // Title text
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Bevy Voxel Game",
                                get_title_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });

            //Bottom holder
            parent
                .spawn(NodeBundle {
                    style: bottom_holder,

                    ..Default::default()
                })
                .with_children(|parent| {
                    //Image Holder

                    parent
                        .spawn(NodeBundle {
                            style: image_holder,
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            //Image Loader
                            parent.spawn(ImageBundle {
                                style: image_style,
                                image: asset_server.load("campfire.png").into(),
                                ..Default::default()
                            });
                        });

                    // Menu
                    parent
                        .spawn(NodeBundle {
                            style: text_menu,
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // Play Button
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: button_style,
                                        background_color: normal_button_color.into(),
                                        ..default()
                                    },
                                    PlayButton {},
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle {
                                        text: Text {
                                            sections: vec![TextSection::new(
                                                "Play",
                                                get_button_text_style(&asset_server),
                                            )],
                                            alignment: TextAlignment::Center,
                                            ..default()
                                        },
                                        ..default()
                                    });
                                });
                            //options button
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: button_style,
                                        background_color: normal_button_color.into(),
                                        ..default()
                                    },
                                    OptionButton {},
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle {
                                        text: Text {
                                            sections: vec![TextSection::new(
                                                "Settings",
                                                get_button_text_style(&asset_server),
                                            )],
                                            alignment: TextAlignment::Center,
                                            ..default()
                                        },
                                        ..default()
                                    });
                                });

                            //  Quit Button
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: button_style,
                                        background_color: normal_button_color.into(),
                                        ..default()
                                    },
                                    QuitButton {},
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle {
                                        text: Text {
                                            sections: vec![TextSection::new(
                                                "Quit",
                                                get_button_text_style(&asset_server),
                                            )],
                                            alignment: TextAlignment::Center,
                                            ..default()
                                        },
                                        ..default()
                                    });
                                });
                        });
                });
        })
        .id();

    main_menu_entity
}
