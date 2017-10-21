use std::panic;

pub fn trace_init() {
	panic::set_hook(Box::new(|_| {
		println!("Custom panic hook");
	}));
}


