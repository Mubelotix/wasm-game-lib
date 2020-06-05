use core::convert::TryFrom;
use crate::elog;

/// An event related to the mouse
#[derive(Debug)]
pub enum MouseEvent {
    Click(Button, u32, u32),
    DoubleClick(Button, u32, u32),
    Move(u32, u32),
    Enter(u32, u32),
    Leave(u32, u32),
    Up(Button, u32, u32),
    Down(Button, u32, u32)
}

use lazy_static::lazy_static;
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use std::sync::Mutex;

/// An enum representing a mouse button
#[derive(Debug)]
pub enum Button {
    /// Main button, usually the left button or the un-initialized state
    Main,
    /// Auxiliary button, usually the wheel button or the middle button (if present)
    Auxiliary,
    /// Secondary button, usually the right button
    Secondary,
    /// Fourth button, typically the Browser Back button
    Fourth,
    /// Fifth button, typically the Browser Forward button
    Fifth
}

impl TryFrom<i16> for Button {
    type Error = i16;

    fn try_from(number: i16) -> Result<Self, i16> {
        match number {
            0 => Ok(Button::Main),
            1 => Ok(Button::Main),
            2 => Ok(Button::Main),
            3 => Ok(Button::Main),
            4 => Ok(Button::Main),
            n => Err(n),
        }
    }
}

lazy_static! {
    static ref IS_RECORDING_MOUSE_EVENTS: AtomicBool = AtomicBool::new(false);
    static ref IS_MAIN_BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);
    static ref IS_AUXILIARY_BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);
    static ref IS_SECONDARY_BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);
    static ref IS_FOURTH_BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);
    static ref IS_FIFTH_BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);
    static ref IS_MOUSE_IN_CANVAS: AtomicBool = AtomicBool::new(true);
    static ref MOUSE_POSITION: Mutex<(u32, u32)> = Mutex::new((0,0));
}

/// Return true if your program already called [start_recording_mouse_events()](fn.start_recording_mouse_events.html) in the past.
pub fn are_mouse_events_recorded() -> bool {
    IS_RECORDING_MOUSE_EVENTS.load(Relaxed)
}

/// Return false if the mouse is outsite the tab.
/// Make sure you called [start_recording_mouse_events()](fn.start_recording_mouse_events.html) before.
pub fn is_mouse_in_canvas() -> bool {
    IS_MOUSE_IN_CANVAS.load(Relaxed)
}

/// Start recording mouse events in real time.
/// This cannot be stopped!
/// It allows you to use [these functions](index.html).
/// You may want to use [are_mouse_events_recorded()](fn.are_mouse_events_recorded.html) to check if your program already called this function in the past.
pub fn start_recording_mouse_events() {
    use wasm_bindgen::{prelude::*, JsCast};
    use web_sys::window;

    if !are_mouse_events_recorded() {
        let window = window().unwrap();

        let event = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            match event.button() {
                0 => IS_MAIN_BUTTON_PRESSED.store(true, Relaxed),
                1 => IS_AUXILIARY_BUTTON_PRESSED.store(true, Relaxed),
                2 => IS_SECONDARY_BUTTON_PRESSED.store(true, Relaxed),
                3 => IS_FOURTH_BUTTON_PRESSED.store(true, Relaxed),
                4 => IS_FIFTH_BUTTON_PRESSED.store(true, Relaxed),
                n => elog!("Unknown mouse button pressed: {}", n),
            }
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);
        window
            .add_event_listener_with_callback("mousedown", event.as_ref().unchecked_ref())
            .unwrap();
        event.forget();

        let event = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            match event.button() {
                0 => IS_MAIN_BUTTON_PRESSED.store(false, Relaxed),
                1 => IS_AUXILIARY_BUTTON_PRESSED.store(false, Relaxed),
                2 => IS_SECONDARY_BUTTON_PRESSED.store(false, Relaxed),
                3 => IS_FOURTH_BUTTON_PRESSED.store(false, Relaxed),
                4 => IS_FIFTH_BUTTON_PRESSED.store(false, Relaxed),
                n => elog!("Unknown mouse button released: {}", n),
            }
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);
        window
            .add_event_listener_with_callback("mouseup", event.as_ref().unchecked_ref())
            .unwrap();
        event.forget();

        let event = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
            IS_MOUSE_IN_CANVAS.store(true, Relaxed);
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);
        window
            .add_event_listener_with_callback("mouseenter", event.as_ref().unchecked_ref())
            .unwrap();
        event.forget();

        let event = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
            IS_MOUSE_IN_CANVAS.store(false, Relaxed);
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);
        window
            .add_event_listener_with_callback("mouseleave", event.as_ref().unchecked_ref())
            .unwrap();
        event.forget();

        let event = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            *MOUSE_POSITION.lock().unwrap() = (event.client_x() as u32, event.client_y() as u32);
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);
        window
            .add_event_listener_with_callback("mousemove", event.as_ref().unchecked_ref())
            .unwrap();
        event.forget();

        IS_RECORDING_MOUSE_EVENTS.store(true, Relaxed);
    } else {
        elog!("Your program is calling start_recording_mouse_events() multiple times! That's bad!");
    }
}

/// Return true if the button of the mouse is currently pressed.
/// Make sure you called [start_recording_mouse_events()](fn.start_recording_mouse_events.html) before.
pub fn is_pressed(button: Button) -> bool {
    match button {
        Button::Main => IS_MAIN_BUTTON_PRESSED.load(Relaxed),
        Button::Auxiliary => IS_AUXILIARY_BUTTON_PRESSED.load(Relaxed),
        Button::Secondary => IS_SECONDARY_BUTTON_PRESSED.load(Relaxed),
        Button::Fourth => IS_FOURTH_BUTTON_PRESSED.load(Relaxed),
        Button::Fifth => IS_FIFTH_BUTTON_PRESSED.load(Relaxed),
    }
}

/// Return the current position of the mouse.
/// Make sure you called [start_recording_mouse_events()](fn.start_recording_mouse_events.html) before.
pub fn get_mouse_position() -> (u32, u32) {
    *MOUSE_POSITION.lock().unwrap()
}