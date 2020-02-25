pub mod types {
    pub const MOUSE_EVENT: u8 =     0b00000001;
    pub const KEYBOARD_EVENT: u8 =  0b00000010;
    pub const SIZE_EVENT: u8 =      0b00000100;
    pub const FOCUS_EVENT: u8 =     0b00001000;
    pub const JOYSTICK_EVENT: u8 =  0b00010000;
}

use super::mouse::*;
use super::keyboard::*;
use super::joystick::*;

pub enum Event {
    MouseEvent(MouseEvent),
    KeyboardEvent(KeyboardEvent),
    SizeEvent(usize, usize),
    FocusEvent(bool),
    JoystickEvent(JoystickEvent)
}