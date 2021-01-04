// DNR(Krey): Figure out why is it not printing on game start

use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Logger;

#[gdnative::methods]
impl Logger {
	fn new(_owner: &Node) -> Self {
		Logger
	}

	#[export]
	fn _ready(&self, _owner: &Node) {
		godot_print!("Starting game..");
	}
}