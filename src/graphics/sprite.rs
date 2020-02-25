use core::ops::AddAssign;
use super::drawable::Drawable;
use super::image::Image;
use super::canvas::Canvas;

pub struct Sprite<'a, T: Into<f64> + Copy + AddAssign> {
    pub texture: &'a Image,
    pub coords: (T, T),
    pub origin: (T, T)
}

impl<'a, T: Into<f64> + Copy + AddAssign> Sprite<'a, T> {
    pub fn new(coords: (T, T), texture: &Image, origin: (T, T)) -> Sprite<T> {
        Sprite {
            coords,
            texture,
            origin
        }
    }

    pub fn set_origin(&mut self, origin: (T, T)) {
        self.origin = origin
    }

    pub fn get_origin(&self) -> (T, T) {
        self.origin
    }

    pub fn get_texture(&self) -> &Image {
        self.texture
    }

    pub fn set_texture(&mut self, texture: &'a Image) {
        self.texture = texture
    }

    pub fn set_x(&mut self, x: T) {
        self.coords.0 = x;
    }

    pub fn set_y(&mut self, y: T) {
        self.coords.1 = y;
    }

    pub fn set_coords(&mut self, coords: (T, T)) {
        self.coords = coords;
    }

    pub fn get_x(&mut self) -> T {
        self.coords.0
    }

    pub fn get_y(&mut self) -> T {
        self.coords.1
    }

    pub fn get_coords(&mut self) -> (T, T) {
        self.coords
    }

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