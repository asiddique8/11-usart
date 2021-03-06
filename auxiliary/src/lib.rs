//! Initialization code

#![no_std]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust53964
extern crate panic_itm; // panic handler

pub use cortex_m::{asm::{bkpt, nop}, iprint, iprintln, peripheral::ITM};
pub use cortex_m_rt::entry;
use stm32f3xx_hal::{pac, prelude::*,  };

pub use stm32f3xx_hal::{pac::usart1, prelude::*, serial::Serial, time::MonoTimer,delay::Delay, prelude::_embedded_hal_blocking_delay_DelayMs};
// imports for led
pub use stm32f3xx_hal::pac::{gpiob, rcc};
use stm32f3xx_hal::pac::{GPIOE, RCC};


pub fn init() -> (&'static mut usart1::RegisterBlock, MonoTimer, ITM, Delay, &'static gpiob::RegisterBlock, &'static rcc::RegisterBlock) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // new delay and leds instance to confirm and operate
    let delay = Delay::new(cp.SYST, clocks);
    let (tx, rx) = match () {
        #[cfg(feature = "adapter")]
        () => {
            let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

            let tx = gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
            let rx = gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh);

            (tx, rx)
        }
        #[cfg(not(feature = "adapter"))]
        () => {
            let mut gpioc = dp.GPIOC.split(&mut rcc.ahb);

            let tx = gpioc.pc4.into_af7(&mut gpioc.moder, &mut gpioc.afrl);
            let rx = gpioc.pc5.into_af7(&mut gpioc.moder, &mut gpioc.afrl);

            (tx, rx)
        }
    };

    Serial::usart1(dp.USART1, (tx, rx), 9600.bps(), clocks, &mut rcc.apb2);
    // If you are having trouble sending/receiving data to/from the
    // HC-05 bluetooth module, try this configuration instead:
    // Serial::usart1(dp.USART1, (tx, rx), 9600.bps(), clocks, &mut rcc.apb2);

    unsafe {
        (
            &mut *(pac::USART1::ptr() as *mut _),
            MonoTimer::new(cp.DWT, clocks),
            cp.ITM,
            delay,
            &*GPIOE::ptr(),
            &*RCC::ptr()
        )
    }
}
