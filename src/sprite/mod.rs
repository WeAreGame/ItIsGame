use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::pixels::Color;
use sdl2::video::{Window, WindowContext};
use std::marker::PhantomData;
use std::path::Path;

pub mod orb;
pub mod tile;
pub mod tree;

pub trait SpritesheetLayout {
    type Sprite;

    fn get_dimensions() -> (u32, u32);
    fn get_sprite(&Self::Sprite) -> Rect;
}

pub struct Spritesheet<'t, SL: SpritesheetLayout> {
    texture: Texture<'t>,
    layout: PhantomData<SL>,
}

impl<'t, SL: SpritesheetLayout> Spritesheet<'t, SL> {
    pub fn new(texture_creator: &'t TextureCreator<WindowContext>, spritesheet: &str) -> Self {
        let texture = texture_creator
            .load_texture(Path::new(spritesheet))
            .unwrap();

        Spritesheet {
            texture,
            layout: PhantomData,
        }
    }

    pub fn draw(
        &self,
        canvas: &mut Canvas<Window>,
        sprite: &SL::Sprite,
        scale: u32,
        x: i32,
        y: i32,
        show_perimeter: bool,
    ) {
        let dimensions = <SL as SpritesheetLayout>::get_dimensions();
        let src = <SL as SpritesheetLayout>::get_sprite(sprite);
        let dst = Rect::new(x, y, dimensions.0 * scale, dimensions.1 * scale);

        // Draw sprite.
        canvas.copy(&self.texture, Some(src), Some(dst)).unwrap();

        // Draw perimeter.
        if show_perimeter {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.draw_rect(dst).unwrap();
        }
    }
}
