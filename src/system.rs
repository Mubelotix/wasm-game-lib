use std::time::Duration;
use js_sys::{Promise};
use web_sys::{window};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::prelude::*;


/// This is the wasm version of the sleep function.
/// For now it is the only way to sleep.
/// The precision of this function is 1ms.
pub async fn sleep(duration: Duration) {
    let promise = Promise::new(&mut |yes, _| {
        window().unwrap().set_timeout_with_callback_and_timeout_and_arguments_0(&yes, duration.as_millis() as i32).unwrap();
    });
    let js_fut = JsFuture::from(promise);
    js_fut.await.unwrap();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn error(s: &str);
}

#[macro_export]
/// A println-like macro. 
/// **Warning**: This is very slow.
macro_rules! log {
    ($($t:tt)*) => ($crate::system::log(&format_args!($($t)*).to_string()))
}

#[macro_export]
/// A eprintln-like macro.
/// **Warning**: This is **extremely** slow.
macro_rules! elog {
    ($($t:tt)*) => ($crate::system::error(&format_args!($($t)*).to_string()))
}