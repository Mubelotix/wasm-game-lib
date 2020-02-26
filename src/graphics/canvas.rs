use super::drawable::Drawable;
use super::image::Image;
use wasm_bindgen::JsCast;

pub struct Canvas {
    pub(crate) context: web_sys::CanvasRenderingContext2d,
    pub(crate) element: web_sys::HtmlCanvasElement
}

impl Canvas {
    pub fn new() -> Canvas {
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

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

    pub fn clear_rect(&mut self, (x, y): (f64, f64), (w, h): (f64, f64)) {
        self.context .clear_rect(x, y, w, h);
    }

    /// Clear the canvas
    pub fn clear(&mut self) {
        self.clear_rect(
            (0.0, 0.0),
            (
                f64::from(self.element.width()),
                f64::from(self.element.height()),
            ),
        )
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

    pub fn fill_text(&mut self, (x, y): (usize, usize), text: &str, max_width: Option<usize>) {
        if let Some(max_width) = max_width {
            self.context.fill_text_with_max_width(text, x as f64, y as f64, max_width as f64).unwrap();
        } else {
            self.context.fill_text(text, x as f64, y as f64).unwrap();
        }
    }
}