#![cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use wasm_game_lib::system::sleep;
use wasm_game_lib::graphics::canvas::Canvas;
use wasm_game_lib::graphics::sprite::Sprite;
use std::time::Duration;

#[wasm_bindgen_test]
fn pass() {
    
}