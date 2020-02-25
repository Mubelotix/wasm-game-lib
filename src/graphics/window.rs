use std::collections::vec_deque::Iter;
use super::image::Image;
use super::canvas::Canvas;
use crate::inputs::event::Event;
use web_sys::{window, Window as WebSysWindow, Document};
use std::collections::VecDeque;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys;
use std::cell::Ref;

pub struct Window {
    document: Document,
    window: WebSysWindow,
    pub canvas: Canvas,
    events: Rc<RefCell<VecDeque<Event>>>
}

impl Window {
    pub fn init() -> Window {
        let window = window().unwrap();
        let document = window.document().unwrap();

        let canvas = Canvas::new();
        document
            .body()
            .unwrap()
            .append_child(&canvas.element)
            .unwrap();
        canvas.element.set_width(document.document_element().unwrap().client_width() as u32);
        canvas.element.set_height(document.document_element().unwrap().client_height() as u32);

        Window {
            window,
            document,
            canvas,
            events: Rc::new(RefCell::new(VecDeque::new()))
        }
    }

    pub fn init_with_events(events: u8) -> Window {
        let mouse_events    = 0b00000001 & events == 0b00000001;
        let key_events      = 0b00000010 & events == 0b00000010;
        let size_events     = 0b00000100 & events == 0b00000100;
        let focus_events    = 0b00001000 & events == 0b00001000;
        let joystick_events = 0b00010000 & events == 0b00010000;

        let mut window = Window::init();
        if mouse_events {
            window.start_recording_mouse_events();
        }
        if key_events {
            window.start_recording_keyboard_events();
        }
        window
    }

    pub fn start_recording_mouse_events(&mut self) {
        use crate::inputs::mouse::*;

        let events2 = Rc::clone(&self.events);
        let click = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            events2.borrow_mut().push_back(Event::MouseEvent(MouseEvent::Click(event.screen_x() as u32, event.screen_y() as u32)))
            
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);
        self.window
            .add_event_listener_with_callback("onclick", click.as_ref().unchecked_ref())
            .unwrap();
        click.forget();

        let events2 = Rc::clone(&self.events);
        let event = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            events2.borrow_mut().push_back(Event::MouseEvent(MouseEvent::DoubleClick(event.screen_x() as u32, event.screen_y() as u32)))
            
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);
        self.window
            .add_event_listener_with_callback("ondblclick", event.as_ref().unchecked_ref())
            .unwrap();
        event.forget();

        let events2 = Rc::clone(&self.events);
        let event = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            events2.borrow_mut().push_back(Event::MouseEvent(MouseEvent::Down(event.screen_x() as u32, event.screen_y() as u32)))
            
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);
        self.window
            .add_event_listener_with_callback("onmousedown", event.as_ref().unchecked_ref())
            .unwrap();
        event.forget();

        let events2 = Rc::clone(&self.events);
        let event = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            events2.borrow_mut().push_back(Event::MouseEvent(MouseEvent::Up(event.screen_x() as u32, event.screen_y() as u32)))
            
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);
        self.window
            .add_event_listener_with_callback("onmouseup", event.as_ref().unchecked_ref())
            .unwrap();
        event.forget();

        let events2 = Rc::clone(&self.events);
        let event = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            events2.borrow_mut().push_back(Event::MouseEvent(MouseEvent::Enter(event.screen_x() as u32, event.screen_y() as u32)))
            
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);
        self.window
            .add_event_listener_with_callback("onmouseenter", event.as_ref().unchecked_ref())
            .unwrap();
        event.forget();

        let events2 = Rc::clone(&self.events);
        let event = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            events2.borrow_mut().push_back(Event::MouseEvent(MouseEvent::Leave(event.screen_x() as u32, event.screen_y() as u32)))
            
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);
        self.window
            .add_event_listener_with_callback("onmouseleave", event.as_ref().unchecked_ref())
            .unwrap();
        event.forget();

        let events2 = Rc::clone(&self.events);
        let event = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            events2.borrow_mut().push_back(Event::MouseEvent(MouseEvent::Move(event.screen_x() as u32, event.screen_y() as u32)))
            
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);
        self.window
            .add_event_listener_with_callback("onmousemove", event.as_ref().unchecked_ref())
            .unwrap();
        event.forget();

        let events2 = Rc::clone(&self.events);
        let event = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            events2.borrow_mut().push_back(Event::MouseEvent(MouseEvent::RightClick(event.screen_x() as u32, event.screen_y() as u32)))
            
        }) as Box<dyn FnMut(web_sys::MouseEvent)>);
        self.window
            .add_event_listener_with_callback("oncontextmenu", event.as_ref().unchecked_ref())
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
            .add_event_listener_with_callback("onkeyup", click.as_ref().unchecked_ref())
            .unwrap();
        click.forget();

        let events2 = Rc::clone(&self.events);
        let event = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            events2.borrow_mut().push_back(Event::KeyboardEvent(KeyboardEvent::Down(Key::from(event.key_code()))))
        }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
        self.window
            .add_event_listener_with_callback("onkeydown", event.as_ref().unchecked_ref())
            .unwrap();
        event.forget();

        let events2 = Rc::clone(&self.events);
        let event = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            events2.borrow_mut().push_back(Event::KeyboardEvent(KeyboardEvent::Press(Key::from(event.key_code()))))
        }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
        self.window
            .add_event_listener_with_callback("onkeypress", event.as_ref().unchecked_ref())
            .unwrap();
        event.forget();
    }

    pub fn get_mut_canvas(&mut self) -> &mut Canvas {
        &mut self.canvas
    }

    pub fn poll_events(&self) -> Ref<VecDeque<Event>> {
        self.events.borrow()
    }

    pub fn set_title(&mut self, title: &str) {
        self.document.set_title(title)
    }

    pub fn get_title(&self) -> String {
        self.document.title()
    }
    
    pub fn set_icon(&mut self, icon: &Image) {
        unimplemented!()
    }
    
    pub fn get_icon(&self) -> Image {
        unimplemented!()
    }
}

