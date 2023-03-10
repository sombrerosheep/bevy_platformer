use bevy::{
    prelude::*,
    window::*,
};

#[derive(Component, Default)]
struct Velocity(Vec2);

#[derive(Component, Default)]
enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Default)]
struct Player{}

#[derive(Bundle, Default)]
struct PlayerBundle {
    player: Player,
    velocity: Velocity,
    direction: Direction,
    sprite: SpriteBundle
}

fn move_entity(
    time: Res<Time>,
    mut entity_position: Query<(&mut Transform, &Velocity)>
) {
    for (mut pos, vel) in &mut entity_position {
        pos.translation += vel.0.extend(0.) * time.delta_seconds();
    }
}

fn player_input(
    keyboard: Res<Input<KeyCode>>,
    mut player_data: Query<(&mut Velocity, &mut Direction, With<Player>)>
) {
    for (mut vel, mut dir, _) in &mut player_data {
        vel.0 = Vec2::ZERO;
        
        const SPEED: f32 = 150.;
    
        if keyboard.pressed(KeyCode::W) {
            vel.0.y += 1.;
            *dir = Direction::Up;
        }

        if keyboard.pressed(KeyCode::A) {
            vel.0.x -= 1.;
            *dir = Direction::Left;
        }

        if keyboard.pressed(KeyCode::S) {
            vel.0.y -= 1.;
            *dir = Direction::Down;
        }

        if keyboard.pressed(KeyCode::D) {
            vel.0.x += 1.;
            *dir = Direction::Right;
        }

        vel.0 = vel.0.normalize_or_zero() * SPEED;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Platformer!".into(),
                resolution: (640.,  480.).into(),
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(player_input)
        .add_system(move_entity)
        .run();
}



fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(PlayerBundle {
        velocity: Velocity(Vec2::ZERO),
        direction: Direction::Up,
        sprite: SpriteBundle{
            sprite: Sprite {
                custom_size: Some(Vec2{ x: 10.0, y: 10.0}),
                color: Color::rgb(0.4, 0.7, 0.3),
                ..default()
            },
            transform: Transform::from_xyz(50., 50., 0.),
            ..default()
        },
        ..default()
    });
    commands.spawn((
        SpriteBundle {
            sprite: Sprite{
                custom_size: Some(Vec2{ x: 10.0, y: 10.0}),
                color: Color::rgb(0.4, 0.9, 0.3),
                ..default()
            },
            transform: Transform::from_xyz(125., 125., 0.),
            ..default()
        },
        Direction::Up,
    ));
}

