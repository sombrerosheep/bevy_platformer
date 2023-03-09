use bevy::{
    prelude::*,
    window::*,
};

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
        .add_system(sprite_movement)
        .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            sprite: Sprite{
                custom_size: Some(Vec2{ x: 10.0, y: 10.0}),
                color: Color::rgb(0.4, 0.7, 0.3),
                ..default()
            },
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        Direction::Up,
    ));
}

fn sprite_movement(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut sprite_position: Query<(&mut Direction, &mut Transform)>
) {
    const SPEED: f32 = 150.;
    let mut movement: Vec2 = Vec2::ZERO;
    
    for (mut dir, mut transform) in &mut sprite_position {
        if keyboard.pressed(KeyCode::W) {
            movement.y += 1.;
            *dir = Direction::Up;
        }

        if keyboard.pressed(KeyCode::A) {
            movement.x -= 1.;
            *dir = Direction::Left;
        }

        if keyboard.pressed(KeyCode::S) {
            movement.y -= 1.;
            *dir = Direction::Down;
        }

        if keyboard.pressed(KeyCode::D) {
            movement.x += 1.;
            *dir = Direction::Right;
        }

        let norm = movement.normalize_or_zero();
        let mvmt = norm * SPEED;

        transform.translation += mvmt.extend(0.) * time.delta_seconds();
    }
}
