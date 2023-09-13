use std::io;
use raylib::prelude::*;
// use std::time::Instant;

mod field;
mod tests;

use raylib::prelude::RaylibDraw;

use crate::field::Drawable;


struct Game {
    field: field::Field
}


fn main() -> io::Result<()> {
    let (mut handle, thread) = raylib::init()
        .size(256, 256)
        .title("Пятнашки")
        .build();

    let mut game : Game = Game {
        field: field::Field::from("FE169B4C0A73D852")
    };

    loop {

        // Input
        {
            process_input(&mut game, &mut handle);
        }

        // Draw
        {
            let mut gfx = handle.begin_drawing(&thread);
            
            gfx.clear_background(Color::WHITE);
            // gfx.draw_fps(0, 0);
            game.field.draw(gfx);
        }
        
        if handle.window_should_close() { break }
    }

    Ok(())
}


#[derive(PartialEq)]
enum KeyPressResult {
    None,
    Success,
    Failure,
}

fn key_pressed(game: &mut Game, key: KeyboardKey) {
    let result = match key {
        KeyboardKey::KEY_UP     => { if game.field.down() { KeyPressResult::Success } else { KeyPressResult::Failure } }
        KeyboardKey::KEY_DOWN   => { if game.field.up() { KeyPressResult::Success } else { KeyPressResult::Failure }   }
        KeyboardKey::KEY_LEFT   => { if game.field.right() { KeyPressResult::Success } else { KeyPressResult::Failure } }
        KeyboardKey::KEY_RIGHT  => { if game.field.left() { KeyPressResult::Success } else { KeyPressResult::Failure } }

        _ => { KeyPressResult::None }
    };

    if result == KeyPressResult::Failure {
        println!("Bad move");
    }
}


fn process_input(game: &mut Game, handle: &mut RaylibHandle) {
    let mut key_opt = Some(KeyboardKey::KEY_NULL);
    while key_opt.is_some() {
        key_opt = handle.get_key_pressed();

        if let Some(key) = key_opt {
            key_pressed(game, key);
        }
    }
}
