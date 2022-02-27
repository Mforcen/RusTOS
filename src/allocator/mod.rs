use core::alloc;
use core::slice;

const BLOCK_FREE_FLAG:u16 = 0x8000;

pub struct SimpleAllocator {
	heap_location: *const u32,
	heap_size: u32,
	heap_blocksize: u16,
	heap_blocknum: u16,
}

unsafe impl Sync for SimpleAllocator {}

#[repr(C)]
#[derive(Copy, Clone)]
struct BlockPtr {
	next: u16,
	prev: u16
}

#[repr(C)]
union BlockBody {
	data: u32,
	free: BlockPtr
}

#[repr(C)]
struct BlockHeader {
	header: BlockPtr,
	body: BlockBody
}

impl BlockHeader {
	pub fn is_free(&self) -> bool {
		self.header.next & BLOCK_FREE_FLAG == 0
	}

	pub fn get_num_block(size: usize, block_size: u16) -> Option<u16> {
		if size > 512*1024 {
			None
		} else {
			if size < (block_size-4) as usize {
				Some(1)
			} else {
				let size = size - (block_size-4) as usize;
				Some(1+(size as u16)/(block_size))
			}
		}
	}

	pub unsafe fn get_prev_free(&self) -> Option<u16> {
		if !self.is_free() {
			None
		} else {
			Some(self.body.free.prev)
		}
	}

	pub unsafe fn get_next_free(&self) -> Option<u16> {
		if !self.is_free() {
			None
		} else {
			Some(self.body.free.next)
		}
	}

	pub fn get_next(&self) -> u16 {
		self.header.next & !BLOCK_FREE_FLAG
	}

	pub fn get_prev(&self) -> u16 {
		self.header.prev
	}
}

impl SimpleAllocator {
	pub unsafe fn new(heap_loc: *const u32, heap_size: u32, heap_blocksize: u16) -> SimpleAllocator {
		let trunc_heapsize: u32 = heap_size & (!(heap_blocksize as u32)); // not really sure about this
		let block_num = (heap_size/(heap_blocksize as u32)) as u16;
		let heap_u8 = slice::from_raw_parts_mut(heap_loc as *mut u8, trunc_heapsize as usize);

		for i in 0..(trunc_heapsize as usize) {
			heap_u8[i] = 0;
		}

		let first_block = &mut *(heap_loc as *mut BlockHeader);
		first_block.header.next = 1;
		first_block.body.free.next = 1;
		first_block.body.free.prev = 1;

		let second_block = &mut *(heap_loc.offset(heap_blocksize as isize) as *mut BlockHeader);
		second_block.header.next = BLOCK_FREE_FLAG | block_num;

		SimpleAllocator {
			heap_location: heap_loc,
			heap_size: trunc_heapsize,
			heap_blocksize: heap_blocksize,
			heap_blocknum: block_num,
		}
	}

	unsafe fn get_block(&self, idx: u16) -> &mut BlockHeader {
		let block_header: *mut BlockHeader = self.heap_location.offset((idx*self.heap_blocksize) as isize) as *mut BlockHeader;
		&mut *block_header
	}

	unsafe fn split_block(&self, idx: u16, numblocks: u16) {
		let start_block = self.get_block(idx);
		let end_block = self.get_block(idx+numblocks);
		
		end_block.header.prev = idx;
		end_block.header.next = start_block.header.next;

		end_block.body.free.next = end_block.header.next;
		end_block.body.free.prev = end_block.header.prev;

		start_block.header.next = idx+numblocks;
		start_block.body.free.next = start_block.header.next;
		start_block.body.free.prev = start_block.header.prev;
	}

	unsafe fn 
}

unsafe impl alloc::GlobalAlloc for SimpleAllocator {
	unsafe fn alloc(&self, layout: alloc::Layout)-> *mut u8 {
		let numblocks = BlockHeader::get_num_block(layout.size(), self.heap_blocksize).unwrap(); // get the required numblocks

		let mut block_opt = (*(self.heap_location as *mut BlockHeader)).get_next_free(); // load keep block

		while let Some(block) = block_opt {
			let next_block = self.get_block(block).get_next();
			if next_block - block > numblocks {

			} else {
				block_opt = self.get_block(block).get_next_free();
			}
		}
		0 as *mut u8
	}

	unsafe fn dealloc(&self, ptr: *mut u8, layout: alloc::Layout) {

	}
}