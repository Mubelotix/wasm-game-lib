//! Welcome!  
//!   
//! The goal of this crate is to help you to make great 2d games running in web browsers.
//! This crate is very similar to SFML.
//! To use this crate, you will have to use wasm-pack. You can install wasm-pack with:
//! 
//! ```text
//! cargo install wasm-pack
//! ```
//!   
//!   
//! To create your game crate, the best way is to use:
//! ```text
//! wasm-pack new
//! ```
//!   
//!   
//! Then, put this crate on Cargo.toml as usual. You can now compile the crate with:
//! ```text
//! wasm-pack build --target=web
//! ```
//! 
//! I suggest you to put this on lib.rs.
//! 
//! ```rust
//! #[allow(unused_imports)]
//! 
//! use wasm_bindgen::{prelude::*, JsCast};
//! use wasm_game_lib::graphics::image::Image;
//! use wasm_game_lib::graphics::sprite::Sprite;
//! use wasm_game_lib::inputs::event::Event;
//! use wasm_game_lib::graphics::window::Window;
//! use wasm_game_lib::system::log;
//! use wasm_game_lib::inputs::event::types::*;
//! use wasm_game_lib::system::sleep;
//! use std::time::Duration;
//! 
//! #[wasm_bindgen(start)]
//! pub async fn start() -> Result<(), JsValue> {
//!     let (mut window, mut canvas) = Window::init_with_events(MOUSE_EVENT + KEYBOARD_EVENT + RESIZE_EVENT + FOCUS_EVENT);
//! 
//!     // load images and fonts here
//!     // you could make a progress bar
//! 
//!     loop {
//!         for event in window.poll_events() {
//!             // do something with events
//!             log(&format!("{:?}", event));
//!         }
//! 
//!         canvas.clear();
//!         // canvas.draw(&object);
//! 
//!         sleep(Duration::from_millis(16)).await;
//!     }
//! 
//!     Ok(())
//! }
//! ```
#![warn(missing_docs)]

pub mod graphics;
pub mod inputs;
/// You will need this module for various things.
pub mod system;