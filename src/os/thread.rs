use core::ptr;

/*pub trait Thread {
	fn get_name(&self) -> [u8; 32];
	fn get_stack_ptr(&self) -> *mut u32;
	fn set_stack_ptr(&mut self, sptr: *mut u32);
	fn get_prev_thread(&self) -> *mut dyn Thread;
	fn set_prev_thread(&mut self, pt: *mut dyn Thread);
	fn get_next_thread(&self) -> *mut dyn Thread;
	fn set_next_thread(&mut self, nt: *mut dyn Thread);
}*/

#[repr(C)]
pub struct Thread
{
	name: [u8;32],
	stack_ptr: *mut usize,
	prev_thread: *mut Thread,
	next_thread: *mut Thread,
	data_head: usize
}

impl Thread
{
	pub fn new() -> Thread {
		Thread{
			name: [0;32],
			stack_ptr: 0 as *mut usize,
			prev_thread: ptr::null_mut::<Thread>(),
			next_thread: ptr::null_mut::<Thread>(),
			data_head: 0
		}
	}

	pub fn get_name(&self) -> [u8; 32] {
		self.name
	}
	
	pub fn get_stack_ptr(&self) -> *mut usize {
		self.stack_ptr
	}

	pub fn set_stack_ptr(&mut self, sptr: *mut usize) {
		self.stack_ptr = sptr;
	}

	pub fn get_prev_thread(&self) -> *mut Thread {
		self.prev_thread
	}

	pub fn set_prev_thread(&mut self, pt: *mut Thread) {
		self.prev_thread = pt;
	}

	pub fn get_next_thread(&self) -> *mut Thread {
		self.next_thread
	}

	pub fn set_next_thread(&mut self, nt: *mut Thread) {
		self.next_thread = nt;
	}

	pub fn get_data(&mut self) -> *mut usize {
		return &mut self.data_head;
	}
}