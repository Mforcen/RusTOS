use stm32f2::stm32f215;

use crate::os::RCC_CONFIG;

pub enum SysClockSrc {
	HSI,
	HSE,
	PLLCLK,
	NotAllowed
}

#[allow(dead_code)]
pub enum AHBPrescaler {
	Div1,
	Div2,
	Div4,
	Div8,
	Div16,
	Div64,
	Div128,
	Div256,
	Div512
}

#[allow(dead_code)]
pub enum CortexTimerSrc{
	Internal,
	External
}

impl AHBPrescaler {
	pub const fn get_divider(&self) -> u32 {
		match self {
			AHBPrescaler::Div1 => 1,
			AHBPrescaler::Div2 => 2,
			AHBPrescaler::Div4 => 4,
			AHBPrescaler::Div8 => 8,
			AHBPrescaler::Div16 => 16,
			AHBPrescaler::Div64 => 64,
			AHBPrescaler::Div128 => 128,
			AHBPrescaler::Div256 => 256,
			AHBPrescaler::Div512 => 512
		}
	}
}

#[allow(dead_code)]
pub enum APBPrescaler{
	Div1,
	Div2,
	Div4,
	Div8,
	Div16
}

impl APBPrescaler {
	pub const fn get_divider(&self) -> u32 {
		match self {
			APBPrescaler::Div1 => 1,
			APBPrescaler::Div2 => 2,
			APBPrescaler::Div4 => 4,
			APBPrescaler::Div8 => 8,
			APBPrescaler::Div16 => 16,
		}
	}
}

#[allow(dead_code)]
pub enum PllClockSrc {
	HSI,
	HSE(u32),
	Invalid
}

pub struct PllCfg {
	m: u32,
	n: u32,
	p: u32,
	q: u32,
	src: PllClockSrc
}

impl PllCfg {
	// TODO change p register value for the divider
	pub const fn new(m: u32, n: u32, p: u32, q: u32,src: PllClockSrc) -> PllCfg { 
		let input_freq = match src {
			PllClockSrc::HSI => 16000000,
			PllClockSrc::HSE(freq) => freq,
			_ => panic!("Invalid PllClockSrc")
		};
		
		let m_freq = input_freq / m;

		if (m < 2) | (m_freq > 2000000) |( m_freq < 1000000) {
			panic!("Invalid PLL m value");
		}

		let n_freq = m_freq * n;

		if (n < 192) | (n > 432) {
			panic!("Invalid PLL n value with value");
		}
		
		if (n_freq > 432000000) | (n_freq < 64000000){
			panic!("Invalid n frequency");
		}

		let pll_clk = n_freq / (p*2+2);

		if (p > 3) | (pll_clk > 120000000) {
			panic!("Invalid PLL p value");
		}

		PllCfg{
			m, n, p, q, src
		}
	}

	pub const fn get_out_freq(&self) -> u32 {
		let input_freq = match self.src {
			PllClockSrc::HSI => 16000000,
			PllClockSrc::HSE(freq) => freq,
			_ => panic!("Invalid PllClockSrc")
		};
		input_freq / self.m * self.n / (self.p*2+2)
	}
}

#[allow(dead_code)]
pub struct RCCConfig {
	sys_clock_src:	SysClockSrc,
	systick_src:	CortexTimerSrc,
	hse_freq:		Option<u32>,
	lse_freq:		Option<u32>,
	pll_cfg:		Option<PllCfg>,
	ahb_prescaler:	AHBPrescaler,
	apb1_prescaler:	APBPrescaler,
	apb2_prescaler:	APBPrescaler,
}

#[allow(dead_code)]
impl RCCConfig {
	pub const fn new(sys_clock_src: SysClockSrc,
		systick_src: CortexTimerSrc,
		hse_freq: Option<u32>,
		lse_freq: Option<u32>,
		pll_cfg: Option<PllCfg>,
		ahb_prescaler: AHBPrescaler,
		apb1_prescaler: APBPrescaler,
		apb2_prescaler: APBPrescaler
	) -> RCCConfig {
		if let Some(pll) = &pll_cfg {
			match &pll.src {
				PllClockSrc::HSE(_) => {
					if let None = &hse_freq {
						panic!("Cannot source hse without configuring it in RCC");
					}
				},
				_ => ()
			};
		}
		let pll_clk: Option<u32> = match &pll_cfg {
			Some(cfg) => Some(cfg.get_out_freq()),
			None => None
		};

		let sys_clk_freq = match sys_clock_src {
			SysClockSrc::HSI => 16000000,
			SysClockSrc::HSE => {
				match hse_freq {
					Some(freq) => freq,
					None => panic!("Cannot use HSE as SysCLK without crystal")
				}
			},
			SysClockSrc::PLLCLK => {
				if let Some(freq) = pll_clk {
					if freq > 120000000 {
						panic!("PLL Clock frequency should be less than 120MHz");
					}
					freq
				}
				else
				{
					panic!("PLL should be configured to be used as SysClk source")
				}
			}
			SysClockSrc::NotAllowed => panic!("SysClockSrc should be defined"),
		};

		let hclk = sys_clk_freq / ahb_prescaler.get_divider();

		let apb1_freq = hclk / apb1_prescaler.get_divider();
		
		if apb1_freq > 30000000 {
			panic!("APB1 clock frequency cannot be higher than 30MHz");
		}

		let apb2_freq = hclk / apb2_prescaler.get_divider();

		if apb2_freq > 60000000{
			panic!("APB2 clock frequency cannot be higher than 60MHz");
		}

		RCCConfig{
			sys_clock_src,
			systick_src,
			hse_freq,
			lse_freq,
			pll_cfg,
			ahb_prescaler,
			apb1_prescaler,
			apb2_prescaler
		}
	}

