use super::image::Image;
use super::canvas::Canvas;
use crate::inputs::event::EventManager;
use web_sys::{window, Window as WebSysWindow, Document};

/// A struct representing the tab in the web browser.
/// It provide event handling.
pub struct Window {
    document: Document,
    window: WebSysWindow,
    events: EventManager
}

impl Window {
    /// Create a [Canvas](../canvas/struct.Canvas.html) and a Window.
    /// Events will not be activated!
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

    /// Create a [Canvas](../canvas/struct.Canvas.html) and a Window.
    /// You may specify which [types of event](../../inputs/event/types/index.html) you want to record. 
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use wasm_game_lib::inputs::event::types::*; // there are only a few events so the wildcard is not a problem
    /// use wasm_game_lib::graphics::window::Window;
    /// 
    /// // create a window recording three types of event
    /// let (window, canvas) = Window::init_with_events(MOUSE_EVENT + KEYBOARD_EVENT + FOCUS_EVENT);
    /// ```
    pub fn init_with_events(events: u8) -> (Window, Canvas) {
        let mouse_events    = 0b00000001 & events == 0b00000001;
        let key_events      = 0b00000010 & events == 0b00000010;
        let size_events     = 0b00000100 & events == 0b00000100;
        let focus_events    = 0b00001000 & events == 0b00001000;
        let joystick_events = 0b00010000 & events == 0b00010000;

        let (mut window, canvas) = Window::init();
        if mouse_events {
            window.events.start_recording_mouse_events();
        }
        if key_events {
            window.events.start_recording_keyboard_events();
        }
        if size_events {
            window.events.start_recording_size_events();
        }
        if focus_events {
            window.events.start_recording_focus_events();
        }
        if joystick_events {
            unimplemented!("joysticks are not implemented for now");
        }
        
        (window, canvas)
    }

    /// Return an Iterator of every events fired after the last call of this method.
    /// Make sure events are activated: [init_with_events()](#methods.init_with_events).
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use wasm_game_lib::inputs::event::types::*;
    /// use wasm_game_lib::graphics::window::Window;
    /// 
    /// // create a window recording three types of event
    /// let (mut window, canvas) = Window::init_with_events(MOUSE_EVENT + KEYBOARD_EVENT + FOCUS_EVENT);
    /// 
    /// loop {
    ///     for event in window.poll_events() {
    ///         // Do something with your event
    ///     }
    /// }
    /// ```
    pub fn poll_events(&mut self) -> &mut EventManager {
        &mut self.events
    }

    /// Set the title of the tab.
    pub fn set_title(&mut self, title: &str) {
        self.document.set_title(title)
    }

    /// Get the title of the tab.
    pub fn get_title(&self) -> String {
        self.document.title()
    }
    
    /// Set the icon of the tab.
    /// UNIMPLEMENTED
    pub fn set_icon(&mut self, icon: &Image) {
        unimplemented!()
    }
    
    /// Get the icon of the tab.
    /// UNIMPLEMENTED
    pub fn get_icon(&self) -> Image {
        unimplemented!()
    }
}

