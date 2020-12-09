

use core::convert::Infallible;
use embedded_hal::digital::v2::OutputPin;




pub struct Leds<L1, L2, L3, L4, L5, L6, L7, L8> 
where L1: OutputPin, L2: OutputPin,L3: OutputPin,L4: OutputPin,
L5: OutputPin,L6: OutputPin,L7: OutputPin, L8: OutputPin
{
    l1 : L1,
    l2 :L2,
    l3 :L3,
    l4 :L4,
    l5 :L5,
    l6 :L6,
    l7 :L7,
    l8 :L8,
}



impl<L1, L2, L3, L4, L5, L6, L7, L8>  Leds<L1, L2, L3, L4, L5, L6, L7, L8> 
where L1: OutputPin, L2: OutputPin,L3: OutputPin,L4: OutputPin,
L5: OutputPin,L6: OutputPin,L7: OutputPin, L8: OutputPin,
<L1 as OutputPin>::Error: core::fmt::Debug,
<L2 as OutputPin>::Error: core::fmt::Debug,
<L3 as OutputPin>::Error: core::fmt::Debug,
<L4 as OutputPin>::Error: core::fmt::Debug,
<L5 as OutputPin>::Error: core::fmt::Debug,
<L6 as OutputPin>::Error: core::fmt::Debug,
<L7 as OutputPin>::Error: core::fmt::Debug,
<L8 as OutputPin>::Error: core::fmt::Debug,
{

    pub fn new(l1: L1, l2: L2, l3 : L3, l4 : L4, l5 : L5, l6 : L6, l7 : L7, l8 : L8) -> Self {
            Leds {
                l1,
                l2,
                l3,
                l4,
                l5,
                l6,
                l7,
                l8,
            }
    }
    pub fn set(&mut self, pattern: u8){
        if pattern & 1 == 1 {
            self.l1.set_high().unwrap();
        }else{
            self.l1.set_low().unwrap();
        }
       
        if pattern & 1 << 1 == 1 {
            self.l2.set_high().unwrap();
        } else {
            self.l2.set_low().unwrap();
        }

        if pattern & 1 << 2 == 1 {
            self.l3.set_high().unwrap();
        }  else {
            self.l3.set_low().unwrap();
        }

        if pattern & 1 << 3 == 1 {
            self.l4.set_high().unwrap();
        }else {
            self.l4.set_low().unwrap();
        }

        if pattern & 1 << 4 == 1 {
            self.l5.set_high().unwrap();
        }else {
            self.l5.set_low().unwrap();
        }
        if pattern & 1 << 5 == 1 {
            self.l6.set_high().unwrap();
        }else {
            self.l6.set_low().unwrap();
        }
        if pattern & 1 << 6 == 1 {
            self.l7.set_high().unwrap();
        }else {
            self.l7.set_low().unwrap();
        }
        if pattern & 1 << 7 == 1 {
            self.l8.set_high().unwrap();
        }else {
            self.l8.set_low().unwrap();
        }
       
    }

    pub fn all_off(&mut self) -> Result<(), Infallible>{
      self.set(0);
        Ok(())
    }
    pub fn all_on(&mut self) -> Result<(), Infallible>{
        self.set(0xff);
        Ok(())
    }

    pub fn on(&mut self, number: u8) -> Result<(), Infallible>{
       
        match number {
            0 => {self.l1.set_high().unwrap()},
            1 => {self.l2.set_high().unwrap()},
            2 => {self.l3.set_high().unwrap()},
            3 => {self.l4.set_high().unwrap()},
            4 => {self.l5.set_high().unwrap()},
            5 => {self.l6.set_high().unwrap()},
            6 => {self.l7.set_high().unwrap()},
            7 => {self.l8.set_high().unwrap()},
            _ => {}
        }
        Ok(())
    }
    pub fn off(&mut self, number: u8) -> Result<(),Infallible>{
        match number {
            0 => {self.l1.set_low().unwrap()},
            1 => {self.l2.set_low().unwrap()},
            2 => {self.l3.set_low().unwrap()},
            3 => {self.l4.set_low().unwrap()},
            4 => {self.l5.set_low().unwrap()},
            5 => {self.l6.set_low().unwrap()},
            6 => {self.l7.set_low().unwrap()},
            7 => {self.l8.set_low().unwrap()},
            _ => {}

        }
        Ok(())
    }
}
