use super::utils::VolatileCell;

#[repr(C)]
pub struct HardStack{
	pub r0: VolatileCell<u32>,
	pub r1: VolatileCell<u32>,
	pub r2: VolatileCell<u32>,
	pub r3: VolatileCell<u32>,
	pub r12: VolatileCell<u32>,
	pub lr: VolatileCell<u32>,
	pub pc: VolatileCell<u32>,
	pub xpsr: VolatileCell<u32>
}