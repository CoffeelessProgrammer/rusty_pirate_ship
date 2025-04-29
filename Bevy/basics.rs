use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (
            hello_bevy,
            (update_people, greet_people).chain())              // .chain() runs systems in order
        );
    }
}

fn hello_bevy() {
    println!("Hello Bevy!");
}

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Person;

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Neil Caffrey".to_string())));
    commands.spawn((Person, Name("Mozzie".to_string())));
    commands.spawn((Person, Name("Peter Burke".to_string())));
    commands.spawn((Person, Name("Elizabeth".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Hello {}!", name.0);
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elizabeth" {
            name.0 = "Elizabeth Burke".to_string();
            break;
        }
    }
}
