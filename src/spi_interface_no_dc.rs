use core::mem::take;

use stm32f1xx_hal::dma::WriteDma;
use crate::shifter::Shifter;
use display_interface::DisplayError;
use display_interface::DataFormat;
use display_interface::WriteOnlyDataCommand;



use embedded_hal::digital::v2::OutputPin;

 enum DirtyBuffer {
     Buffer1,
     Buffer2
 }

 impl DirtyBuffer {
     pub fn toggle(&mut self) {
         match self {
             Self::Buffer1 => *self = Self::Buffer2,
             Self::Buffer2 => *self = Self::Buffer1,
         }
     }
 }

pub struct SPIInterfaceNoDC<'a,  TXDMA, CS> {
    tx_dma: Option<TXDMA>,
    cs: CS,
    dirty: DirtyBuffer,
    shifter1: Shifter<'a>,
    shifter2: Shifter<'a>
}

impl<'a, TXDMA, CS> SPIInterfaceNoDC<'a, TXDMA,  CS>
where
    TXDMA: WriteDma<&'static [u8],u8>,
    CS: OutputPin 
{
    /// Create new SPI interface for communication with a display driver
    pub fn new(tx_dma: TXDMA, cs: CS, buffer1: &'static mut [u8], buffer2: &'static mut [u8] ) -> Self {
        Self { tx_dma: Some(tx_dma), 
            cs, 
            dirty: DirtyBuffer::Buffer1,
            shifter1: Shifter::new(buffer1),
            shifter2: Shifter::new(buffer2) 
        }
    }

    /// Consume the display interface and return
    /// the underlying peripherial driver and GPIO pins used by it
    pub fn release(self) -> (Option<TXDMA>,  CS) {
        (self.tx_dma,  self.cs)
    }

    fn write_buffer(&mut self, cmd: bool, byte: u8) {
            match self.dirty {
                    DirtyBuffer::Buffer1 => {
                        self.shifter1.shift_bool(cmd);
                        self.shifter1.shift_u8(byte);

                        if self.shifter1.len() == self.shifter1.capacity() {
                                                      
                            self.dirty.toggle();
                            let tx_dma = take(&mut self.tx_dma);
                            if let Some(tx_dma) = tx_dma {
                                //let buffer1: & mut [u8] = cortex_m::singleton!(:[u8; 20 * 72] = [0; 20 * 72]).unwrap();
                              //  let  transfer =tx_dma.write(b"hjkhjk");
//                                let (buffer, tx_dma) = transfer.wait();
  //                              self.tx_dma = Some(tx_dma);
                            }

                        }
                    },
                    DirtyBuffer::Buffer2 => {
                        self.shifter2.shift_bool(cmd);
                        self.shifter2.shift_u8(byte);
                    }
            }
            

    }

    fn send<'b>(&mut self, cmd: bool, data: DataFormat<'b>) -> Result<(), DisplayError> {
      
        match data {
            DataFormat::U8(bytes) =>  {
                for byte in bytes {
                    self.write_buffer(cmd, *byte);
                    }
                
                },
                _ => {}

    }
    Ok(())
    }
}

impl<'a, TXDMA, CS> WriteOnlyDataCommand for SPIInterfaceNoDC<'a, TXDMA,  CS>
where
TXDMA: WriteDma<&'static[u8],u8> + 'static,
    CS: OutputPin + 'static,
{
    fn send_commands(&mut self, cmds: DataFormat<'_>) -> Result<(), DisplayError> {
        // Assert chip select pin
        self.cs.set_low();

        // 1 = data, 0 = command
        
        // Send words over SPI
        let err = self.send(false, cmds);

        // Deassert chip select pin
        self.cs.set_high();

        err
    }

    fn send_data(&mut self, buf: DataFormat<'_>) -> Result<(), DisplayError> {
        // Assert chip select pin
        self.cs.set_low();

        // Send words over SPI
        let err = self.send(true, buf);
        // Deassert chip select pin
        self.cs.set_high();

        err
    }
}





