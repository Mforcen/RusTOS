mod scheduler;
mod allocator;
mod thread;
mod syscalls;
mod stack;

use cortex_m::peripheral;
use core::arch::asm;

#[global_allocator]
static mut ALLOCATOR: allocator::SimpleAllocator = allocator::SimpleAllocator::empty();

extern "C" {
	static mut __sheap: u32;
}

#[no_mangle]
unsafe fn SVCall() {
	let mut stack_ptr: *mut stack::HardStack = core::ptr::null_mut();
	let mut svc_number = 0u32;
	asm!(
		"tst	lr, #4",
		"ite	eq",
		"mrseq	{0}, msp",
		"mrsne	{0}, psp",
		"ldr	{1}, [{0}, #24]",
		"ldrb	{1}, [{1}, #-2]",
		inout(reg) stack_ptr,
		inout(reg) svc_number
	);
	match svc_number {
		syscalls::THREAD_YIELD => {
			scheduler::task_yield();
		},
		syscalls::THREAD_PAUSE => {

		},
		syscalls::THREAD_GETID => {

		},
		syscalls::THREAD_SPAWN => {
			let fn_ptr = (*stack_ptr).r0 as *const fn()->!;
			scheduler::create_task(fn_ptr);
		},
		syscalls::THREAD_DEL => {
			let fn_ptr = (*stack_ptr).r0 as *mut thread::Static_thread;
			scheduler::delete_task(fn_ptr);
			scheduler::scheduler();
		},
		_ => {
			
		}	
	}
}

fn create_task(_fn_ptr: *const fn()->!) {
	unsafe {asm!("svc 0x43")};
}

fn delete_task(_fn_ptr: *const fn()->!){
	unsafe {asm!("svc 0x44")};
}

fn os_loop() -> ! {
	create_task(waitloop2 as *const fn()->!);
	loop {
	}
}

fn waitloop2() -> ! {
	let mut counter = 0u32;
	loop {
		counter = counter.wrapping_add(1);
		if counter > 1000000 {
			delete_task(core::ptr::null_mut());
		}
	}
}

pub fn init_os() {
	unsafe {
		ALLOCATOR = allocator::SimpleAllocator::new((&mut __sheap) as *mut u32 as *mut u64, 65536, 16);
		scheduler::create_task(os_loop as *const fn()->!);
		
		scheduler::scheduler();
		scheduler::load_context();

		{
			let mut periph = peripheral::Peripherals::take().unwrap();
			
			periph.SYST.set_reload(0x0000ffff);

			periph.SYST.set_clock_source(peripheral::syst::SystClkSource::Core);
			periph.SYST.enable_interrupt();
			periph.SYST.enable_counter();

		}


		
		let psp = (*scheduler::THREAD_HEAD).get_stack_ptr();
		let scratch = 0x00000003usize;
		asm!(
			"msr psp, {0}",
			"msr control, {1}",
			"isb",
			in(reg) psp,
			in(reg) scratch,
		);
		os_loop();
	}
}