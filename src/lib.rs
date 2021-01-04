use gdnative::prelude::*;

//mod logger;
mod extensions;
mod player;

fn init(handle: InitHandle) {
	//handle.add_class::<logger::Logger>();
	handle.add_class::<player::Player>();
}

godot_init!(init);
