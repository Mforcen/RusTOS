use core::ptr;
use crate::bitfield;

/*pub trait Thread {
	fn get_name(&self) -> [u8; 32];
	fn get_stack_ptr(&self) -> *mut u32;
	fn set_stack_ptr(&mut self, sptr: *mut u32);
	fn get_prev_thread(&self) -> *mut dyn Thread;
	fn set_prev_thread(&mut self, pt: *mut dyn Thread);
	fn get_next_thread(&self) -> *mut dyn Thread;
	fn set_next_thread(&mut self, nt: *mut dyn Thread);
}*/

pub union ThreadNotify {
	notify_val: u32,
	event_groups: *const u32,
}

impl ThreadNotify {
	pub fn new() -> ThreadNotify{
		ThreadNotify {
			notify_val:0
		}
	}
}

bitfield!(ThreadState, ThreadStateFlags, {
	WaitTime: 0,
	WaitNotify: 1,
	WaitEventGroup: 2,
	WaitCallback: 3,
	PendingNotify: 4
});

#[repr(C)]
pub struct Thread
{
	name: [u8;32],				//Space for thread name
	stack_ptr: *mut usize,		//Stack pointer of thread to unstack
	prev_thread: *mut Thread,	//Previous node pointer in linked list
	next_thread: *mut Thread,	//Next node pointer in linked list
	pub id: u32,					//Thread id
	pub state: ThreadState,			//Bitfield indicating some status flags
	pub tick_count: u32,			//Indicates how many ticks this thread has run
	pub wait_start: u32,			//Indicates when wait started
	pub wait_count: u32,			//Indicates until which tick does this function has to wait
	pub notify: ThreadNotify,		//Stores values for notification and event groups
	data_head: usize			//variable whose address represent the end of the stack memory
}

impl Thread
{
	pub fn new() -> Thread {
		Thread{
			name: [0;32],
			stack_ptr: 0 as *mut usize,
			prev_thread: ptr::null_mut::<Thread>(),
			next_thread: ptr::null_mut::<Thread>(),
			id: 0,
			state: ThreadState{0: 0},
			tick_count: 0,
			wait_start: 0,
			wait_count: 0,
			notify: ThreadNotify::new(),
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

	pub fn get_id(&self) -> u32 {
		return self.id;
	}
}