	pub fn get_sysclk_freq(&self) -> u32 {
		match self.sys_clock_src {
			SysClockSrc::HSI => 16000000,
			SysClockSrc::HSE => {
				match &self.hse_freq{
					Some(freq) => *freq,
					None => panic!("SysClock requires HSE")
				}
			},
			SysClockSrc::PLLCLK => {
				match &self.pll_cfg {
					Some(cfg) => cfg.get_out_freq(),
					None => panic!("SysClock requires PLL")
				}
			},
			SysClockSrc::NotAllowed => panic!("SysClockSrc should be defined"),
		}
	}

	pub fn get_hclk_freq(&self) -> u32 {
		self.get_sysclk_freq()/self.ahb_prescaler.get_divider()
	}

	pub fn get_apb1_freq(&self) -> u32 {
		self.get_sysclk_freq()/self.apb1_prescaler.get_divider()
	}

	pub fn get_apb2_freq(&self) -> u32 {
		self.get_sysclk_freq()/self.apb2_prescaler.get_divider()
	}
}

static mut RCC_CONFIG_GLOBAL: Option<&'static RCCConfig> = None;

pub struct RCC {}

#[allow(dead_code)]
impl RCC {
	pub const PTR: *const stm32f215::rcc::RegisterBlock = stm32f215::RCC::PTR;

	pub fn is_hsi_on() -> bool {
		unsafe {
			((*Self::PTR).cr.read().bits() & 0x00000001) != 0
		}
	}

	pub fn set_hsi_on(enable: bool) {
		unsafe { (*Self::PTR).cr.modify(|r, w| {
			match enable {
				true => w.bits(r.bits() | 0x01),
				false => w.bits(r.bits() & (!0x01))
			}
		})}
	}

	pub fn is_hsi_ready() -> bool {
		unsafe {
			((*Self::PTR).cr.read().bits() & 0x00000002) != 0
		}
	}

	pub fn set_hse_on(enable: bool) {
		unsafe { (*Self::PTR).cr.modify(|r, w| {
			match enable {
				true => w.bits(r.bits() | 0x00010000),
				false => w.bits(r.bits() & (!0x00010000))
			}
		})}
	}

	pub fn is_hse_on() -> bool {
		unsafe {
			((*Self::PTR).cr.read().bits() & 0x00010000) != 0
		}
	}

	pub fn is_hse_ready() -> bool {
		unsafe {
			((*Self::PTR).cr.read().bits() & 0x00020000) != 0
		}
	}

	pub fn set_pll(cfg: &PllCfg) -> bool {
		Self::set_sys_source(&SysClockSrc::HSI);
		
		loop {
			match Self::get_sys_source() {
				SysClockSrc::HSI => break,
				_ => ()
			}
		}

		let src_mask = match cfg.src {
			PllClockSrc::HSE(_) => (1 << 22),
			_ => 0
		};

		unsafe{
			(*Self::PTR).cr.modify(|r, w| { // Powering off PLL
				w.bits(r.bits() & (!(1<<24)))
			});

			(*Self::PTR).pllcfgr.write(|w| {
				w.bits((cfg.q << 24) | src_mask | (cfg.p << 16) | (cfg.n << 6) | cfg.m)
			});

			(*Self::PTR).cr.modify(|r, w| {
				w.bits(r.bits() | (1<<24))
			});

			while ((*Self::PTR).cr.read().bits() & (1<<25)) != 0 {};
		}
		
		true
	}

	pub fn get_pll_frec(_: u32) -> u32 { // input should be hse freq
		0
	}

	pub fn set_sys_source(src: &SysClockSrc) {
		let cfg = match src {
			SysClockSrc::HSI => 0,
			SysClockSrc::HSE => 1,
			SysClockSrc::PLLCLK => 2,
			_ => 0
		};
		unsafe {
			(*Self::PTR).cfgr.modify(|r,w| {
				w.bits((r.bits() & (!0x3)) | cfg)
			})
		}
	}

	pub fn get_sys_source() -> SysClockSrc {
		let cfg = unsafe{
			(*Self::PTR).cfgr.read().bits() & 0x00000003
		};

		match cfg {
			0 => SysClockSrc::HSI,
			1 => SysClockSrc::HSE,
			2 => SysClockSrc::PLLCLK,
			_ => SysClockSrc::NotAllowed
		}
	}

