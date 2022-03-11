mod scheduler;
mod allocator;
mod thread;

use cortex_m::peripheral;
use cortex_m_rt::exception;
use core::arch::asm;

#[global_allocator]
static mut ALLOCATOR: allocator::SimpleAllocator = allocator::SimpleAllocator::empty();

extern "C" {
	static mut __sheap: u32;
}

#[exception]
unsafe fn SVCall() {
	scheduler::save_context();
	scheduler::scheduler();
	scheduler::load_context();
}

fn waitloop() -> ! {
	let mut counter = 0u32;
	loop {
		unsafe {counter = scheduler::SYSTICK_VAL};
	}
}

pub fn init_os() {
	unsafe {
		{
			let mut periph = peripheral::Peripherals::take().unwrap();
			
			periph.SYST.set_reload(0x0000ffff);

			periph.SYST.set_clock_source(peripheral::syst::SystClkSource::Core);
			periph.SYST.enable_interrupt();
			periph.SYST.enable_counter();

		}


		ALLOCATOR = allocator::SimpleAllocator::new((&mut __sheap) as *mut u32 as *mut u64, 65536, 16);
		scheduler::create_task(waitloop);
		scheduler::scheduler();
		scheduler::load_context();
		let psp = (*scheduler::THREAD_HEAD).get_stack_ptr();
		let scratch = 0x00000003usize;
		asm!(
			"msr psp, {0}",
			"msr control, {1}",
			"isb",
			in(reg) psp,
			in(reg) scratch,
		);
		waitloop();
	}
}