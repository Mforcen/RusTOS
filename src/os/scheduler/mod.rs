use super::thread::Static_thread;
use alloc::boxed::Box;
use cortex_m_rt::exception;
use core::arch::asm;

extern "C" {
	fn switch_to_psp();
	fn raise_svc();
}

#[no_mangle]
pub static mut THREAD_HEAD: *mut Static_thread = 0 as *mut Static_thread;
pub static mut THREAD_PTR: *mut Static_thread = 0 as *mut Static_thread;
pub static mut SYSTICK_VAL: u32 = 0;

pub fn create_task(fn_ptr: fn()->!) {
	let task = Box::<Static_thread>::into_raw(Box::new(Static_thread::new()));
	if task == 0 as *mut Static_thread {
		panic!("Allocation for task failed");
	}
	unsafe {
		let stack = (*task).get_data().as_mut_ptr() as *mut usize;
		*stack.offset(2036) = 0x01000000; // Last PSR
		*stack.offset(2035) = fn_ptr as *const () as usize;
		(*task).set_stack_ptr(stack.offset(2037-16) as *mut usize);
	}

	unsafe {
		if THREAD_HEAD != 0 as *mut Static_thread {
			(*THREAD_HEAD).set_prev_thread(task);
			(*task).set_next_thread(THREAD_HEAD);
		}
		THREAD_HEAD = task;
	}
}

/// This function is meant to be called within the exception handler, since it assumes that
/// registers r0-r3, r12, lr(r13), pc(r14), and psr(r15) are in the stack.
/// It loads the PSP value in order to push the rest of the registers on top of it.
/// After that, the new value of the PSP is stored into r0 again and returned.
/// Returns the stack pointer value
pub unsafe fn save_context() {
	if THREAD_PTR == 0 as *mut Static_thread {
		panic!("Thread ptr is null");
	}
	let mut r0: *mut usize = 0 as *mut usize;
	asm!(
		"mrs	{0}, psp", //read process stack pointer address and place it in r0"
		"stmdb	{0}!, {{r4-r11}}",//write several registers at once (store multiple decrement before), due to !, r0 new address will be updated"
		inout(reg) r0
	);

	let thread = &mut *THREAD_PTR;
	thread.set_stack_ptr(r0);
}

/// This function is meant to be called within the exception handler, since it assumes that
/// registers r0-r3, r12, lr(r13), pc(r14) and psr (r15) will be loaded afterwards.
/// It receives the PSP value as a parameter
pub unsafe fn load_context() { // this function retrieves r4 to r11 (register not stored by processor)
	if THREAD_PTR == 0 as *mut Static_thread {
		panic!("Thread ptr is null");
	}
	let curr_thread = &*THREAD_PTR;
	let scratch = curr_thread.get_stack_ptr();
	asm!(
		"ldmia {0}!, {{r4-r11}}",
		"msr psp, {0}",
		in(reg) scratch
	)
}

/// This function is meant to be called in Handler mode. It loads the new thread into the
/// THREAD_PTR variable, that will be used to set the PSP value to do the context load
pub unsafe fn scheduler() {
	if THREAD_PTR == 0 as *mut Static_thread {
		THREAD_PTR = THREAD_HEAD;
	} else {
		let thread = &mut *THREAD_PTR;
		if thread.get_next_thread() != 0 as *mut Static_thread {
			THREAD_PTR = thread.get_next_thread();
		} else {
			THREAD_PTR = THREAD_HEAD;
		}
	}
}

#[exception]
unsafe fn SysTick() {
	SYSTICK_VAL += 1;
}