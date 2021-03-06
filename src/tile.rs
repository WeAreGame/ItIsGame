use sdl2::render::Canvas;
use sdl2::video::Window;
use world::InWorld;
use sdl2::rect::Rect;
use sprite::tile::{TileLayout, TileSprite, TileSpritesheet};
use sprite::SpritesheetLayout;

const SCALE: u32 = 2;

pub struct Tile<'s> {
    spritesheet: &'s TileSpritesheet<'s>,
    sprite: TileSprite,

    scale: u32,

    pub world_posx: i32,
    pub world_posy: i32,
}

impl<'s> Tile<'s> {
    pub fn new(
        spritesheet: &'s TileSpritesheet<'s>,
        sprite: TileSprite,
        world_posx: i32,
        world_posy: i32,
    ) -> Self {
        Tile {
            spritesheet,
            sprite,
            scale: SCALE,
            world_posx,
            world_posy,
        }
    }

    pub fn size() -> (u32, u32) {
        let spritesheet_size = <TileLayout as SpritesheetLayout>::get_dimensions();
        (SCALE * spritesheet_size.0, SCALE * spritesheet_size.1)
    }
}

impl<'a> InWorld for Tile<'a> {
    fn world_rect(&self) -> Rect {
        let dimensions = <TileLayout as SpritesheetLayout>::get_dimensions();
        Rect::new(self.world_posx, self.world_posy, dimensions.0, dimensions.1)
    }

    fn render(
        &self,
        canvas: &mut Canvas<Window>,
        screen_x: i32,
        screen_y: i32,
        show_perimeter: bool,
    ) {
        self.spritesheet.draw(
            canvas,
            &self.sprite,
            self.scale,
            screen_x,
            screen_y,
            show_perimeter,
        );
    }
}
