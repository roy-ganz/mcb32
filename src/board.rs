
use stm32f1xx_hal::dma::{TxDma, WriteDma};
use stm32f1xx_hal::pac::{USART1, USART2, SPI3};
use stm32f1xx_hal::serial::Serial;
use stm32f1xx_hal::spi;
use crate::board_config::BoardConfig;
use cortex_m::peripheral::SYST;
use stm32f1xx_hal::timer::Timer;
use stm32f1xx_hal::serial;
use cortex_m::singleton;

use stm32f1xx_hal::{gpio, gpio::{gpioa, gpiob,gpioc, gpiod, gpioe}, pac};

use crate::joystick::Joystick;
use crate::led_bank::leds::Leds;
use crate::button::Button;
//use crate::spi_interface_no_dc::SPIInterfaceNoDC;



// TODO wrap devices in features
pub struct Board {
    pub joystick: Joystick<
        gpiod::PD15<gpio::Input<gpio::Floating>>,
        gpiod::PD14<gpio::Input<gpio::Floating>>,
        gpiod::PD13<gpio::Input<gpio::Floating>>,
        gpiod::PD12<gpio::Input<gpio::Floating>>,
        gpiod::PD11<gpio::Input<gpio::Floating>>,
    >,

    pub leds: Leds<
        gpioe::PE8<gpio::Output<gpio::PushPull>>,
        gpioe::PE9<gpio::Output<gpio::PushPull>>,
        gpioe::PE10<gpio::Output<gpio::PushPull>>,
        gpioe::PE11<gpio::Output<gpio::PushPull>>,
        gpioe::PE12<gpio::Output<gpio::PushPull>>,
        gpioe::PE13<gpio::Output<gpio::PushPull>>,
        gpioe::PE14<gpio::Output<gpio::PushPull>>,
        gpioe::PE15<gpio::Output<gpio::PushPull>>,
    >,

    pub timer: Timer<SYST>,
    pub wakeup_button: Button< gpioa::PA0<gpio::Input<gpio::Floating>>>,
    pub tamper_button: Button< gpioc::PC13<gpio::Input<gpio::Floating>>>,
  
    pub uart1: Serial<USART1, (
            stm32f1xx_hal::gpio::gpiob::PB6<stm32f1xx_hal::gpio::Alternate<stm32f1xx_hal::gpio::PushPull>>,
            stm32f1xx_hal::gpio::gpiob::PB7<stm32f1xx_hal::gpio::Input<stm32f1xx_hal::gpio::Floating>>
        )>,

    pub uart2: Serial<USART2, (
                stm32f1xx_hal::gpio::gpiod::PD5<stm32f1xx_hal::gpio::Alternate<stm32f1xx_hal::gpio::PushPull>>,
                stm32f1xx_hal::gpio::gpiod::PD6<stm32f1xx_hal::gpio::Input<stm32f1xx_hal::gpio::Floating>>
            )>

    // display

    // Analog in

    // uart1
    // uart2

    // pwm 1 + 2
}

impl Board {
    pub fn take() -> Self {
        Self::take_with_config(BoardConfig::default())
    }
    pub fn take_with_config(config: BoardConfig) -> Self {
        
        use stm32f1xx_hal::prelude::*;
        use stm32f1xx_hal::dma::Half;

        let cp = cortex_m::Peripherals::take().unwrap();
        // Get access to the device specific peripherals from the peripheral access crate
        let dp = pac::Peripherals::take().unwrap();

        let mut flash = dp.FLASH.constrain();
        let mut rcc = dp.RCC.constrain();
        let mut afio = dp.AFIO.constrain(&mut rcc.apb2);
        

        // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
        // `clocks`
        let clocks = rcc.cfgr.freeze(&mut flash.acr);

        let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
        let wakeup_in = gpioa.pa0.into_floating_input(&mut gpioa.crl);
        let wakeup_button = Button::new(wakeup_in);

        let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
        let uart1_tx = gpiob.pb6.into_alternate_push_pull(&mut gpiob.crl);
        let uart1_rx = gpiob.pb7.into_floating_input(&mut gpiob.crl);

       
        let uart1 = serial::Serial::usart1(
            dp.USART1, (uart1_tx, uart1_rx), &mut afio.mapr,
            config.uart1, clocks,  &mut rcc.apb2);
        
        let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);
        let tamper_in = gpioc.pc13.into_floating_input(&mut gpioc.crh);
        let tamper_button = Button::new(tamper_in);

