use sdl2::rect::Rect;
use sprite::{Spritesheet, SpritesheetLayout};

const SPRITE_WIDTH: u32 = 128;
const SPRITE_HEIGHT: u32 = 192;

pub type TreeSpritesheet<'t> = Spritesheet<'t, TreeLayout>;

pub struct TreeLayout;

#[derive(Clone, PartialEq)]
pub enum TreeSprite {
    Tree1,
}

impl SpritesheetLayout for TreeLayout {
    type Sprite = TreeSprite;

    fn get_dimensions() -> (u32, u32) {
        (SPRITE_WIDTH, SPRITE_HEIGHT)
    }

    fn get_sprite(spr: &Self::Sprite) -> Rect {
        let grid = match *spr {
            TreeSprite::Tree1 => (0, 0),
        };

        Rect::new(
            grid.0 * SPRITE_WIDTH as i32,
            grid.1 * SPRITE_HEIGHT as i32,
            SPRITE_WIDTH as u32,
            SPRITE_HEIGHT as u32,
        )
    }
}
