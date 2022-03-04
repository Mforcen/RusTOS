use core::alloc;
use core::slice;

const BLOCK_FREE_FLAG:u32 = 0x80000000;
const BLOCK_ADDR_MASK:u32 = 0x7fffffff;

pub struct SimpleAllocator {
	heap_location: *mut u64,
	heap_size: u32,
	heap_blocksize: u16,
	heap_blocknum: u16,
}

unsafe impl Sync for SimpleAllocator {}

#[repr(C)]
#[derive(Copy, Clone)]
struct BlockPtr {
	next: u32,
	prev: u32
}

#[repr(C)]
union BlockBody {
	data: u64,
	free: BlockPtr
}

#[repr(C)]
pub struct BlockHeader {
	header: BlockPtr,
	body: BlockBody
}

impl BlockHeader {
	pub fn is_free(&self) -> bool {
		self.header.next & BLOCK_FREE_FLAG != 0
	}

	pub fn get_num_block(size: usize, block_size: u16) -> Option<u16> { //revisar
		if size > 0xffff {
			None
		} else {
			if size < (block_size-8) as usize {
				Some(1)
			} else {
				let size = size - (block_size-8) as usize; // substract the first 8 bytes from the free structure of the block
				let bs = block_size as usize;
				let blocks = (size / bs) +  if (size % bs) > 0 {1} else {0};
				Some(1 + blocks as u16) //debería ser integer ceil
			}
		}
	}

	pub unsafe fn get_prev_free(&self) -> Option<u32> {
		if !self.is_free() {
			None
		} else {
			Some(self.body.free.prev)
		}
	}

	pub unsafe fn get_next_free(&self) -> Option<u32> {
		if !self.is_free() {
			None
		} else {
			Some(self.body.free.next)
		}
	}

	pub fn get_next(&self) -> u32 {
		self.header.next & BLOCK_ADDR_MASK
	}

	pub fn get_prev(&self) -> u32 {
		self.header.prev
	}

	pub fn get_data(&mut self) -> *mut u8 {
		let block_body = &mut self.body;
		(block_body as *mut BlockBody) as *mut u8
	}
}

impl SimpleAllocator {
	pub unsafe fn new(heap_loc: *mut u64, heap_size: u32, heap_blocksize: u16) -> SimpleAllocator {
		let trunc_heapsize: u32 = heap_size & (!8); // not really sure about this
		let block_num = (heap_size/(heap_blocksize as u32)) as u16;
		let heap_u8 = slice::from_raw_parts_mut(heap_loc as *mut u8, trunc_heapsize as usize);

		for i in 0..(trunc_heapsize as usize) {
			heap_u8[i] = 0;
		}

		let root = &mut *(heap_loc as *mut BlockHeader);
		root.header.next = 1;
		root.body.free.next = 1;
		root.body.free.prev = 1; //not used really

		let first_block = &mut *(heap_loc.offset((heap_blocksize/8) as isize) as *mut BlockHeader);
		first_block.header.next = BLOCK_FREE_FLAG | ((block_num - 1) as u32);

		let last_block = &mut *(heap_loc.offset(((block_num-1)*2) as isize) as *mut BlockHeader);
		last_block.header.prev = 1;


		SimpleAllocator {
			heap_location: heap_loc,
			heap_size: trunc_heapsize,
			heap_blocksize: heap_blocksize,
			heap_blocknum: block_num,
		}
	}

	pub unsafe fn get_block(&self, idx: u32) -> &mut BlockHeader {
		let block_header: *mut BlockHeader = self.heap_location.offset((idx*((self.heap_blocksize/8) as u32)) as isize) as *mut BlockHeader;
		&mut *block_header
	}

	unsafe fn split_block(&self, idx: u32, numblocks: u16) { //split free block
		let start_block = self.get_block(idx);
		let nb_index = idx+(numblocks as u32);
		let end_block = self.get_block(nb_index);
		
		if !start_block.is_free() {
			panic!("Start block should be free");
		}

		end_block.header.prev = idx;
		end_block.header.next = start_block.header.next;

		end_block.body.free.next = start_block.body.free.next;
		end_block.body.free.prev = end_block.header.prev;

		start_block.header.next = nb_index;
		start_block.body.free.next = start_block.header.next;

		let next_block = self.get_block(end_block.get_next());
		next_block.header.prev = nb_index;
	}

	unsafe fn unlink_free(&self, idx: u32) {
		let now_block = self.get_block(idx);
		let prev_block = self.get_block(now_block.body.free.prev);
		let next_block = self.get_block(now_block.body.free.next);

		prev_block.body.free.next = now_block.body.free.next;
		next_block.body.free.prev = now_block.body.free.prev;
	}

	unsafe fn get_root(&self) -> &mut BlockHeader {
		&mut (*(self.heap_location as *mut BlockHeader))
	}

	pub unsafe fn count_blocks(&self) -> usize {
		let mut n = 0usize;
		let mut idx = self.get_root().get_next();
		while (n < self.heap_blocknum as usize) && (idx != 0) {
			idx = self.get_block(idx).get_next();
			n = n+1;
		}
		n-1
	}
}

unsafe impl alloc::GlobalAlloc for SimpleAllocator {
	unsafe fn alloc(&self, layout: alloc::Layout)-> *mut u8 {
		let numblocks = BlockHeader::get_num_block(layout.size(), self.heap_blocksize).unwrap(); // get the required numblocks
		let root = self.get_root();
		let mut block = root.body.free.next; // load keep block

		while block != 0 {
			let next_block = self.get_block(block).get_next();
			let block_size = next_block - block;
			if  block_size > (numblocks as u32) {
				self.split_block(block, numblocks);
				self.unlink_free(block);
				self.get_block(block).header.next &= !BLOCK_FREE_FLAG; // mark as occupied
				return self.get_block(block).get_data();
			} else {
				block = self.get_block(block).get_next_free().unwrap_or(0);
			}
		}
		0 as *mut u8
	}

	unsafe fn dealloc(&self, ptr: *mut u8, layout: alloc::Layout) {
		let ptr = ptr as *mut u64;
		let block_hdr = ptr.offset(-1);
		let block_distance = block_hdr.offset_from(self.heap_location);
		let mut idx = (block_distance/((self.heap_blocksize as isize )/8)) as u32;
		let mut update_root = true;

		self.get_block(idx).header.next |= BLOCK_FREE_FLAG;

		if idx > 1 { // do not assimilate root
			let pb = self.get_block(idx).get_prev();
			if self.get_block(pb).is_free(){
				let next_block = self.get_block(self.get_block(idx).get_next()); 
				let prev_block = self.get_block(pb);
				next_block.header.prev = pb;
				prev_block.header.next = self.get_block(idx).header.next;
				idx = pb;
				update_root = false;
			}
		}

		let nb = self.get_block(idx).get_next();

		if self.get_block(nb).is_free() {
			self.unlink_free(nb);
			self.get_block(idx).header.next = self.get_block(nb).header.next | BLOCK_FREE_FLAG;
		}

		if update_root {
			self.get_block(self.get_root().body.free.next).body.free.prev = idx;
			self.get_block(idx).body.free.next = self.get_root().body.free.next;
			self.get_root().body.free.next = idx;
		}
	}
}