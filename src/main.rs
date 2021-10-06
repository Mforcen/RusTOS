#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;

use stm32f2::stm32f215;

#[allow(non_snake_case)]
#[no_mangle]
extern "C" fn Hola() {
	return;
}


#[no_mangle]
static mut NOW_TASK: u8 = 0;
#[no_mangle]
static mut NUM_TASKS: u8 = 0;
#[no_mangle]
static mut GLOBAL_SP : [*mut u32;4] = [0 as *mut u32;4];
//#[no_mangle]
//static mut GLOBAL_NAMES : [[u8;32];4] = [[0;32];4];
#[no_mangle]
static mut GLOBAL_STACKS : [[u64;512];4] = [[0;512];4]; //8 byte aligned memory 4kb spaces

fn task_1() {
	loop {
		asm::nop();
	}
}

extern "C" {
	fn switch_to_psp();
	fn load_context();
	fn save_context();
	fn raise_svc();
	fn init_os();
}

fn create_task(fn_ptr: *const ()) {
	unsafe {
		let this_task = NUM_TASKS as usize;
		let this_stack = GLOBAL_STACKS[this_task].as_mut_ptr() as *mut u32;
		NUM_TASKS += 1;
		GLOBAL_SP[this_task] = this_stack.offset(2048 - 16); // first stack level should be created
		*this_stack.offset(2047) = 0x01000000;
		*this_stack.offset(2046) = fn_ptr as u32;
	}
}

#[entry]
fn main() -> ! {
	unsafe {
		NOW_TASK = 0;
		NUM_TASKS = 0;
	}
	
	create_task(task_1 as *const ());

	unsafe {
		init_os();
	}
    
	loop {
        // your code goes here
    }
}
