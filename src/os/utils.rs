use core::cell::UnsafeCell;
use core::ptr;

#[repr(transparent)]
pub struct VolatileCell<T>{
	value: UnsafeCell<T>
}

impl<T> VolatileCell<T>{
	pub const fn new(value: T) -> Self {
		VolatileCell{ value: UnsafeCell::new(value)}
	}

	#[inline(always)]
	pub fn get(&self) -> T
		where T: Copy
	{
		unsafe {
			ptr::read_volatile(self.value.get())
		}
	}

	#[inline(always)]
	pub fn set(&self, new: T) 
		where T: Copy
	{
		unsafe {
			ptr::write_volatile(self.value.get(), new)
		}
	}

	#[inline(always)]
	pub fn as_ptr(&self) -> *mut T {
		self.value.get()
	}
}