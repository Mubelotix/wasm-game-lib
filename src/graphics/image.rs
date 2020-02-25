use web_sys::HtmlImageElement;
use wasm_bindgen::JsCast;
use js_sys::Promise;
use wasm_bindgen_futures::JsFuture;

#[derive(Debug)]
pub struct Image {
    element: HtmlImageElement,
}

impl Image {
    pub async fn load(url: &str) -> Result<Image, ()> {
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document
            .create_element("img")
            .unwrap()
            .dyn_into::<web_sys::HtmlImageElement>()
            .unwrap();
        element.set_src(url);

        let result = JsFuture::from(Promise::new(&mut |yes, no| {
            element.add_event_listener_with_callback("load", &yes).unwrap();
            element.add_event_listener_with_callback("error", &no).unwrap();
        })).await;

        if result.is_ok() {
            Ok(Image {
                element
            })
        } else {
            Err(())
        }
    }

    pub(crate) fn get_html_element(&self) -> &HtmlImageElement {
        &self.element
    }

    pub fn get_width<T: From<u32>>(&self) -> T {
        self.element.width().into()
    }

    pub fn get_height<T: From<u32>>(&self) -> T {
        self.element.height().into()
    }

    pub fn get_size<T: From<u32>>(&self) -> (T, T) {
        (self.element.width().into(), self.element.height().into())
    }
}