use core::ops::AddAssign;
use super::drawable::Drawable;
use super::image::Image;
use super::canvas::Canvas;

/// Use a Sprite for an object on your game which can move.
/// 
/// # Example
/// 
/// ```rust
/// use wasm_game_lib::graphics::image::Image;
/// use wasm_game_lib::graphics::sprite::Sprite;
/// # async fn test() {
/// # let (window, mut canvas) = Window::init(); 
/// // load a texture from the web
/// let ferris_texture = Image::load("https://rustacean.net/assets/cuddlyferris.svg").await.unwrap();
/// 
/// // create a sprite
/// let ferris = Sprite::<u32>::new((0,0), &ferris_texture, (0,0));
/// 
/// // draw the sprite on a canvas
/// canvas.draw(&sprite);
/// # }
pub struct Sprite<'a, T: Into<f64> + Copy + AddAssign> {
    /// The texture of the Sprite
    pub texture: &'a Image,
    /// Where the Sprite is located on the screen
    pub coords: (T, T),
    /// The point on the texture which is considered as the center of the Sprite.
    /// The coordinates of this point must be relative to the top-left corner of the object.
    pub origin: (T, T)
}

impl<'a, T: Into<f64> + Copy + AddAssign> Sprite<'a, T> {
    /// Create a new Sprite.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use wasm_game_lib::graphics::image::Image;
    /// use wasm_game_lib::graphics::sprite::Sprite;
    /// # async fn test() {
    /// // load a texture from the web
    /// let ferris_texture = Image::load("https://rustacean.net/assets/cuddlyferris.svg").await.unwrap();
    /// 
    /// // create a sprite
    /// let ferris = Sprite::<u32>::new((0,0), &ferris_texture, (0,0));
    /// # }
    /// ```
    pub fn new(coords: (T, T), texture: &Image, origin: (T, T)) -> Sprite<T> {
        Sprite {
            coords,
            texture,
            origin
        }
    }

    /// Set the origin.
    /// The origin is the point on the texture which is considered as the center of the Sprite.
    /// The coordinates of this point must be relative to the top-left corner of the object.
    pub fn set_origin(&mut self, origin: (T, T)) {
        self.origin = origin
    }

    /// Return the origin.
    /// The origin is the point on the texture which is considered as the center of the Sprite.
    /// The coordinates of this point must be relative to the top-left corner of the object.
    pub fn get_origin(&self) -> (T, T) {
        self.origin
    }

    /// Return the texture.
    pub fn get_texture(&self) -> &Image {
        self.texture
    }

    /// Set the texture
    pub fn set_texture(&mut self, texture: &'a Image) {
        self.texture = texture
    }

    /// Set the x coordinate.
    pub fn set_x(&mut self, x: T) {
        self.coords.0 = x;
    }

    /// Set the y coordinate.
    pub fn set_y(&mut self, y: T) {
        self.coords.1 = y;
    }

    /// Set the coordinates.
    pub fn set_coords(&mut self, coords: (T, T)) {
        self.coords = coords;
    }

    /// Return the x coordinate.
    pub fn get_x(&mut self) -> T {
        self.coords.0
    }

    /// Return the y coordinate.
    pub fn get_y(&mut self) -> T {
        self.coords.1
    }

    /// Return the coordinates.
    pub fn get_coords(&mut self) -> (T, T) {
        self.coords
    }

    /// Add a value to the actual coordinates.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use wasm_game_lib::graphics::image::Image;
    /// use wasm_game_lib::graphics::sprite::Sprite;
    /// # async fn test() {
    /// # let texture = Image::load("https://rustacean.net/assets/cuddlyferris.svg").await.unwrap();
    /// # let mut sprite = Sprite::<u32>::new((0,0), &texture, (0,0));
    /// // move a sprite one pixel right and two pixels down.
    /// sprite.move_by((1,2));
    /// # }
    /// ```
    pub fn move_by(&mut self, movement: (T, T)) {
        self.coords.0 += movement.0;
        self.coords.1 += movement.1;
    }
}

impl<'a, T: Into<f64> + Copy + AddAssign> Drawable for Sprite<'a, T> {
    fn draw_on_canvas(&self, canvas: &mut Canvas) {
        canvas.draw_image((self.coords.0.into() - self.origin.0.into(), self.coords.1.into() - self.origin.1.into()), &self.texture);
    }
}