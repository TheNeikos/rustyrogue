extern crate piston_window;

use piston_window::*;

mod tile;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("RustyRogue", (720, 720))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Could not start window: {}", e));

    let ts = tile::Tileset::new(&mut window, "assets/spritesheet/roguelike_sheet_transparent.png",
                                1, 16, 16);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |_c, g| {
            clear([00.5, 1.0, 0.5, 1.0], g);
        });
    }
}
