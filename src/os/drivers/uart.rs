use crate::os::vfs::{
	VirtualFileSystem,
	FileLike,
	FileDescriptor,
};

trait UartLike {
	fn sync_write(buf: &[u8]) -> u32;
	fn sync_read(buf: &[u8]) -> u32;
}

struct UartEndpoint<T> where
	T: UartLike,
{
	uart: T,

	baudrate: u32,
	open_flags:u32,

	read_idx: usize,
	read_buf: [u8; 128]
}

impl<T> UartEndpoint<T> where
	T: UartLike
{
	/*pub fn new(num: u32, flags: u32) -> UartEndpoint<T> {
		UartEndpoint {
			uart: UartBase::new(num).unwrap(),
			baudrate: 115200,
			open_flags: flags,
			read_idx: 0,
			read_buf: [0;128],
		}
	}*/
}

impl<T> FileLike for UartEndpoint<T> where
	T: UartLike
{
	fn close(&self) {

	}

	fn read(&self, buf: &[u8]) -> Result<usize, isize> {
		Err(-1)
	}

	fn write(&self, buf: &[u8]) -> Result<usize, isize> {
		Err(-1)
	}

	fn seek(&self, _offset: usize, _whence: usize) -> Result<(), isize> {
		Err(-1)
	}

	fn ioctl(&self, req_num: u32, req_val: u32) -> Result<(), isize> {
		Err(-1)
	}
}

struct UartFS {
	//uart: UartBase,

	baudrate: u32,
	open_flags:u32,

	read_idx: usize,
	read_buf: [u8; 128]
}

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