        let glcd_nss = gpioc.pc8.into_push_pull_output(&mut gpioc.crh);

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

   /*  //fn test_write<T: WriteDma<&'static[u8],u8>>(t: T) {
    fn test_write<W: WriteDma<&'static[u8], u8>>(t: W)
    
    {
        let transfer = t.write(b"hkj");
        let x = transfer.wait();

    }

    let buffer1: &'static mut [u8] = cortex_m::singleton!(:[u8; 20 * 72] = [0; 20 * 72]).unwrap();
    test_write(tx_dma); */
   // let buffer2: &'static mut [u8] = cortex_m::singleton!(:[u8; 20 * 72] = [0; 20 * 72]).unwrap();
 //  let transfer = tx_dma.write(buffer1);
  //      let x = transfer.wait();
   // let transfer = tx_dma.write(b"gjhgjh");

   
    

    //let glcd_interface = super::spi_interface_no_dc::SPIInterfaceNoDC::new(tx_dma, glcd_nss, buffer1, buffer2);

    //let transfer = tx_dma.write(b"hello, world");

    // Wait for it to finnish. The transfer takes ownership over the SPI device
    // and the data being sent anb those things are returned by transfer.wait
   // let (_buffer, _spi_dma) = transfer.wait();

        let mut gpiod = dp.GPIOD.split(&mut rcc.apb2);
        let center_in = gpiod.pd15.into_floating_input(&mut gpiod.crh);
        let north_in = gpiod.pd14.into_floating_input(&mut gpiod.crh);
        let east_in = gpiod.pd13.into_floating_input(&mut gpiod.crh);
        let south_in = gpiod.pd12.into_floating_input(&mut gpiod.crh);
        let west_in = gpiod.pd11.into_floating_input(&mut gpiod.crh);
        let joystick = Joystick::new(north_in, east_in, south_in, west_in, center_in);

        let uart2_tx = gpiod.pd5.into_alternate_push_pull(&mut gpiod.crl);
        let uart2_rx = gpiod.pd6.into_floating_input(&mut gpiod.crl);

       
      
        let uart2 = serial::Serial::usart2(
            dp.USART2, (uart2_tx, uart2_rx), &mut afio.mapr,
            config.uart2, clocks,  &mut rcc.apb1); 
        

        let mut gpioe = dp.GPIOE.split(&mut rcc.apb2);
        let l1 = gpioe.pe8.into_push_pull_output(&mut gpioe.crh);
        let l2 = gpioe.pe9.into_push_pull_output(&mut gpioe.crh);
        let l3 = gpioe.pe10.into_push_pull_output(&mut gpioe.crh);
        let l4 = gpioe.pe11.into_push_pull_output(&mut gpioe.crh);
        let l5 = gpioe.pe12.into_push_pull_output(&mut gpioe.crh);
        let l6 = gpioe.pe13.into_push_pull_output(&mut gpioe.crh);
        let l7 = gpioe.pe14.into_push_pull_output(&mut gpioe.crh);
        let l8 = gpioe.pe15.into_push_pull_output(&mut gpioe.crh);
        let leds = Leds::new(l1, l2, l3, l4, l5, l6, l7, l8);


        let timer = Timer::syst(cp.SYST, &clocks);


        // For graphics we need, display interface and  display driver
        /*let display_spi = */
        //let display_iface = SPIInterface::new()
        /* let display_ili = */

      

        

        Board {
            joystick,
            leds,
            timer,
            wakeup_button,
            tamper_button,
            uart1,
            uart2
        }
    }
}

