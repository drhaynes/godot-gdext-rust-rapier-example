use godot::engine::{Engine, CharacterBody2D, CharacterBody2DVirtual};
use godot::prelude::*;
use godot::builtin::Vector2;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    speed: f32,
    gravity: f32,

    #[base]
    base: Base<CharacterBody2D>
}

#[godot_api]
impl CharacterBody2DVirtual for Player {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            speed: -30.0,
            gravity: 9.8,
            base
        }
    }

    fn physics_process(&mut self, delta: f64) {
        if Engine::singleton().is_editor_hint() {
            return
        }

        let velocity = Vector2::new(self.speed, self.get_velocity().y + self.gravity);
        self.set_velocity(velocity);
        self.move_and_slide();
    }
}
