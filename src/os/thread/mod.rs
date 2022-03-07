pub trait Thread {
	fn get_name(&self) -> [u8; 32];
	fn get_stack_ptr(&self) -> *mut u32;
	fn set_stack_ptr(&mut self, sptr: *mut u32);
	fn get_prev_thread(&self) -> *mut dyn Thread;
	fn set_prev_thread(&mut self, pt: *mut dyn Thread);
	fn get_next_thread(&self) -> *mut dyn Thread;
	fn set_next_thread(&mut self, nt: *mut dyn Thread);
}

#[repr(C)]
struct Static_thread<const SIZE: usize>
where 
	[();SIZE-40]: Sized
{
	name: [u8;32],
	stack_ptr: *mut u32,
	prev_thread: *mut dyn Thread,
	next_thread: *mut dyn Thread,
	data: [u8; SIZE-40]
}

impl<const SIZE: usize> Thread for Static_thread<SIZE> 
	where [(); SIZE-40]: Sized
{
	fn get_name(&self) -> [u8; 32] {
		self.name
	}
	
	fn get_stack_ptr(&self) -> *mut u32 {
		self.stack_ptr
	}

	fn set_stack_ptr(&mut self, sptr: *mut u32) {
		self.stack_ptr = sptr;
	}

	fn get_prev_thread(&self) -> *mut dyn Thread {
		self.prev_thread
	}

	fn set_prev_thread(&mut self, pt: *mut dyn Thread) {
		self.prev_thread = pt;
	}

	fn get_next_thread(&self) -> *mut dyn Thread {
		self.next_thread
	}

	fn set_next_thread(&mut self, nt: *mut dyn Thread) {
		self.next_thread = nt;
	}
}