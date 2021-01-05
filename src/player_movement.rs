///! This library implement handling to control the player it does that by processing the user input and changing the position of a Area2D object to which the player is assigned

// FIXME-RESEARCH(Krey): Allegedly 

// DNM-QA(Krey): assume rust-analyzer misinterpretation of unresolved import, needs to be solved prior to merge
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

	/// The '_ready' functions runs only once after the game finished it's initialization
	#[export]
	fn _ready(&mut self, owner: &Area2D) {
		// DNR-QA(Krey): Rust doesn't allow us to get the screen_size outside of unsafe function right?
		let viewport = unsafe { owner.get_viewport().unwrap().assume_safe() };

		// DNM-QA(Krey): This doesn't work when the screen is resized which is unexpected
		// Set the screen resolution
		self.screen_resolution = viewport.size();

		// DNR(Krey): Figure out better way to handle these
		godot_print!("Screen resolution has been set to '{:?}'", viewport.size());
	}

	/// Function that runs in a loop when the game is running
	#[export]
	fn _process(&mut self, owner: &Area2D, delta: f32) {
		// WTF(Krey)
		let input = Input::godot_singleton();
		
		// It's important to set the velocity back to 0 after the key-press has been processed as without it the player will unwantedly continue in it's movement
		let mut velocity = Vector2::new(0.0, 0.0);

		// FIXME-QA(Krey): This is implementing a long if conditional.. can't we use match that is in theory prettier and more efficient?
		// FIXME-QA(Krey): Implement 'ui_right' as variable so that they can be later implemented in UI for the user to change the keybinds
		if Input::is_action_pressed(&input, "ui_right") {
			velocity.x += self.movement_speed;

		// FIXME-QA(Krey): Implement 'ui_left' as variable so that they can be later implemented in UI for the user to change the keybinds
		} else if Input::is_action_pressed(&input, "ui_left") {
			velocity.x -= self.movement_speed;

		}

		let change = velocity * delta;
		// DNR-QA(Krey): Player can overflow on the right side
		let position = (owner.global_position() + change).clamp(Vector2::new(0.0, 0.0), self.screen_resolution);
		
		owner.set_global_position(position);
	}
}
