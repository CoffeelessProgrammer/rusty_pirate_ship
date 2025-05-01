# Bevy Fundamentals

**Explore:** [Home](/Bevy/README.md) [Plugins](/Bevy/0-Plugins.md)

## Toolbox —
- world ∙∙∙∙∙∙∙ schedule ∙∙∙∙∙∙∙ runner
- ECS Entity Component System
    - Entity (u32) - contains components
    - Component - data attached to entities
        - `#[derive(Component)]`
    - System - manipulate components/perform app operations
        - `app.add_systems()`
- Resource - for global data, e.g. Timer
    - `#[derive(Resource)]`
    - `app.insert_resource()`
- Commands - spawn/despawn entities
    - `commands.spawn((NPC, Name, Health))`
- Query - select specific entities for component modification
    - `Query<&CompToRead, With<ContainsComp2>>`
    - `Query<&mut CompToModify, Without<NotContainsComp2>>`
- **Plugin** - equiv. to module; added via `app.add_plugins()`


## main.rs  —
```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins()
        .add_systems()
        .insert_resource()
        .run();
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {...}
}
```

## Cargo.toml —
```toml
[dependencies]
bevy = "0.XX.XX"
log = { version = "*", features = ["max_level_debug", "release_max_level_warn"] }       # Runtime optimization

........................
# Belong in workspace toml

[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3
```
- `cargo run --features bevy/dynamic_linking`

## Resources  —
- [Bevy ECS Intro](https://bevyengine.org/learn/quick-start/getting-started/ecs/)
- [Next Steps | Bevy Quickstart](https://bevyengine.org/learn/quick-start/next-steps/)
