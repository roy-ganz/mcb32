//! Sends "Hello, world!" through the ITM port 0
//!
//! ITM is much faster than semihosting. Like 4 orders of magnitude or so.
//!
//! **NOTE** Cortex-M0 chips don't support ITM.
//!
//! You'll have to connect the microcontroller's SWO pin to the SWD interface. Note that some
//! development boards don't provide this option.
//!
//! You'll need [`itmdump`] to receive the message on the host plus you'll need to uncomment two
//! `monitor` commands in the `.gdbinit` file.
//!
//! [`itmdump`]: https://docs.rs/itm/0.2.1/itm/
//!
//! ---

#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m::{iprintln, Peripherals};
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let mut p = Peripherals::take().unwrap();
    let stim = &mut p.ITM.stim[0];

    iprintln!(stim, "Hello, world!");


    
  let rcc = p.RCC;
  rcc.iopenr.write( |w| w.iopben().set_bit() );
  let gpiob = p.GPIOB;
  unsafe { gpiob.moder.write( |w| w.mode3().bits( 0b01 ) ); }
  gpiob.otyper.write( |w| w.ot3().clear_bit() );
  // Restart the SysTick counter.
  syst.clear_current();
  // Main loop.
  loop {
    // Toggle the LED every SysTick tick.
    while !syst.has_wrapped() {};
    gpiob.odr.write( |w| w.od3().set_bit() );
    while !syst.has_wrapped() {};
    gpiob.odr.write( |w| w.od3().clear_bit() );
  }
    loop {}
}
