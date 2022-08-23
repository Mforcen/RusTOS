use core::ops::Deref;

use crate::os::drivers::{gpio, rcc::RCC};
use crate::os::utils::VolatileCell;
use crate::{bitconfig, bitregister};

#[repr(C)]
pub struct UartRegisters {
    pub sr: VolatileCell<u32>,
    pub dr: VolatileCell<u32>,
    pub brr: VolatileCell<u32>,
    pub cr1: VolatileCell<u32>,
    pub cr2: VolatileCell<u32>,
    pub cr3: VolatileCell<u32>,
}

bitconfig!(UartOversampling, 15, 1, {
	Oversampling16: 0,
	Oversampling8: 1
});

bitconfig!(UartWordLength, 12, 1, {
	Word8bits: 0,
	Word9bits: 1
});

bitconfig!(UartStopBits, 12, 2, {
	Stop1bit: 0,
	Stop05bits: 1,
	Stop2bits: 2,
	Stop15bits: 3
});

bitconfig!(UartTxEnable, 3, 1, {
	Disable: 0,
	Enable: 1
});

bitconfig!(UartRxEnable, 2, 1 ,{
	Disable: 0,
	Enable: 1
});

bitregister!(UartCR1, {
	over8: UartOversampling,
	wordlength: UartWordLength,
	txe: UartTxEnable,
	rxe: UartRxEnable
});

pub struct UartBase {
	ptr_base: *const UartRegisters,
	instance: u32
}

impl UartBase {
	pub const fn new(num: u32) -> Option<UartBase> {
		match num {
			1 => Some(UartBase{ptr_base: 0x4001_1000 as *const _, instance: 1}),
			2 => Some(UartBase{ptr_base: 0x4001_4400 as *const _, instance: 2}),
			3 => Some(UartBase{ptr_base: 0x4001_4800 as *const _, instance: 3}),
			4 => Some(UartBase{ptr_base: 0x4001_4c00 as *const _, instance: 4}),
			5 => Some(UartBase{ptr_base: 0x4001_5000 as *const _, instance: 5}),
			6 => Some(UartBase{ptr_base: 0x4001_1400 as *const _, instance: 6}),
			_ => None
		}
	}

	pub fn init(&self) {
		let gpio = gpio::GPIO::new(b'a').unwrap();
		
		gpio.set_pin_mode(9, 2);
		gpio.set_pin_mode(10, 2);

		gpio.set_pin_af(9, 7);
		gpio.set_pin_af(10, 7);

		unsafe { (*RCC::PTR).apb2enr.modify(|r, w| {
			w.bits(r.bits() | (1<<4))
		})}
	}

	pub fn set_baudrate(&self, baudrate: u32) {
		let freq_opt = match self.instance {
			1 => Some(RCC::get_config().unwrap().get_apb2_freq()),
			_ => None
		};
		if let Some(freq) = freq_opt {
			let div = freq/baudrate;
			(*self).brr.set(div); // cheap ass and improvable
		}
	}

	pub fn set_cr1(&self, cfg: UartCR1) {
		(*self).cr1.set(
			((*self).cr1.get() & cfg.get_clear_mask()) | cfg.get_bit_value()
		);
	}

	pub fn enable(&self) {
		(*self).cr1.set((*self).cr1.get() | (1<<13));
	}

	pub fn disable(&self) {
		(*self).cr1.set((*self).cr1.get() & (!(1<<13)));
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