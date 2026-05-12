use bevy::prelude::*;
use rand::random_range;

const PREDATORS: usize = 5;
const PREY: usize = 30;
const PREDATOR_SPEED: f32 = 24.0;
const PREY_SPEED: f32 = 20.0;
const PREDATOR_SIZE: f32 = 6.0;
const PREY_SIZE: f32 = 4.0;
const EAT_DISTANCE: f32 = PREDATOR_SIZE / 2.0 + PREY_SIZE / 2.0;
const SIZE: f32 = 400.0;

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Predator;

#[derive(Component)]
struct Prey;

#[derive(Message)]
struct EatPrey {
    prey: Entity,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_message::<EatPrey>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                predator_ai,
                prey_ai,
                movement_system,
                detect_eaten_prey,
                handle_eat_prey,
                render_system,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    for _ in 0..PREDATORS {
        commands.spawn(entity(Predator, Color::srgb(1.0, 0.2, 0.2), PREDATOR_SIZE));
    }
    for _ in 0..PREY {
        commands.spawn(entity(Prey, Color::srgb(0.5, 0.7, 1.0), PREY_SIZE));
    }
}

fn entity(npc_type: impl Component, color: Color, size: f32) -> impl Bundle {
    (
        npc_type,
        Position(Vec2::new(
            random_range(-SIZE..SIZE),
            random_range(-SIZE..SIZE),
        )),
        Velocity(Vec2::ZERO),
        Sprite {
            color,
            custom_size: Some(Vec2::splat(size)),
            ..default()
        },
        Transform::default(),
    )
}

fn predator_ai(
    mut predators: Query<(&Position, &mut Velocity), With<Predator>>,
    prey: Query<&Position, (With<Prey>, Without<Predator>)>,
) {
    for (pred_pos, mut pred_vel) in &mut predators {
        if let Some(target) = prey.iter().min_by(|a, b| {
            wrapped_distance_sq(pred_pos.0, a.0).total_cmp(&wrapped_distance_sq(pred_pos.0, b.0))
        }) {
            let dir = wrapped_delta(pred_pos.0, target.0).normalize_or_zero();
            pred_vel.0 = dir * PREDATOR_SPEED;
        } else {
            pred_vel.0 = Vec2::ZERO;
        }
    }
}

fn prey_ai(
    mut prey: Query<(&Position, &mut Velocity), With<Prey>>,
    predators: Query<&Position, (With<Predator>, Without<Prey>)>,
) {
    for (prey_pos, mut prey_vel) in &mut prey {
        if let Some(threat) = predators.iter().min_by(|a, b| {
            wrapped_distance_sq(prey_pos.0, a.0).total_cmp(&wrapped_distance_sq(prey_pos.0, b.0))
        }) {
            let dir = wrapped_delta(threat.0, prey_pos.0).normalize_or_zero();
            prey_vel.0 = dir * PREY_SPEED;
        } else {
            prey_vel.0 = Vec2::ZERO;
        }
    }
}

fn detect_eaten_prey(
    predators: Query<&Position, With<Predator>>,
    prey: Query<(Entity, &Position), (With<Prey>, Without<Predator>)>,
    mut eat_prey: MessageWriter<EatPrey>,
) {
    let eat_distance_squared = EAT_DISTANCE * EAT_DISTANCE;

    for pred_pos in &predators {
        for (prey_entity, prey_pos) in &prey {
            if wrapped_distance_sq(pred_pos.0, prey_pos.0) <= eat_distance_squared {
                eat_prey.write(EatPrey { prey: prey_entity });
            }
        }
    }
}

fn handle_eat_prey(mut commands: Commands, mut eat_prey: MessageReader<EatPrey>) {
    for EatPrey { prey } in eat_prey.read() {
        commands.entity(*prey).try_despawn();
    }
}

fn wrapped_distance_sq(from: Vec2, to: Vec2) -> f32 {
    wrapped_delta(from, to).length_squared()
}

fn wrapped_delta(from: Vec2, to: Vec2) -> Vec2 {
    let mut dx = to.x - from.x;
    let mut dy = to.y - from.y;
    let span = SIZE * 2.0;

    if dx > SIZE {
        dx -= span;
    } else if dx < -SIZE {
        dx += span;
    }

    if dy > SIZE {
        dy -= span;
    } else if dy < -SIZE {
        dy += span;
    }

    Vec2::new(dx, dy)
}

fn movement_system(time: Res<Time>, mut query: Query<(&mut Position, &Velocity)>) {
    for (mut pos, vel) in &mut query {
        pos.0 += vel.0 * time.delta_secs();

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
