use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GreetPlugin)
        .run();
}

// ############################
// ###    PLUGIN: Greet    ###
// ############################

pub struct GreetPlugin;

impl Plugin for GreetPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(3.0, TimerMode::Repeating)));
        app.add_systems(Startup, 
            (add_people, update_people).chain()         // .chain() runs systems in order
        );
        app.add_systems(Update, greet_people);
    }
}

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Person;

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("Hello {}!", name.0);
        }
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Neil Caffrey".to_string())));
    commands.spawn((Person, Name("Mozzie".to_string())));
    commands.spawn((Person, Name("Peter Burke".to_string())));
    commands.spawn((Person, Name("Elizabeth".to_string())));
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elizabeth" {
            name.0 = "Elizabeth Burke".to_string();
            break;
        }
    }
}
