use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(HealthPlugin)
        .run();
}

struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_entities)
            .add_system(regenerate_healths)
            .add_system(print_healths);
    }
}

#[derive(Component)]
struct Health {
    current: f32,
}

#[derive(Component)]
struct Regeneration {
    rate: f32,
}

fn spawn_entities(mut commands: Commands) {
    commands.spawn(Health { current: 100.0 });
    commands.spawn((Health { current: 50.0 }, Regeneration { rate: 1.0 }));
}

fn regenerate_healths(mut query: Query<(&mut Health, &Regeneration)>, time: Res<Time>) {
    for (mut health, regeneration) in &mut query {
        health.current += regeneration.rate * time.delta_seconds();
    }
}

fn print_healths(query: Query<&Health>) {
    println!("Healths:");
    for health in &query {
        println!("- {}", health.current);
    }
}
