use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .add_systems(Update, handle_keys)
        .run();
}


#[derive(Component)]
enum Direction {
    Left, Right, Up, Down
}
#[derive(Component)]
struct Snake;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_color(Color::srgb(1.0, 0.5, 0.0), Vec2::new(50.0, 50.0)),
        Transform::from_xyz(0., 0., 0.),
        Direction::Right,
        Snake
    ));
}

fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Right => transform.translation.x += 150. * time.delta_secs(),
            Direction::Left => transform.translation.x -= 150. * time.delta_secs(),
            Direction::Down => transform.translation.y -= 150. * time.delta_secs(),
            Direction::Up => transform.translation.y += 150. * time.delta_secs(),
        }
        if transform.translation.x > 200. {
            *logo = Direction::Left;
        } else if transform.translation.x < -200. {
            *logo = Direction::Right;
        } else if transform.translation.y > 200. {
            *logo = Direction::Down;
        } else if transform.translation.y < -200. {
            *logo = Direction::Up;
        }

    }
}

fn handle_keys(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut snake_query: Query<&mut Direction, With<Snake>>,
){
    let mut logo = snake_query.single_mut().unwrap();

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        *logo = Direction::Left

    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        *logo = Direction::Up
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        *logo = Direction::Right

    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        *logo = Direction::Down
    }
    
}