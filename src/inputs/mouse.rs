#[derive(Debug)]
pub enum MouseEvent {
    Click(u32, u32),
    RightClick(u32, u32),
    DoubleClick(u32, u32),
    Move(u32, u32),
    Enter(u32, u32),
    Leave(u32, u32),
    Up(u32, u32),
    Down(u32, u32)
}

use lazy_static::lazy_static;
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use std::sync::Mutex;

lazy_static! {
    static ref IS_RECORDING_MOUSE_EVENTS: AtomicBool = AtomicBool::new(false);
    static ref IS_MOUSE_DOWN: AtomicBool = AtomicBool::new(false);
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

        let event = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
            IS_MOUSE_DOWN.store(true, Relaxed);        
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);
        window
            .add_event_listener_with_callback("mousedown", event.as_ref().unchecked_ref())
            .unwrap();
        event.forget();

        let event = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
            IS_MOUSE_DOWN.store(false, Relaxed);       
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
        crate::elog!("Your program is calling start_recording_mouse_events() multiple times! That's bad!");
    }
}

/// Return true if the left button of the mouse is currently pressed.
/// Make sure you called [start_recording_mouse_events()](fn.start_recording_mouse_events.html) before.
pub fn is_mouse_pressed() -> bool {
    IS_MOUSE_DOWN.load(Relaxed)
}

/// Return the current position of the mouse.
/// Make sure you called [start_recording_mouse_events()](fn.start_recording_mouse_events.html) before.
pub fn get_mouse_position() -> (u32, u32) {
    *MOUSE_POSITION.lock().unwrap()
}