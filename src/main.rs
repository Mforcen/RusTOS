#![no_std]
#![no_main]
#![feature(default_alloc_error_handler)]
#![feature(const_fn_trait_bound)]
#![feature(const_mut_refs)]

use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

extern crate alloc;


use cortex_m_rt::entry;

mod os;

#[entry]
fn main() -> ! {
	os::init_os();
	loop {
        // your code goes here
    }
}
