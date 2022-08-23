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

#[macro_export]
macro_rules! bitconfig {
	($name:ident, $offset:literal, $bits:literal, {$($id:ident : $val:literal),*}) => {
		#[allow(dead_code)]
		pub enum $name {
			$($id,)*
		}

		#[allow(dead_code)]
		impl $name {
			const fn get_clear_mask(&self) -> u32 {
				let mut mask = 0;
				let mut i = 0;
				while i < $bits {
					mask = mask << 1 | 1;
					i += 1;
				}
				!(mask << $offset)
			}

			const fn get_bit_value(&self) -> u32 {
				let mask = match *self {
					$($name::$id => $val,)*
				};
				mask << $offset
			}
		}
	};
}

#[macro_export]
macro_rules! bitregister {
	($name:ident, {$($fieldname:ident : $fieldtype:ident),*}) => {
		pub struct $name {
			$(pub $fieldname: $fieldtype,)*
		}

		impl $name {
			pub fn get_bit_value(&self) -> u32 {
				let mut val: u32 = 0;
				$(val |= self.$fieldname.get_bit_value();)*
				val
			}

			pub fn get_clear_mask(&self) -> u32 {
				let mut free:u32 = 0;
				$(free |= self.$fieldname.get_clear_mask();)*
				free
			}
		}
	};
}

#[macro_export]
macro_rules! bitfield {
	($name:ident, $fieldname:ident, {$($field:ident : $index:literal),*}) => {
		pub enum $fieldname {
			$($field),*
		}

		pub struct $name(u32);

		impl $name {
			fn get_flag(&self, f: $fieldname) -> bool {
				match f {
					$($fieldname::$field => {
						(self.0 & (1 << $index)) != 0
					}),*
				}
			}

			fn set_flag(&mut self, f: $fieldname) {
				match f {
					$($fieldname::$field => {
						self.0 |= (1 << $index)
					}),*
				}
			}

			fn clear_flag(&mut self, f: $fieldname) {
				match f {
					$($fieldname::$field => {
						self.0 &= !(1 << $index)
					}),*
				}
			}
		}
	};
}