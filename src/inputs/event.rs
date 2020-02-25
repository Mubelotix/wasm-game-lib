pub mod types {
    pub const MOUSE_EVENT: u8 =     0b00000001;
    pub const KEYBOARD_EVENT: u8 =  0b00000010;
    pub const RESIZE_EVENT: u8 =    0b00000100;
    pub const FOCUS_EVENT: u8 =     0b00001000;
    pub const JOYSTICK_EVENT: u8 =  0b00010000;
}

use super::mouse::*;
use super::keyboard::*;
use super::joystick::*;

#[derive(Debug)]
pub enum Event {
    MouseEvent(MouseEvent),
    KeyboardEvent(KeyboardEvent),
    ResizeEvent(u32, u32),
    FocusEvent(bool),
    JoystickEvent(JoystickEvent)
}

use std::rc::Rc;
use web_sys;
use std::collections::VecDeque;
use std::cell::RefCell;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{window, Window as WebSysWindow};
use crate::system::log;

pub struct EventManager {
    window: WebSysWindow,
    events: Rc<RefCell<VecDeque<Event>>>
}

impl EventManager {
    pub fn new() -> Self {
        EventManager {
            window: window().unwrap(),
            events: Rc::new(RefCell::new(VecDeque::new()))
        }
    }

    pub fn start_recording_mouse_events(&mut self) {
        use crate::inputs::mouse::*;

        let events2 = Rc::clone(&self.events);
        let click = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            events2.borrow_mut().push_back(Event::MouseEvent(MouseEvent::Click(event.client_x() as u32, event.client_y() as u32)))
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);
        self.window
            .add_event_listener_with_callback("click", click.as_ref().unchecked_ref())
            .unwrap();
        click.forget();

        let events2 = Rc::clone(&self.events);
        let event = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            events2.borrow_mut().push_back(Event::MouseEvent(MouseEvent::DoubleClick(event.client_x() as u32, event.client_y() as u32)));            
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);
        self.window
            .add_event_listener_with_callback("dblclick", event.as_ref().unchecked_ref())
            .unwrap();
        event.forget();

        let events2 = Rc::clone(&self.events);
        let event = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            events2.borrow_mut().push_back(Event::MouseEvent(MouseEvent::Down(event.client_x() as u32, event.client_y() as u32)));            
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);
        self.window
            .add_event_listener_with_callback("mousedown", event.as_ref().unchecked_ref())
            .unwrap();
        event.forget();

        let events2 = Rc::clone(&self.events);
        let event = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            events2.borrow_mut().push_back(Event::MouseEvent(MouseEvent::Up(event.client_x() as u32, event.client_y() as u32)));            
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);
        self.window
            .add_event_listener_with_callback("mouseup", event.as_ref().unchecked_ref())
            .unwrap();
        event.forget();

        let events2 = Rc::clone(&self.events);
        let event = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            events2.borrow_mut().push_back(Event::MouseEvent(MouseEvent::Enter(event.client_x() as u32, event.client_y() as u32)));            
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);
        self.window
            .add_event_listener_with_callback("mouseenter", event.as_ref().unchecked_ref())
            .unwrap();
        event.forget();

        let events2 = Rc::clone(&self.events);
        let event = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            events2.borrow_mut().push_back(Event::MouseEvent(MouseEvent::Leave(event.client_x() as u32, event.client_y() as u32)));            
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);
        self.window
            .add_event_listener_with_callback("mouseleave", event.as_ref().unchecked_ref())
            .unwrap();
        event.forget();

        log("ok");
        let events2 = Rc::clone(&self.events);
        let event = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            events2.borrow_mut().push_back(Event::MouseEvent(MouseEvent::Move(event.client_x() as u32, event.client_y() as u32)));
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);
        self.window
            .add_event_listener_with_callback("mousemove", event.as_ref().unchecked_ref())
            .unwrap();
        event.forget();

        let events2 = Rc::clone(&self.events);
        let event = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            events2.borrow_mut().push_back(Event::MouseEvent(MouseEvent::RightClick(event.client_x() as u32, event.client_y() as u32)));      
            log("test");      
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);
        self.window
            .add_event_listener_with_callback("contextmenu", event.as_ref().unchecked_ref())
            .unwrap();
        event.forget();
    }

    pub fn start_recording_keyboard_events(&mut self) {
        use crate::inputs::keyboard::*;

        let events2 = Rc::clone(&self.events);
        let click = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            events2.borrow_mut().push_back(Event::KeyboardEvent(KeyboardEvent::Up(Key::from(event.key_code()))))
        }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
        self.window
            .add_event_listener_with_callback("keyup", click.as_ref().unchecked_ref())
            .unwrap();
        click.forget();

        let events2 = Rc::clone(&self.events);
        let event = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            events2.borrow_mut().push_back(Event::KeyboardEvent(KeyboardEvent::Down(Key::from(event.key_code()))));
            
        }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
        self.window
            .add_event_listener_with_callback("keydown", event.as_ref().unchecked_ref())
            .unwrap();
        event.forget();
    }

    pub fn start_recording_focus_events(&mut self) {
        let events2 = Rc::clone(&self.events);
        let event = Closure::wrap(Box::new(move || {
            events2.borrow_mut().push_back(Event::FocusEvent(true))
        }) as Box<dyn FnMut()>);
        self.window
            .add_event_listener_with_callback("focus", event.as_ref().unchecked_ref())
            .unwrap();
        event.forget();

        let events2 = Rc::clone(&self.events);
        let event = Closure::wrap(Box::new(move || {
            events2.borrow_mut().push_back(Event::FocusEvent(false));
            
        }) as Box<dyn FnMut()>);
        self.window
            .add_event_listener_with_callback("blur", event.as_ref().unchecked_ref())
            .unwrap();
        event.forget();
    }

    pub fn start_recording_size_events(&mut self) {
        let events2 = Rc::clone(&self.events);
        let event = Closure::wrap(Box::new(move || {
            let width = window().unwrap().document().unwrap().document_element().unwrap().client_width() as u32;
            let height = window().unwrap().document().unwrap().document_element().unwrap().client_height() as u32;
            events2.borrow_mut().push_back(Event::ResizeEvent(width, height));
        }) as Box<dyn FnMut()>);
        self.window
            .add_event_listener_with_callback("resize", event.as_ref().unchecked_ref())
            .unwrap();
        event.forget();
    }
}

impl Iterator for EventManager {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.events.borrow_mut().pop_front()
    }
}