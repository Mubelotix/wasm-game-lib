use super::drawable::Drawable;
use super::image::Image;
use wasm_bindgen::{JsCast, JsValue};

/// A Canvas is an object on which you can draw.
/// Only the main Canvas is displayed (returned by Window::init()).
/// 
/// # Example
/// 
/// ```rust
/// use wasm_game_lib::graphics::window::Window;
/// use wasm_game_lib::graphics::image::Image;
/// use wasm_game_lib::graphics::sprite::Sprite;
/// use wasm_game_lib::system::sleep;
/// use std::time::Duration;
/// 
/// # async fn test() {
/// // Create a sprite to demonstrate how to draw a sprite on the canvas
/// let texture = Image::load("https://www.gravatar.com/avatar/419218774d04a581476ea1887a0921e0?s=128&d=identicon&r=PG").await.unwrap();
/// let sprite = Sprite::<u32>::new((0,0), &texture, (150, 150));
/// 
/// // create the main canvas
/// let (window, mut canvas) = Window::init(); 
/// 
/// loop {
///     canvas.clear();         // clear the canvas at each iteration
///     canvas.draw(&sprite);   // draw a sprite on the canvas
///     // note that canvas.display() is not needed unlike a lot of graphics libraries
///     
///     // you may want to slow down the loop to keep your game at 60fps
///     sleep(Duration::from_millis(16)).await; 
/// }
/// # }
/// ```
pub struct Canvas {
    pub(crate) context: web_sys::CanvasRenderingContext2d,
    pub(crate) element: web_sys::HtmlCanvasElement
}

impl Canvas {
    /// Create a canvas which will not be displayed.
    /// To create a displayed canvas, see [Window::init()](../window/struct.Window.html#method.init).
    /// Creating a undisplayed canvas can be useful because a canvas is drawable on another canvas.
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

    /// Fill a part of the canvas with a color.
    /// 
    /// Example valid values for the color parameter: 
    /// - "blue",
    /// - "#241F45",
    /// - "#aaa"
    pub fn fill_rect(&mut self, (x, y): (f64, f64), (w, h): (f64, f64), color: &str) {
        self.context.set_fill_style(&JsValue::from_str(color));
        self.context.fill_rect(x, y, w, h);
    }

    /// Clear a part of the canvas.
    pub fn clear_rect(&mut self, (x, y): (f64, f64), (w, h): (f64, f64)) {
        self.context.clear_rect(x, y, w, h);
    }

    /// Clear all the canvas with a transparent black (white).
    pub fn clear(&mut self) {
        self.clear_rect(
            (0.0, 0.0),
            (
                f64::from(self.element.width()),
                f64::from(self.element.height()),
            ),
        )
    }

    /// Clear all the canvas with a visible black.
    pub fn clear_black(&mut self) {
        self.fill_rect(
            (0.0, 0.0),
            (
                f64::from(self.element.width()),
                f64::from(self.element.height()),
            ),
            "black"
        );
}

    /// Draw an object implementing the [Drawable trait](../drawable/trait.Drawable.html) on the canvas.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// # use wasm_game_lib::graphics::window::Window;
    /// # use wasm_game_lib::graphics::image::Image;
    /// # use wasm_game_lib::graphics::sprite::Sprite;
    /// # use wasm_game_lib::system::sleep;
    /// # use std::time::Duration;
    /// # async fn test() {
    /// // create a sprite to draw it on the canvas
    /// let texture = Image::load("https://www.gravatar.com/avatar/419218774d04a581476ea1887a0921e0?s=128&d=identicon&r=PG").await.unwrap();
    /// let sprite = Sprite::<u32>::new((0,0), &texture, (150, 150));
    /// 
    /// // create the main canvas
    /// let (window, mut canvas) = Window::init(); 
    /// 
    /// // draw the sprite on the canvas
    /// canvas.draw(&sprite);
    /// # }
    /// ```
    /// 
    /// See above for a more complete example.
    pub fn draw(&mut self, object: &impl Drawable) {
        object.draw_on_canvas(self);
    }

    /// Return the width and the height of a canvas.
    pub fn get_size(&self) -> (usize, usize) {
        unimplemented!()
    }

    /// Draw an image at a specific position.
    /// This method is intended to be used inside the [Drawable trait](../drawable/trait.Drawable.html).
    /// In the main code of your game, you should use a [Sprite](../sprite/struct.Sprite.html) and the [draw](#method.draw) method.
    pub fn draw_image(&mut self, (x, y): (f64, f64), image: &Image) {
        self.context
            .draw_image_with_html_image_element(
                image.get_html_element(),
                x,
                y,
            )
            .unwrap();
    }

    /// Draw a canvas at a specific position.
    pub fn draw_canvas(&mut self, (x, y): (f64, f64), canvas: &Canvas) {
        self.context
            .draw_image_with_html_canvas_element(
                &canvas.element,
                x,
                y,
            )
            .unwrap();
    }

    /// Print text on the canvas.
    /// The [Text](../text/struct.Text.html) struct is a better way to print text.
    pub fn fill_text(&mut self, (x, y): (usize, usize), text: &str, max_width: Option<usize>) {
        if let Some(max_width) = max_width {
            self.context.fill_text_with_max_width(text, x as f64, y as f64, max_width as f64).unwrap();
        } else {
            self.context.fill_text(text, x as f64, y as f64).unwrap();
        }
    }
    
    /// Set the canvas width in pixels
    pub fn set_width(&mut self, width: u32) {
        self.element.set_width(width);
    }

    /// Set the canvas height in pixels
    pub fn set_height(&mut self, height: u32) {
        self.element.set_height(height);
    }

    /// Return the actual canvas width in pixels
    pub fn get_width(&self) -> u32 {
        self.element.width()
    }

    /// Return the actual canvas height in pixels
    pub fn get_height(&self) -> u32 {
        self.element.height()
    }
}