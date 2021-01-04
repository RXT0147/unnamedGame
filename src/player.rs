use crate::extensions::NodeExt as _;
use gdnative::api::Area2D;
use gdnative::prelude::*;

/// The player "class"
#[derive(NativeClass)]
#[inherit(Area2D)]
pub struct Player {
	#[property(default = 400.0)]
	speed: f32,

	screen_size: Vector2,
}

#[methods]
impl Player {
	// FIXME-DOCS(Krey): Unfortunately, this won't compile just yet: Rust will complain about the lack of a 'new' method and a 'NativeClassMethods' trait. This is because all scripts must also have a zero-argument constructor and a set of exported methods. To fix this, simply add two impl blocks:
	fn new(_owner: &Area2D) -> Self {
		Player {
			// FIXME-QA(Krey): Why is this float? Those are slow..
			speed: 400.0,
			screen_size: Vector2::new(0.0, 0.0),
		}
	}

	#[export]
	fn _ready(&mut self, owner: &Area2D) {
		// DNR-QA(Krey): Rust doesn't allow us to get the screen_size outside of unsafe function
		let viewport = unsafe { owner.get_viewport().unwrap().assume_safe() };
		// Get screen size
		self.screen_size = viewport.size();
		// DNR(Krey): Figure out better way to handle these
		godot_print!("Screen size has been set to '{:?}'", viewport.size());
	}

	#[export]
	fn _process(&mut self, owner: &Area2D, delta: f32) {
		let input = Input::godot_singleton();
		let mut velocity = Vector2::new(0.0, 0.0);

		#[cfg(feature = "debug-movement")]
		godot_print!("Player is at '{:?}'", owner.global_position());

		if Input::is_action_pressed(&input, "ui_right") {
			#[cfg(feature = "debug-movement")]
			godot_print!("Registered 'ui_right'");

			velocity.x += 100.0
		} else if Input::is_action_pressed(&input, "ui_left") {
			#[cfg(feature = "debug-movement")]
			godot_print!("Registered 'ui_left'");

			velocity.x -= 100.0
		} else {
			#[cfg(feature = "debug-movement")]
			godot_print!("No input registered");
		}

		let change = velocity * delta;
		let position = (owner.global_position() + change).clamp(Vector2::new(0.0, 0.0), self.screen_size);
		

		#[cfg(feature = "debug-movement")]
		godot_print!("Moving player to {:?}", position);
		owner.set_global_position(position);
	}
}
