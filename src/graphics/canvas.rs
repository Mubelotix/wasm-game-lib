use super::drawable::Drawable;
use super::color::Color;
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
    pub fn clear_with_black(&mut self) {
        self.fill_rect(
            (0.0, 0.0),
            (
                f64::from(self.element.width()),
                f64::from(self.element.height()),
            ),
            Color::black()
        );
    }

    /// Clear all the canvas with a [Color](../color/struct.Color.html).
    pub fn clear_with_color(&mut self, color: Color) {
        self.fill_rect(
            (0.0, 0.0),
            (
                f64::from(self.element.width()),
                f64::from(self.element.height()),
            ),
            color
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

    /// You can use the canvas rendering context to make advanced drawing
    pub fn get_2d_canvas_rendering_context(&mut self) -> &mut web_sys::CanvasRenderingContext2d {
        &mut self.context
    }

    /// Fill a part of the canvas with a [Color](../color/struct.Color.html).
    pub fn fill_rect(&mut self, (x, y): (f64, f64), (w, h): (f64, f64), color: Color) {
        self.context.set_fill_style(&JsValue::from_str(&color.to_string()));
        self.context.fill_rect(x, y, w, h);
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

    /// Return the width and the height of a canvas.
    pub fn get_size(&self) -> (u32, u32) {
        (self.element.width(), self.element.height())
    }
}

/// An enum representing a [lineCap mode](https://developer.mozilla.org/fr/docs/Web/API/CanvasRenderingContext2D/lineCap).
/// You may want to use [LineStyle](struct.LineStyle.html) for a complete set of options.
/// 
/// # Example
/// 
/// ![line cap demonstration](https://media.prod.mdn.mozit.cloud/attachments/2012/07/09/236/50366ad18b04b40276d6ef95d76281b1/Canvas_linecap.png)
pub enum LineCap {
    /// The first line of the example above
    Butt,
    /// The second line of the example above
    Round,
    /// The third line of the example above
    Square
}

impl ToString for LineCap {
    fn to_string(&self) -> String {
        match self {
            LineCap::Butt => String::from("butt"),
            LineCap::Round => String::from("round"),
            LineCap::Square => String::from("square"),
        }
    }
}

/// An enum representing the [lineJoin mode](https://developer.mozilla.org/fr/docs/Web/API/CanvasRenderingContext2D/lineJoin).
/// You may want to use [LineStyle](struct.LineStyle.html) for a complete set of options.
/// 
/// # Example
/// 
/// ![line join demonstration](https://media.prod.mdn.mozit.cloud/attachments/2012/07/09/237/2b7a14b3921934ae35486afd6ba6704a/Canvas_linejoin.png)
pub enum LineJoin {
    /// The first line of the example above
    Round,
    /// The second line of the example above
    Bevel,
    /// The third line of the example above
    Miter,
}

impl ToString for LineJoin {
    fn to_string(&self) -> String {
        match self {
            LineJoin::Miter => String::from("miter"),
            LineJoin::Round => String::from("round"),
            LineJoin::Bevel => String::from("bevel"),
        }
    }
}

/// A struct containing every line option.
pub struct LineStyle {
    /// The color of the line
    pub color: Color,
    /// The width of the line in pixels
    pub size: f64,
    /// The lineCap mode
    pub cap: LineCap,
    /// The lineJoin mode
    pub join: LineJoin
}

impl LineStyle {
    /// Apply these properties on a canvas
    pub fn apply_on_canvas(&self, canvas: &mut Canvas) {
        canvas.context.set_line_width(self.size);
        canvas.context.set_stroke_style(&JsValue::from_str(&self.color.to_string()));
        canvas.context.set_line_cap(&self.cap.to_string());
        canvas.context.set_line_join(&self.join.to_string());
    }
}

impl Default for LineStyle {
    fn default() -> LineStyle {
        LineStyle {
            color: Color::black(),
            size: 3.0,
            cap: LineCap::Butt,
            join: LineJoin::Miter
        }
    }
}