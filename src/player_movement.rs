///! This library implement handling to control the player it does that by processing the user input and changing the position of a Area2D object to which the player is assigned

// FIXME-RESEARCH(Krey): Allegedly the modern GPUs are using floats because that's how GPUs operate.. Benchmark to see if that is more efficient and compatible

// FIXME-QA(Krey): assume rust-analyzer misinterpretation of unresolved import, needs to be solved prior to merge
use gdnative::api::Area2D;
// FIXME-QA(Krey): Cherry-pick the used imports instead of using asterisk?
use gdnative::prelude::*;

/// The player structure
// FIXME-QA(Krey): Complains that 'NativeClass' macro not expanded?
#[derive(NativeClass)]
#[inherit(Area2D)]
pub struct Player {
	#[property(default = 400.0)]
	movement_speed: f32,
	screen_resolution: Vector2,
}

#[methods]
impl Player {
	/// Each implementation for godot requires a `new` method and a 'NativeClassMethod` trait as all scripts must have a zero-argument constructor and a set of exported methods
	fn new(_owner: &Area2D) -> Self {		Player {
			movement_speed: 400.0,
			screen_resolution: Vector2::new(0.0, 0.0),
		}
	}

	// DNR-NOTE(Krey): This may cause issues with serialization as it may not be processing it correctly when multiple inputs are pressed at the same time it may be to the game advantage to be more responsive and as such it should be decided prior to release (https://github.com/godot-rust/godot-rust/pull/660#issuecomment-754409177) 
	/// The '_ready' functions runs only once after the game finished it's initialization
	#[export]
	fn _ready(&mut self, owner: &Area2D) {
		let viewport = owner.get_viewport_rect();

		// Set the screen resolution
		self.screen_resolution = viewport.size.to_vector();
	}

	/// Function that runs in a loop when the game is running
	#[export]
	fn _process(&mut self, owner: &Area2D, delta: f32) {
		let input = Input::godot_singleton();
		
		// This is defining the initial velocity that also stops the player after the key-press
		let mut velocity = Vector2::new(0.0, 0.0);

		// FIXME-QA(Krey): This is implementing a long if conditional.. can't we use match that is in theory prettier and more efficient?
		// FIXME-QA(Krey): Implement 'ui_right' as variable so that they can be later implemented in UI for the user to change the keybinds
		if Input::is_action_pressed(&input, "ui_right") {
			// DNM(Krey): Has to be investigated as this value does not control movement speed
			velocity.x += self.movement_speed;

		// FIXME-QA(Krey): Implement 'ui_left' as variable so that they can be later implemented in UI for the user to change the keybinds
		} else if Input::is_action_pressed(&input, "ui_left") {
			// DNM(Krey): Has to be investigated as this value does not control movement speed
			velocity.x -= self.movement_speed;

		}

		let change = velocity * delta;
		// DNR-QA(Krey): Player can overflow on the right side
		let position = (owner.global_position() + change).clamp(Vector2::new(0.0, 0.0), self.screen_resolution);
		
		owner.set_global_position(position);
	}
}
