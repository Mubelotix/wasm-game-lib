use std::time::Duration;
use js_sys::{Promise};
use web_sys::{window};
use wasm_bindgen_futures::JsFuture;
use web_sys::console::log_1;
use wasm_bindgen::JsValue;

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

/// Log a message in the console.
pub fn log(message: &str) {
    log_1(&JsValue::from_str(message));
}