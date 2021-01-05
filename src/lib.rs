use gdnative::prelude::*;

mod player_movement;

fn init(handle: InitHandle) {
	//handle.add_class::<logger::Logger>();
	handle.add_class::<player_movement::Player>();
}

godot_init!(init);
