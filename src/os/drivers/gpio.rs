use stm32f2::stm32f215;

pub trait BASE {
	fn init(&self);
	fn set_pin_mode(&self, num: u8, mode: u8);
	fn set_pin_val(&self, num: u8, level: bool);
	fn get_pin_val(&self, num: u8) -> bool;
}

impl BASE for stm32f215::gpioa::RegisterBlock {
	fn init(&self) {
		unsafe{ (*stm32f215::RCC::PTR).ahb1enr.modify(|r,w| unsafe {
			w.bits(r.bits() | 0x01)
		})}
	}

	fn set_pin_mode(&self, num: u8, mode: u8) {
		let mask = 0x3 << (num*2);
		self.moder.modify(|r, w| unsafe {
			w.bits((r.bits() & mask) | ((mode << (num*2)) as u32))
		})
	}

	fn set_pin_val(&self, num: u8, level: bool) {
		let mask = if level {
			1 << (num + 16)
		} else {
			1 << num
		};

		self.moder.write(|w| unsafe {w.bits(mask)});
	}

	fn get_pin_val(&self, num: u8) -> bool {
		false
	}

}

impl BASE for stm32f215::gpioi::RegisterBlock {
	fn init(&self) {
		unsafe { (*stm32f215::RCC::PTR).ahb1enr.modify(|r, w| unsafe {
			w.bits(r.bits() | 0x08)
		})}
	}

	fn set_pin_mode(&self, num: u8, mode: u8) {
		let mask = 0x3 << (num*2);
		self.moder.modify(|r, w| unsafe {
			w.bits((r.bits() & mask) | ((mode << (num*2)) as u32))
		})
	}

	fn set_pin_val(&self, num: u8, level: bool) {
		let mask = if level {
			1 << num
		} else {
			1 << (num+16)
		};

		self.bsrr.write(|w| unsafe {w.bits(mask)});
	}

	fn get_pin_val(&self, num: u8) -> bool {
		false
	}
}

pub struct GPIO {
	periph: *const dyn BASE
}

impl GPIO {
	pub fn new(periph: u8) -> Option<GPIO> {
		match periph{
			b'a' => {
				let gpio = GPIO { periph: stm32f215::GPIOA::PTR };
				unsafe {(*gpio.periph).init()};
				Some(gpio)
			},
			b'd' => {
				let gpio = GPIO { periph: stm32f215::GPIOD::PTR };
				unsafe {(*gpio.periph).init()};
				Some(gpio)
			}
			_ => None
		}
	}

	pub fn set_pin_mode(&self, num: u8, mode: u8) {
		unsafe{(*self.periph).set_pin_mode(num, mode)}
	}

	pub fn set_pin_val(&self, num: u8, level: bool) {
		unsafe{ (*self.periph).set_pin_val(num, level)}
	}

	pub fn get_pin_val(&self, num: u8) -> bool {
		false
	}
}