	pub fn get_sys_source_status() -> SysClockSrc {
		let cfg = unsafe {
			((*Self::PTR).cfgr.read().bits() & 0x0000000c) >> 2
		};

		match cfg {
			0 => SysClockSrc::HSI,
			1 => SysClockSrc::HSE,
			2 => SysClockSrc::PLLCLK,
			_ => SysClockSrc::NotAllowed
		}
	}

	pub fn set_hclk_prescaler(cfg: &AHBPrescaler) {
		let mask: u32 = match cfg {
			AHBPrescaler::Div1 => 0,
			AHBPrescaler::Div2 => 0x80,
			AHBPrescaler::Div4 => 0x90,
			AHBPrescaler::Div8 => 0xa0,
			AHBPrescaler::Div16 => 0xb0,
			AHBPrescaler::Div64 => 0xc0,
			AHBPrescaler::Div128 => 0xd0,
			AHBPrescaler::Div256 => 0xe0,
			AHBPrescaler::Div512 => 0xf0
		};

		unsafe {
			(*Self::PTR).cfgr.modify(|r, w| {
				w.bits((r.bits() & 0xffffff0f) | mask)
			});
		}
	}

	pub fn get_hclk_prescaler() -> AHBPrescaler {
		let cfg = unsafe {
			((*Self::PTR).cfgr.read().bits() & 0x000000f0) >> 4
		};

		match cfg {
			0x08 => AHBPrescaler::Div2,
			0x09 => AHBPrescaler::Div4,
			0x0a => AHBPrescaler::Div8,
			0x0b => AHBPrescaler::Div16,
			0x0c => AHBPrescaler::Div64,
			0x0d => AHBPrescaler::Div128,
			0x0e => AHBPrescaler::Div256,
			0x0f => AHBPrescaler::Div512,
			_ => AHBPrescaler::Div1
		}
	}

	pub fn set_apb1_prescaler(cfg: &APBPrescaler) {
		let mask = match cfg {
			APBPrescaler::Div1 => 0,
			APBPrescaler::Div2 => 0x04,
			APBPrescaler::Div4 => 0x05,
			APBPrescaler::Div8 => 0x06,
			APBPrescaler::Div16 => 0x07,
		} << 10;

		unsafe {
			(*Self::PTR).cfgr.modify(|r, w| {
				w.bits((r.bits() & (!(0x7 << 10))) | mask)
			})
		};
	}

	pub fn get_apb1_prescaler() -> APBPrescaler {
		let cfg: u32 = unsafe {
			((*Self::PTR).cfgr.read().bits() >> 10) & 0x07
		};

		match cfg {
			0x04 => APBPrescaler::Div2,
			0x05 => APBPrescaler::Div4,
			0x06 => APBPrescaler::Div8,
			0x07 => APBPrescaler::Div16,
			_ => APBPrescaler::Div1
		}
	}

	pub fn set_apb2_prescaler(cfg: &APBPrescaler) {
		let mask = match cfg {
			APBPrescaler::Div1 => 0,
			APBPrescaler::Div2 => 0x04,
			APBPrescaler::Div4 => 0x05,
			APBPrescaler::Div8 => 0x06,
			APBPrescaler::Div16 => 0x07,
		} << 13;

		unsafe {
			(*Self::PTR).cfgr.modify(|r, w| {
				w.bits((r.bits() & (!(0x7 << 13))) | mask)
			})
		};
	}

	pub fn get_apb2_prescaler() -> APBPrescaler {
		let cfg: u32 = unsafe {
			((*Self::PTR).cfgr.read().bits() >> 13) & 0x07
		};

		match cfg {
			0x04 => APBPrescaler::Div2,
			0x05 => APBPrescaler::Div4,
			0x06 => APBPrescaler::Div8,
			0x07 => APBPrescaler::Div16,
			_ => APBPrescaler::Div1
		}
	}

	pub fn config(cfg: &'static RCCConfig) {
		unsafe { // look for a better solution
			RCC_CONFIG_GLOBAL = Some(&cfg);
		}

		match &cfg.pll_cfg {
			Some(pll) => {
				Self::set_pll(&pll);
			},
			None => {
				Self::set_sys_source(&SysClockSrc::HSI);
				loop {
					match Self::get_sys_source() {
						SysClockSrc::HSI => break,
						_ => ()
					}
				};
			}
		};

		Self::set_hclk_prescaler(&cfg.ahb_prescaler);

		Self::set_apb1_prescaler(&cfg.apb1_prescaler);

		Self::set_apb2_prescaler(&cfg.apb2_prescaler);

		if let Some(_) = &cfg.hse_freq {
			Self::set_hse_on(true);
			while !Self::is_hse_ready() {};
		}

		Self::set_sys_source(&cfg.sys_clock_src);
	}

	pub fn get_config() -> Option<&'static RCCConfig> {
		unsafe { RCC_CONFIG_GLOBAL }
	}
}