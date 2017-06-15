use std::path::Path;

use piston_window::*;


pub struct Tileset {
    tex: G2dTexture,
    margin: i32,
    tile_w: i32,
    tile_h: i32
}

impl Tileset {
    pub fn new<P: AsRef<Path>>(win: &mut PistonWindow, path: P, m: i32, w: i32, h: i32) -> Tileset {
        Tileset {
            tex: Texture::from_path(&mut win.factory, path.as_ref(),
                                    Flip::None, &TextureSettings::new()).unwrap(),
            margin: m,
            tile_w: w,
            tile_h: h
        }
    }

    fn get_full_rect(&self, x : i32, y : i32) -> [f64; 4] {
        [ 0.0, 0.0, self.tex.get_width() as f64, self.tex.get_height() as f64 ]
    }

    fn get_draw_rect(&self, x : i32, y : i32) -> [f64; 4] {
        [
            (x * self.tile_w + x * self.margin) as f64,
            (y * self.tile_h + y * self.margin) as f64,
            self.tile_w as f64,
            self.tile_h as f64
        ]
    }

    pub fn draw<G: Graphics>(&self, g: &mut G, trans: math::Matrix2d,
                             x: i32, y: i32, at_x: i32, at_y: i32) {
        image::draw_many(
            &[(self.get_full_rect(x, y), self.get_draw_rect(x, y))],
            [1.0, 1.0, 1.0, 1.0],
            &self.tex,
            &DrawState::new_alpha(),
            trans.trans(at_x as f64, at_y as f64),
            g
        );
    }
}
