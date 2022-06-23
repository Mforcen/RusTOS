use core::ops::Deref;

use crate::os::vfs::{
	VirtualFileSystem,
	FileLike,
	FileDescriptor,
};
use crate::os::utils::VolatileCell;

#[repr(C)]
struct UartRegisters {
    pub sr: VolatileCell<u32>,
    pub dr: VolatileCell<u32>,
    pub brr: VolatileCell<u32>,
    pub cr1: VolatileCell<u32>,
    pub cr2: VolatileCell<u32>,
    pub cr3: VolatileCell<u32>,
}

struct UartBase {
	ptr_base: *const UartRegisters,
}

impl UartBase {
	pub const fn new(num: u32) -> Option<UartBase> {
		match num {
			1 => Some(UartBase{ptr_base: 0x4001_1000 as *const _}),
			2 => Some(UartBase{ptr_base: 0x4001_4400 as *const _}),
			3 => Some(UartBase{ptr_base: 0x4001_4800 as *const _}),
			4 => Some(UartBase{ptr_base: 0x4001_4c00 as *const _}),
			5 => Some(UartBase{ptr_base: 0x4001_5000 as *const _}),
			6 => Some(UartBase{ptr_base: 0x4001_1400 as *const _}),
			_ => None
		}
	}

	pub fn ptr(&self) -> *const UartRegisters {
		self.ptr_base
	}
}

impl Deref for UartBase {
	type Target = UartRegisters;
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		unsafe{&*self.ptr_base}
	}
}

struct UartEndpoint {
	uart: UartBase,

	baudrate: u32,
	open_flags:u32,

	write_idx: usize,
	write_buf: [u8; 128],

	read_idx: usize,
	read_buf: [u8; 128]
}

impl UartEndpoint {
	pub fn new(num: u32, flags: u32) -> UartEndpoint {
		UartEndpoint {
			uart: UartBase::new(num).unwrap(),
			baudrate: 115200,
			open_flags: flags,
			write_idx: 0,
			write_buf: [0;128],
			read_idx: 0,
			read_buf: [0;128],
		}
	}
}

impl FileLike for UartEndpoint {
	fn close(&self) {

	}

	fn read(&self, buf: &[u8], len: usize) -> Result<usize, isize> {
		Err(-1)
	}

	fn write(&self, buf: &[u8], len: usize) -> Result<usize, isize> {
		Err(-1)
	}

	fn seek(&self, _offset: usize, _whence: usize) -> Result<(), isize> {
		Err(-1)
	}

	fn ioctl(&self, req_num: u32, req_val: u32) -> Result<(), isize> {
		Err(-1)
	}
}

struct UartFS {}

impl VirtualFileSystem for UartFS {
	fn open(&self, path: &str) -> Result<FileDescriptor, i32> {
		use alloc::alloc::{alloc_zeroed, Layout};
		if path.starts_with("uart") {
			if let Some(numc) = path.chars().nth(4) {
				if let Some(num) = numc.to_digit(10) {
					if num > 0 && num <= 6 {
						let layout = Layout::new::<FileDescriptor>();

					}
				}	
			}
		}
		Err(-1)
	}
	fn stat(&self, _path: &str) -> Result<(), i32> {
		Err(-1)
	}
	fn mkdir(&self, _path: &str) -> Result<(), i32> {
		Err(-1)
	}
	fn rmdir(&self, _path: &str) -> Result<(), i32> {
		Err(-1)
	}
	fn remove(&self, _path: &str) -> Result<(), i32> {
		Err(-1)
	}
}