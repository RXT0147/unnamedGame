use gdnative::prelude::*;

//mod logger;
//mod extensions.rs;
mod player_controls;

fn init(handle: InitHandle) {
	//handle.add_class::<logger::Logger>();
	handle.add_class::<player_controls::Player>();
}

godot_init!(init);
