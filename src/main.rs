#![no_std]
#![no_main]
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;

use stm32f2::stm32f215;

mod scheduler;
mod allocator;

static mut ALLOCATOR: Option<allocator::SimpleAllocator> = None;

extern "C" {
	static mut __sheap: u32;
}


#[entry]
fn main() -> ! {
	unsafe {
		ALLOCATOR = Some(allocator::SimpleAllocator::new((&mut __sheap) as *mut u32 as *mut u64, 4096, 16));
	}

	loop {
        // your code goes here
    }
}
