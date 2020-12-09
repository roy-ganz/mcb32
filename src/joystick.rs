
use core::convert::Infallible;
use embedded_hal::digital::v2::InputPin;


#[derive(PartialEq)]
pub enum Clockwise {
    None,
    Quarter,
    Half,
    ThreeQuarter,
}

#[derive(PartialEq)]
pub enum Direction {
    
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
    Center
}

pub struct Joystick<C, N, E, S, W> 
where C: InputPin, N: InputPin, E:InputPin, S:InputPin, W:InputPin
{
    rotation: u8,
    previous_pattern: u8,
    center_in : C,
    north_in :N,
    east_in :E,
    south_in :S,
    west_in :W,
}


impl<C, N, E, S, W> Joystick<C, N, E, S, W> 
where C: InputPin, N: InputPin, E:InputPin, S:InputPin, W:InputPin,
<C as InputPin>::Error: core::fmt::Debug,
<N as InputPin>::Error: core::fmt::Debug,
<E as InputPin>::Error: core::fmt::Debug,
<S as InputPin>::Error: core::fmt::Debug,
<W as InputPin>::Error: core::fmt::Debug

{

    pub fn new(north_in: N, east_in :E, south_in:S, west_in:W, center_in:C) -> Self {

            Joystick {
                rotation: 0,
                previous_pattern: 0,
                center_in,
                north_in,
                east_in,
                south_in,
                west_in,
            }
    }
    pub fn rotate(&mut self, angle: Clockwise) {
         let rotation = match angle {
              Clockwise::None => 0,
              Clockwise::Quarter => 1,
              Clockwise::Half => 2,
              Clockwise::ThreeQuarter => 3,
         };
         self.rotation = rotation;
    }

    pub fn direction(&self) -> Option<Direction> {

     let pattern = self.read_pattern();     

     Self::direction_from_pattern(pattern)
    }

    pub fn released(&self) -> nb::Result<Direction, Infallible> {

     let pattern = self.read_pattern();     
    
    if pattern == 0 && pattern != self.previous_pattern  {
     if self.center_in.is_low().unwrap() {
          Self::direction_from_pattern(self.previous_pattern)
               .ok_or(nb::Error::WouldBlock) 
      } else {
          Err(nb::Error::WouldBlock)
      }
   
    } else {
     Err(nb::Error::WouldBlock)
    }
 }

    
    pub fn pressed(&self) -> nb::Result<Direction, Infallible> {

       let pattern = self.read_pattern();     
       
       if pattern != self.previous_pattern && self.previous_pattern != 0 {
          Self::direction_from_pattern(pattern).ok_or(nb::Error::WouldBlock) 
       } else {
          if self.center_in.is_high().unwrap() {
             Self::direction_from_pattern(pattern).ok_or(nb::Error::WouldBlock) 
           } else {
               Err(nb::Error::WouldBlock)
           }
       }
    }
    fn read_pattern(&self) -> u8 {
     let mut pattern :u8 = 0;

     pattern |=   if self.north_in.is_high().unwrap() {1} else {0};
     pattern |=   if self.east_in.is_high().unwrap() {2} else {0};
     pattern |=   if self.south_in.is_high().unwrap() {3} else {0};
     pattern |=   if self.west_in.is_high().unwrap() {4} else {0};

       // Rotate low nibble
       pattern <<= self.rotation; 
       let t = pattern & 0xf0;
       pattern |= t >> 4;

     pattern
    }

    fn direction_from_pattern(mut pattern:u8) -> Option<Direction> {

        if pattern | 3 == 3 { // 1 + 2
             Some(Direction::NorthEast)
        } else if pattern | 6 == 6 { // 2 + 4
             Some(Direction::SouthEast)
        }else if pattern | 12 == 12 { // 4 + 8
             Some(Direction::SouthWest)
        } else if pattern | 9 == 9 { // 8 + 1
             Some(Direction::NorthWest)
        } else if pattern | 1 == 1 {
             Some(Direction::North)
        } else if pattern | 2 == 2 {
             Some(Direction::East)
        }else if pattern | 4 == 4 {
             Some(Direction::South)
        } else if pattern | 8 == 8 {
             Some(Direction::West)
        } else {
            None
        }  
    }



}