use super::drawable::Drawable;
use super::image::Image;
use wasm_bindgen::JsCast;

pub struct Canvas {
    context: web_sys::CanvasRenderingContext2d,
    element: web_sys::HtmlCanvasElement
}

impl Canvas {
    pub fn new(visible: bool) -> Canvas {
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        
        if visible {
            document
                .body()
                .unwrap()
                .append_child(&element)
                .unwrap();
            
            element.set_width(document.document_element().unwrap().client_width() as u32);
            element.set_height(document.document_element().unwrap().client_height() as u32);
        }

        let context = element
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        Canvas {
            context,
            element
        }
    }

    pub fn clear(&mut self) {
        unimplemented!()
    }

    pub fn draw(&mut self, object: &impl Drawable) {
        object.draw_on_canvas(self);
    }

    pub fn get_size(&self) -> (usize, usize) {
        unimplemented!()
    }

    pub fn draw_image(&mut self, (x, y): (f64, f64), image: &Image) {
        self.context
            .draw_image_with_html_image_element(
                image.get_html_element(),
                x,
                y,
            )
            .unwrap();
    }

    pub fn draw_canvas(&mut self, (x, y): (f64, f64), canvas: &Canvas) {
        self.context
            .draw_image_with_html_canvas_element(
                &canvas.element,
                x,
                y,
            )
            .unwrap();
    }
}