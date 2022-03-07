mod scheduler;
mod allocator;
mod thread;

#[global_allocator]
static mut ALLOCATOR: allocator::SimpleAllocator = allocator::SimpleAllocator::empty();

#[no_mangle]
static mut THREAD_HEAD: *mut u32 = 0 as *mut u32;

extern "C" {
	static mut __sheap: u32;
	fn init_os_asm();
	fn raise_svc();
}

pub fn init_os() {
	unsafe {
		ALLOCATOR = allocator::SimpleAllocator::new((&mut __sheap) as *mut u32 as *mut u64, 4096, 16);
		init_os_asm();
		raise_svc();
	}
}