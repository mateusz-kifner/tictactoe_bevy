#![allow(clippy::type_complexity)]

use bevy::a11y::accesskit::Orientation;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::touch::TouchPhase;
use bevy::prelude::*;
use bevy::window::WindowResized;
use bevy::{app::App, window::PrimaryWindow};

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(States, Debug, Hash, Copy, PartialEq, Eq, Clone, Default)]
pub enum GameState {
    #[default]
    Running,

    Paused,
}

#[derive(Resource)]
struct Board {
    grid: [[char; 3]; 3],
}

#[derive(Resource)]
struct Data {
    turn: u32,
    won: char,
    input_position: Vec2,
}

#[derive(Component)]
struct Divider;

fn spawn_board(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(-70.0, 0.0, 0.0),
        texture: asset_server.load("textures/line.png"),
        ..Default::default()
    });
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(70.0, 0.0, 0.0),
        texture: asset_server.load("textures/line.png"),
        ..Default::default()
    });
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(0.0, -70.0, 0.0)
            .with_rotation(Quat::from_rotation_z((90.0_f32).to_radians())),

        texture: asset_server.load("textures/line.png"),
        ..Default::default()
    });
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(0.0, 70.0, 0.0)
            .with_rotation(Quat::from_rotation_z((90.0_f32).to_radians())),
        texture: asset_server.load("textures/line.png"),
        ..Default::default()
    });
}

fn print_board(board: &Board) {
    for (i, row) in board.grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            print!("{}", col);
        }
        println!()
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn check_win(grid: &[[char; 3]; 3], player: char) -> bool {
    for i in 0..=2 {
        if (grid[0][i] == player && grid[1][i] == player && grid[2][i] == player) {
            return true;
        }
    }
    for i in 0..=2 {
        if (grid[i][0] == player && grid[i][1] == player && grid[i][2] == player) {
            return true;
        }
    }
    if (grid[0][0] == player && grid[1][1] == player && grid[2][2] == player) {
        return true;
    }
    if (grid[2][0] == player && grid[1][1] == player && grid[0][2] == player) {
        return true;
    }

    return false;
}

fn mouse_input(
    buttons: Res<Input<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut data: ResMut<Data>,
) {
    let window = window_query.get_single().unwrap();

    if buttons.pressed(MouseButton::Left) {
        data.input_position = window.cursor_position().unwrap();
    }
}

fn touch_input(mut touches: EventReader<TouchInput>, mut data: ResMut<Data>) {
    if touches.len() > 0 {
        let touch = touches.iter().nth(0).unwrap();
        if touch.phase == TouchPhase::Ended {
            data.input_position = touch.position
        }
    }
}

fn evaluate_game(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut board: ResMut<Board>,
    mut data: ResMut<Data>,
    asset_server: Res<AssetServer>,
    game_state: Res<State<GameState>>,
) {
    let window = window_query.get_single().unwrap();
    if data.input_position.x != -1.0 {
        let board_pos_x = (window.resolution.width() - 400.0) / 2.0;
        let board_pos_y = (window.resolution.height() - 400.0) / 2.0;
        let pos = data.input_position;
        let x = ((pos.x - board_pos_x) / 133.0);
        let y = ((pos.y - board_pos_y) / 133.0);
        println!(
            "{}, {}, {}, {}, {}, {}",
            x as u32, y as u32, pos.x as u32, pos.y as u32, board_pos_x as u32, board_pos_y as u32
        );

        if (x >= 0.0 && x < 3.0 && y >= 0.0 && y < 3.0 && board.grid[x as usize][y as usize] == ' ')
        {
            if (data.turn % 2 == 0) {
                commands.spawn(SpriteBundle {
                    transform: Transform::from_xyz(
                        (133 * x as u32) as f32 + 67.5 - 200.0,
                        (133 * (3.0 - y) as u32) as f32 + 67.5 - 200.0,
                        0.0,
                    ),
                    texture: asset_server.load("textures/cross.png"),
                    ..default()
                });
                board.grid[x as usize][y as usize] = 'X';
            } else {
                commands.spawn(SpriteBundle {
                    transform: Transform::from_xyz(
                        (133 * x as u32) as f32 + 67.5 - 200.0,
                        (133 * (3.0 - y) as u32) as f32 + 67.5 - 200.0,
                        0.0,
                    ),
                    texture: asset_server.load("textures/circle.png"),
                    ..Default::default()
                });
                board.grid[x as usize][y as usize] = 'O';
            }
            if check_win(&board.grid, 'O') {
                data.won = 'O';
            }
            if check_win(&board.grid, 'X') {
                data.won = 'X';
            }

            if (data.won != ' ') {
                println!("{}", window.width());
                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.9, 0.9, 0.7),
                        custom_size: Some(Vec2::new(window.width() * 0.8, 100.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, 5.0),
                    ..default()
                });
                commands.spawn(Text2dBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 10.0),
                    text: Text {
                        sections: vec![TextSection {
                            value: format!("Player {} won", data.won.to_string()),
                            style: TextStyle {
                                color: Color::rgb(0.0, 0.0, 0.0),
                                font_size: 96.0,
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            },
                        }],
                        ..Default::default()
                    },
                    ..Default::default()
                });
                commands.insert_resource(NextState(Some(GameState::Paused)))
            } else if (data.turn == 8) {
                println!("{}", window.width());
                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.9, 0.9, 0.7),
                        custom_size: Some(Vec2::new(window.width() * 0.8, 100.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, 5.0),
                    ..default()
                });
                commands.spawn(Text2dBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 10.0),
                    text: Text {
                        sections: vec![TextSection {
                            value: format!("It's a Tie"),
                            style: TextStyle {
                                color: Color::rgb(0.0, 0.0, 0.0),
                                font_size: 96.0,
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            },
                        }],
                        ..Default::default()
                    },
                    ..Default::default()
                });
                commands.insert_resource(NextState(Some(GameState::Paused)))
            }

            data.turn += 1;
        }
        data.input_position = Vec2 { x: -1.0, y: -1.0 };
    }
}

/// This system shows how to respond to a window being resized.
/// Whenever the window is resized, the text will update with the new resolution.
fn on_resize_system(mut resize_reader: EventReader<WindowResized>) {
    for e in resize_reader.iter() {
        // When resolution is being changed
        println!("{:.1} x {:.1}", e.width, e.height);
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .insert_resource(ClearColor(BACKGROUND_COLOR))
            .insert_resource(Board {
                grid: [[' '; 3]; 3],
            })
            .insert_resource(Data {
                turn: 0,
                won: ' ',
                input_position: Vec2 { x: -1.0, y: -1.0 },
            })
            .add_systems(Startup, (spawn_camera, spawn_board))
            .add_systems(Update, evaluate_game.run_if(in_state(GameState::Running)))
            .add_systems(Update, on_resize_system);
        #[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
        {
            app.add_systems(Update, mouse_input);
        }
        #[cfg(target_os = "android")]
        {
            app.add_systems(Update, touch_input);
            app.insert_resource(Msaa::Off);
        }
        #[cfg(debug_assertions)]
        {
            //app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}
