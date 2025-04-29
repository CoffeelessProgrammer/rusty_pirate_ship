# Bevy Fundamentals

**Explore:** [Home](/Bevy/README.md)

## Toolbox —
- world ∙∙∙∙∙∙∙ schedule ∙∙∙∙∙∙∙ runner

## main.rs
```rust
use bevy::prelude::*;

fn main() {
    App::new().run();
}
```

## Cargo.toml —
```toml
[dependencies]
bevy = "0.XX.XX"
log = { version = "*", features = ["max_level_debug", "release_max_level_warn"] }       # Runtime optimization

[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3
```

## Resources
- [Bevy ECS Intro](https://bevyengine.org/learn/quick-start/getting-started/ecs/)
