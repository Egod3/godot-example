/// add back for mouse example
//use godot::classes::InputEventMouse;
use godot::classes::{ISprite2D, Input, InputEvent, Sprite2D};
use godot::global::Key;
use godot::prelude::*;
use std::collections::HashMap;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    speed: f64,
    _angular_speed: f64,
    keys: Vec<Key>,
    key_states: HashMap<Key, bool>,

    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("What wrath hath god wrought");

        Self {
            speed: 400.0,
            _angular_speed: std::f64::consts::PI,
            keys: vec![
                Key::LEFT,
                Key::A,
                Key::RIGHT,
                Key::D,
                Key::UP,
                Key::W,
                Key::DOWN,
                Key::S,
            ],
            key_states: HashMap::new(),
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        // GDScript code:
        //
        // rotation += angular_speed * delta
        // var velocity = Vector2.UP.rotated(rotation) * speed
        // position += velocity * delta

        // Spin the sprite like a top
        //let radians = (self.angular_speed * delta) as f32;
        //self.base_mut().rotate(radians);

        let mut current_position = self.base_mut().get_position();

        self.keys.iter().for_each(|k| {
            match self.key_states.get(k) {
                Some(true) => {}
                _ => return,
            };

            match *k {
                Key::UP => current_position.y -= 1.0 * 1000.0 * delta as f32,
                Key::W => current_position.y -= 1.0 * 1000.0 * delta as f32,
                Key::RIGHT => current_position.x += 1.0 * 1000.0 * delta as f32,
                Key::D => current_position.x += 1.0 * 1000.0 * delta as f32,
                Key::DOWN => current_position.y += 1.0 * 1000.0 * delta as f32,
                Key::S => current_position.y += 1.0 * 1000.0 * delta as f32,
                Key::LEFT => current_position.x -= 1.0 * 1000.0 * delta as f32,
                Key::A => current_position.x -= 1.0 * 1000.0 * delta as f32,
                _ => {
                    godot_print!("key not supported {:?}", k);
                }
            }
        });

        self.base_mut().set_position(current_position);

        // Enable for the entire sprite to orbit an invisible planetary object
        //let rotation = self.base().get_rotation();
        //let velocity = Vector2::UP.rotated(rotation) * self.speed as f32;
        //self.base_mut().translate(velocity * delta as f32);

        // or verbose:
        // let this = self.base_mut();
        // this.set_position(
        //     this.position() + velocity * delta as f32
        // );
    }

    fn input(&mut self, _event: Gd<InputEvent>) {
        // Handle key press events
        self.keys.iter().for_each(|k| {
            let is_key_pressed = Input::singleton().is_key_pressed(*k);
            self.key_states.insert(*k, is_key_pressed);
        });
        /*
        // handle mouse events
        match event.try_cast::<InputEventMouse>() {
            Ok(e) => {
                let mouse_position = e.get_position();
                self.base_mut().set_position(mouse_position);
            }
            Err(e) => {
                godot_print!("err: {}", e);
            }
        }
        */
    }
}

#[godot_api]
impl Player {
    #[func]
    fn increase_speed(&mut self, amount: f64) {
        self.speed += amount;
        self.signals().speed_increased().emit();
    }

    #[signal]
    fn speed_increased();
}
