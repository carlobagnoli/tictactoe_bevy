use std::f32::consts::PI;

use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, sprite::MaterialMesh2dBundle, winit::WinitSettings};

const SIZE: f32 = 200.;

fn main() {
    App::new()
        .init_resource::<Game>()
        .insert_resource(Msaa { samples: 4 })
        .add_state(GameState::Playing)
        .insert_resource(WindowDescriptor {
            title: "Tic Tac Toe".to_string(),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_startup_system(setup)
        .add_system(mouse_events)
        .add_system(solution_detection_system)
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Debug, PartialEq)]
enum GameCell {
    Empty,
    X,
    O,
}

#[derive(Debug)]
enum WinningEvent {
    X,
    O,
    Draw,
    None
}

#[derive(Hash, Debug, Eq, PartialEq, Clone)]
enum GameState {
    Playing,
    GameOver,
}

#[derive(Default)]
struct TextureMap {
    circle: Handle<Image>,
    cross: Handle<Image>,
}

#[derive(Default)]
struct Game {
    board: Vec<Vec<GameCell>>,
    entities: Vec<Vec<Option<Entity>>>,
    textures: TextureMap,
    turn: usize,
}

fn setup(mut commands: Commands, mut game: ResMut<Game>, asset_server: Res<AssetServer>) {
    game.board = (0..3).map(|_| (0..3).map(|_| GameCell::Empty).collect()).collect();

    game.textures = TextureMap {
        circle: asset_server.load("circle.png"),
        cross: asset_server.load("cross.png")
    };

    // game.board[1][1] = GameCell::O;

    println!("{:?}", game.board);

    commands
        .spawn_bundle(Camera2dBundle::default());

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::Z + Vec3::X * 100.0,
                scale: Vec3::new(12., SIZE * 3., 0.),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(0.75, 0.75, 0.75),
                ..default()
            },
            ..default()
        });

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::Z + Vec3::NEG_X * 100.0,
                scale: Vec3::new(12., SIZE * 3., 0.),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(0.75, 0.75, 0.75),
                ..default()
            },
            ..default()
        });

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::Z + Vec3::Y * 100.0,
                scale: Vec3::new(SIZE * 3., 12., 0.),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(0.75, 0.75, 0.75),
                ..default()
            },
            ..default()
        });

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::Z + Vec3::NEG_Y * 100.0,
                scale: Vec3::new(SIZE * 3., 12., 0.),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(0.75, 0.75, 0.75),
                ..default()
            },
            ..default()
        });
}

fn draw_board(cmds: &mut Commands, game: &mut ResMut<Game>) {
    for entity_option in game.entities.iter().flatten() {
        if let Some(entity) = entity_option {
            cmds.entity(*entity).despawn_recursive();
        }
    }

    game.entities = (-1 as i32..2).map(|y| {
        (-1 as i32..2).map(|x| {
            match game.board[(x + 1) as usize][(y + 1) as usize] {
                GameCell::O => {
                    Some(cmds
                        .spawn_bundle(SpriteBundle {
                            texture: game.textures.circle.clone(),
                            transform: Transform::from_translation(Vec3::new(x as f32, y as f32, 0.) * SIZE),
                            ..default()
                        }).id())
                },
                GameCell::X => {
                    Some(cmds
                        .spawn_bundle(SpriteBundle {
                            texture: game.textures.cross.clone(),
                            transform: Transform::from_translation(Vec3::new(x as f32, y as f32, 0.) * SIZE),
                            ..default()
                        }).id())
                },
                GameCell::Empty => None
            }
        }).collect()

    }).collect();
}

fn mouse_events(mut cmds: Commands, mouse_button_input: Res<Input<MouseButton>>, windows: Res<Windows>, mut game: ResMut<Game>) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Some(window) = windows.get_primary() {
            let window_size = Vec2::new(window.width(), window.height());

            if let Some(cursor_pos_raw) = window.cursor_position() {
                let cursor_pos = (((cursor_pos_raw - window_size / 2.) / SIZE) + Vec2::ONE).round();

                if cursor_pos.cmpge(Vec2::ZERO).all() && cursor_pos.cmple(Vec2::ONE * 2.).all() {
                    let [x, y] = [cursor_pos.x as usize, cursor_pos.y as usize];

                    if game.board[x][y] == GameCell::Empty {
                        if game.turn % 2 == 0 {
                            game.board[x][y] = GameCell::O;
                        } else {
                            game.board[x][y] = GameCell::X;
                        }
                        game.turn += 1;

                        draw_board(&mut cmds, &mut game);
                    }
                }
            }
        }
    }
}

fn solution_detection_system(mut cmds: Commands, mut game: ResMut<Game>) {
    let solutions = [
        // Lines
        ([0, 1, 2], Transform { scale: Vec3::new(12., SIZE * 3., 1.), translation: Vec3::X * -SIZE, ..Default::default() }),
        ([3, 4, 5], Transform { scale: Vec3::new(12., SIZE * 3., 1.), ..Default::default() }),
        ([6, 7, 8], Transform { scale: Vec3::new(12., SIZE * 3., 1.), translation: Vec3::X * SIZE, ..Default::default() }),
        // Rows
        ([0, 3, 6], Transform { scale: Vec3::new(SIZE * 3., 12., 1.), translation: Vec3::Y * -SIZE, ..Default::default() }),
        ([1, 4, 7], Transform { scale: Vec3::new(SIZE * 3., 12., 1.), ..Default::default() }),
        ([2, 5, 8], Transform { scale: Vec3::new(SIZE * 3., 12., 1.), translation: Vec3::Y * SIZE, ..Default::default() }),
        // Diagonals
        ([0, 4, 8], Transform { scale: Vec3::new(12.5, SIZE * 3. * 2f32.sqrt(), 1.), rotation: Quat::from_rotation_z(-PI / 4.), ..Default::default() }),
        ([6, 4, 2], Transform { scale: Vec3::new(12.5, SIZE * 3. * 2f32.sqrt(), 1.), rotation: Quat::from_rotation_z(PI / 4.), ..Default::default() }),
    ];

    let (mut o, mut x) = (0, 0);

    let mut solution_transform: Option<Transform> = None;

    solutions
        .iter()
        .for_each(|(solution, transform)| {{
            let (solution_o, solution_x) = solution
                .iter()
                .map(|pos| (pos / 3, pos % 3))
                .fold((0, 0), |acc, (x, y)| match game.board[x][y] {
                    GameCell::O => (acc.0 + 1, acc.1),
                    GameCell::X => (acc.0, acc.1 + 1),
                    GameCell::Empty => acc,
                });

            if solution_x >= 3 || solution_o >= 3 {
                solution_transform = Some(*transform);
            }

            o = std::cmp::max(solution_o, o);
            x = std::cmp::max(solution_x, x);
        }});

    let empties = game.board.iter().flatten().filter(|cell| **cell == GameCell::Empty).count();

    let who_won = match (x, o, empties) {
        (3, 3, _) => WinningEvent::Draw,
        (3, _, _) => WinningEvent::X,
        (_, 3, _) => WinningEvent::O,
        (_, _, 0) => WinningEvent::Draw,
        _ => WinningEvent::None,
    };

    if let Some(transform) = solution_transform {
        cmds
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.75, 0.75, 0.75),
                    ..Default::default()
                },
                transform,
                ..Default::default()
            });
    }
}
