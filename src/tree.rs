use sdl2::render::Canvas;
use sdl2::video::Window;
use std::sync::Arc;
use world::InWorld;
use sdl2::rect::Rect;
use sprite::tree::{TreeLayout, TreeSprite, TreeSpritesheet};
use sprite::SpritesheetLayout;

const SCALE: u32 = 1;

pub struct Tree<'s> {
    spritesheet: Arc<TreeSpritesheet<'s>>,
    sprite: TreeSprite,

    scale: u32,

    pub world_posx: i32,
    pub world_posy: i32,
}

impl<'s> Tree<'s> {
    pub fn new(
        spritesheet: Arc<TreeSpritesheet<'s>>,
        sprite: TreeSprite,
        world_posx: i32,
        world_posy: i32,
    ) -> Self {
        Tree {
            spritesheet,
            sprite,
            scale: SCALE,
            world_posx,
            world_posy,
        }
    }
}

impl<'a> InWorld for Tree<'a> {
    fn world_rect(&self) -> Rect {
        let dimensions = <TreeLayout as SpritesheetLayout>::get_dimensions();
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
