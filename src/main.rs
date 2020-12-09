#![deny(unsafe_code)]
#![no_std]
#![no_main]


use joystick::Direction;
use panic_halt as _;

use nb::block;

use cortex_m_rt::entry;
use stm32f1xx_hal::{dma::WriteDma, pac, prelude::*, timer::Timer, spi};
use stm32f1xx_hal::dma::TransferPayload;


mod joystick;
mod led_bank;
mod board;
mod board_config;
mod button;
mod spi_interface_no_dc;
mod shifter;


use board::Board;

use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::dma::TxDma;
use stm32f1xx_hal::pac::SPI3;

#[entry]
fn main() -> ! {

    /*
    let  Board{
        joystick, 
        mut leds,
        timer, ..} = Board::take();

    let mut delay = timer.start_count_down(1.hz());
    loop {
        block!(delay.wait()).unwrap();

        if block!(joystick.pressed()).unwrap() == Direction::North {
            leds.on(0).unwrap();
        }
    }
    */

       
       
        let cp = cortex_m::Peripherals::take().unwrap();
        // Get access to the device specific peripherals from the peripheral access crate
        let dp = pac::Peripherals::take().unwrap();

        let mut flash = dp.FLASH.constrain();
        let mut rcc = dp.RCC.constrain();
        let mut afio = dp.AFIO.constrain(&mut rcc.apb2);
        

        // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
        // `clocks`
        let clocks = rcc.cfgr.freeze(&mut flash.acr);
        let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);
        let spi3_sck = gpioc.pc10.into_alternate_push_pull(&mut gpioc.crh);
        let spi3_miso = gpioc.pc11.into_floating_input(&mut gpioc.crh);
        let spi3_mosi = gpioc.pc12.into_alternate_push_pull(&mut gpioc.crh);

        let spi_mode = spi::Mode {
            polarity: spi::Polarity::IdleLow,
            phase: spi::Phase::CaptureOnFirstTransition,
        };

        let  spi_glcd = spi::Spi::spi3(
            dp.SPI3,
            (spi3_sck, spi3_miso, spi3_mosi),
            &mut afio.mapr,
            spi_mode,
            3.mhz(),
            clocks,
            &mut rcc.apb1
        );

              
    // Set up the DMA device
   let dma = dp.DMA2.split(&mut rcc.ahb);
    
    // Connect the SPI device to the DMA
   let tx_dma = spi_glcd.with_tx_dma(dma.2);

  

  fn test_write<REMAP, PINS>(t: SpiTxDma<SPI3, REMAP, PINS, C2>, buffer: &'static [u8])
{
    let transfer = t.write(buffer);
    let x = transfer.wait();
}

   let buffer1: &'static mut [u8] = cortex_m::singleton!(:[u8; 100] = [0; 100]).unwrap();
   test_write(tx_dma, &buffer1);
      
    loop {}
}
