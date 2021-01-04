///! This library implement handling to control the player it does that by processing the user input and changing the position of a Area2D object to which the player is assigned

// DNM-QA(Krey): assume rust-analyzer misinterpretation of unresolved import, needs to be solved prior to merge
use gdnative::api::Area2D;
// FIXME-QA(Krey): Cherry-pick the used imports instead of using asterisk
use gdnative::prelude::*;

/// The player structure
// FIXME-QA(Krey): Complains that 'NativeClass' macro not expanded?
#[derive(NativeClass)]
#[inherit(Area2D)]
pub struct Player {
	#[property(default = 400.0)]
	speed: f32,

	screen_size: Vector2,
}

#[methods]
impl Player {
	// FIXME-DOCS(Krey): Unfortunately, this won't compile just yet: Rust will complain about the lack of a 'new' method and a 'NativeClassMethods' trait. This is because all scripts must also have a zero-argument constructor and a set of exported methods.
	fn new(_owner: &Area2D) -> Self {
		Player {
			speed: 400.0,
			screen_size: Vector2::new(0.0, 0.0),
		}
	}

	/// Function that runs only once and that's after the game initialized
	#[export]
	fn _ready(&mut self, owner: &Area2D) {
		// DNR-QA(Krey): Rust doesn't allow us to get the screen_size outside of unsafe function right?
		let viewport = unsafe { owner.get_viewport().unwrap().assume_safe() };
		// Get screen size
		self.screen_size = viewport.size();
		// DNR(Krey): Figure out better way to handle these
		godot_print!("Screen size has been set to '{:?}'", viewport.size());
	}

	/// Function that runs in a loop when the game is running
	#[export]
	fn _process(&mut self, owner: &Area2D, delta: f32) {
		let input = Input::godot_singleton();
		// FIXME-QA(Krey): This runs multiple times a second, can't we move that outside of _process as it needs to be processed only once?
		let mut velocity = Vector2::new(0.0, 0.0);

		// FIXME-QA(Krey): This is implementing a long if conditional.. can't we use match that is in theory prettier and more efficient?
		// FIXME-QA(Krey): Implement 'ui_right' as variable so that they can be later implemented in UI for the user to change the keybinds
		if Input::is_action_pressed(&input, "ui_right") {
			// DNM-QA(Krey): Expected to expand in 'Player::speed'
			velocity.x += 400.0

		// FIXME-QA(Krey): Implement 'ui_left' as variable so that they can be later implemented in UI for the user to change the keybinds
		} else if Input::is_action_pressed(&input, "ui_left") {
			// FIXME-QA(Krey): Define this into a variable
			velocity.x -= 400.0

		}

		let change = velocity * delta;
		// DNR-QA(Krey): Player can overflow on the right side
		let position = (owner.global_position() + change).clamp(Vector2::new(0.0, 0.0), self.screen_size);
		
		owner.set_global_position(position);
	}
}
