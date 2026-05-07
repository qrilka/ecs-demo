use bevy::prelude::*;
use rand::random_range;

const COUNT: usize = 500;
const SPEED: f32 = 100.0;
const SIZE: f32 = 400.0;

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Velocity(Vec2);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (movement_system, render_system))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    for _ in 0..COUNT {
        commands.spawn((
            Position(Vec2::new(
                random_range(-SIZE..SIZE),
                random_range(-SIZE..SIZE),
            )),
            Velocity(Vec2::new(random_range(-1.0..1.0), random_range(-1.0..1.0))),
            Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::splat(4.0)),
                ..default()
            },
            Transform::default(),
        ));
    }
}

fn movement_system(time: Res<Time>, mut query: Query<(&mut Position, &Velocity)>) {
    for (mut pos, vel) in &mut query {
        pos.0 += vel.0 * SPEED * time.delta_secs();

        // screen wrapping
        if pos.0.x > SIZE {
            pos.0.x = -SIZE;
        }
        if pos.0.x < -SIZE {
            pos.0.x = SIZE;
        }
        if pos.0.y > SIZE {
            pos.0.y = -SIZE;
        }
        if pos.0.y < -SIZE {
            pos.0.y = SIZE;
        }
    }
}

fn render_system(mut query: Query<(&Position, &mut Transform)>) {
    for (pos, mut transform) in &mut query {
        transform.translation.x = pos.0.x;
        transform.translation.y = pos.0.y;
    }
}
