
use core::convert::Infallible;
use embedded_hal::digital::v2::InputPin;

use panic_halt as _;

#[derive(PartialEq, Clone)]
pub enum ButtonState {
    Pressed,
    Released

}
pub struct Button<P: InputPin> {
    input: P,
    previous_state: ButtonState
}


impl<P> Button<P> where P: InputPin,
<P as InputPin>::Error: core::fmt::Debug{

    pub fn new (input: P) -> Self {

        let mut b = Button::<P> {
            input,
            previous_state: ButtonState::Released
        };
        b.state(); // Update previous state
        b
    }

    pub fn pressed(&mut self) -> nb::Result<ButtonState, Infallible> {
        if self.previous_state == ButtonState::Released
        && self.input.is_high().unwrap() == true{
            self.previous_state = ButtonState::Pressed;
            Ok(self.previous_state.clone())
        }
        else {
            Err(nb::Error::WouldBlock)
        }
    }

    pub fn state(&mut self) -> ButtonState {
        if self.input.is_low().unwrap() {
            self.previous_state = ButtonState::Released;
            ButtonState::Released
        } else {
            self.previous_state = ButtonState::Pressed;
            ButtonState::Pressed
        }
    }

}

