use sdl2::render::Canvas;
use sdl2::video::Window;
use tile::Tile;
use map::Map;
use sdl2::event::Event;
use sdl2::rect::Rect;

pub trait InWorld {
    fn world_rect(&self) -> Rect;

    fn update(&mut self) {}
    fn event_update(&mut self, &Event) {}

    fn render(&self, &mut Canvas<Window>, i32, i32) {}
}

pub struct World<'a> {
    tiles: Vec<Tile<'a>>,
    width: u32,
    height: u32,
}

impl<'a> World<'a> {
    pub fn new(map: Map<'a>) -> Self {
        World {
            tiles: map.tiles,
            width: map.width,
            height: map.height,
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn update(&mut self) {}

    pub fn event_update(&mut self, _event: &Event) {}

    pub fn render(&self, canvas: &mut Canvas<Window>, viewport: Rect) {
        for tile in &self.tiles {
            let world_rect = tile.world_rect();

            // TODO: optimise not to render items outside viewport.
            tile.render(canvas, world_rect.x - viewport.x, world_rect.y - viewport.y);
        }
    }
}
