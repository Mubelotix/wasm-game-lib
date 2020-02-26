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

#![warn(missing_docs)]

pub mod graphics;
pub mod inputs;
pub mod system;