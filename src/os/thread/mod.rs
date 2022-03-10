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
pub struct Static_thread
{
	name: [u8;32],
	stack_ptr: *mut usize,
	prev_thread: *mut Static_thread,
	next_thread: *mut Static_thread,
	data: [u8; 8148]
}

impl Static_thread
{
	pub fn new() -> Static_thread {
		Static_thread{
			name: [0;32],
			stack_ptr: 0 as *mut usize,
			prev_thread: ptr::null_mut::<Static_thread>(),
			next_thread: ptr::null_mut::<Static_thread>(),
			data: [0; 8148]
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

	pub fn get_prev_thread(&self) -> *mut Static_thread {
		self.prev_thread
	}

	pub fn set_prev_thread(&mut self, pt: *mut Static_thread) {
		self.prev_thread = pt;
	}

	pub fn get_next_thread(&self) -> *mut Static_thread {
		self.next_thread
	}

	pub fn set_next_thread(&mut self, nt: *mut Static_thread) {
		self.next_thread = nt;
	}

	pub fn get_data(&mut self) -> &mut [u8;8148] {
		return &mut self.data;
	}
}