use super::image::Image;
use super::canvas::Canvas;
use crate::inputs::event::{Event, EventManager};
use web_sys::{window, Window as WebSysWindow, Document};
use crate::system::log;

pub struct Window {
    document: Document,
    window: WebSysWindow,
    events: EventManager
}

impl Window {
    pub fn init() -> (Window, Canvas) {
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

        (Window {
            window,
            document,
            events: EventManager::new()
        }, canvas)
    }

    pub fn init_with_events(events: u8) -> (Window, Canvas) {
        let mouse_events    = 0b00000001 & events == 0b00000001;
        let key_events      = 0b00000010 & events == 0b00000010;
        let size_events     = 0b00000100 & events == 0b00000100;
        let focus_events    = 0b00001000 & events == 0b00001000;
        let joystick_events = 0b00010000 & events == 0b00010000;

        let (mut window, canvas) = Window::init();
        if mouse_events {
            window.events.start_recording_mouse_events();
        } else {
        }
        if key_events {
            window.events.start_recording_keyboard_events();
        }
        
        (window, canvas)
    }

    pub fn poll_events(&mut self) -> &mut EventManager {
        &mut self.events
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

