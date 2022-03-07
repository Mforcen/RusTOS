extern "C" {
	fn switch_to_psp();
	fn load_context();
	fn save_context();
	fn raise_svc();
}


pub fn create_task(fn_ptr: *const ()) {
	unsafe {
		/*let this_task = NUM_TASKS as usize;
		let this_stack = GLOBAL_STACKS[this_task].as_mut_ptr() as *mut u32;
		NUM_TASKS += 1;
		GLOBAL_SP[this_task] = this_stack.offset(2048 - 16); // first stack level should be created
		*this_stack.offset(2047) = 0x01000000;
		*this_stack.offset(2046) = fn_ptr as u32;*/
	}
}