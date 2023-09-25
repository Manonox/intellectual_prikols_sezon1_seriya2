use std::io::{self, Write};
use std::str::FromStr;
// use std::{thread, time::Duration};
use raylib::prelude::*;
use raylib::prelude::RaylibDraw;


mod field;
use crate::astar::AStarSolution;
use crate::field::Drawable;

mod astar;
mod tests;



struct Game {
    field: field::Field
}


fn main() -> io::Result<()> {
    let mut game : Game = Game {
        field: field::Field::new()
    };

    loop {
        let input: String = read("Field: 0x");

        let result = field::Field::from_string(input).ok();
        if result.is_none() { println!("Incorrent input!"); continue; }

        let field = result.unwrap();
        if !field.is_solvable() { println!("Field isn't solvable!"); continue; }

        game.field = field;
        break;
    }

    game.field.display();
    println!("\n<=======>\n");
    
    if read("Do you want to: \n- Initialize solver? (true)\n- Play it yourself? (false)\n> (true/false): ") {
        init_ida_solver(&mut game);
        return Ok(());
    }
    
    println!("\n");

    let (mut handle, thread) = raylib::init()
        .size(256, 256)
        .title("Пятнашки")
        .build();
    
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
            //drop(gfx);
        }
        
        if handle.window_should_close() { break }
    }

    Ok(())
}



fn read<T: FromStr>(name: &str) -> T {
    loop {
        print!("{name}");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        input.pop(); input.pop();

        if let Ok(x) = input.parse::<T>() {
            return x;
        }
    }
}


fn init_solver(game: &mut Game) {
    let mut star = astar::AStar::new(&game.field);
    let mut result_option: Option<Result<AStarSolution, ()>> = Default::default();

    const MAX_STEPS: i32 = 160000000;
    let mut steps = 0;
    while result_option.is_none() && steps < MAX_STEPS {
        result_option = star.step();
        steps += 1;
    }

    let Some(result) = result_option
    else { println!("Step limit exceeded!"); return; };

    let Ok(solution) = result
    else { println!("Something went wrong..."); return; };

    println!("===========\n{} Moves:", solution.moves.len());
    solution.moves.iter().for_each(|x| {
        print!("{}", x);
    });

    println!("\n===========\n");
}

fn init_ida_solver(game: &mut Game) {
    let mut idastar = astar::IDAStar::new(&game.field);
    let result = idastar.run();

    let Ok(solution) = result
    else { println!("Something went wrong..."); return; };

    println!("===========\n{} Moves:", solution.moves.len());
    solution.moves.iter().for_each(|x| {
        print!("{}", x);
    });

    println!("\n===========\n");
}



#[derive(PartialEq)]
enum KeyPressResult {
    None,
    Success,
    Failure,
}

fn key_pressed(game: &mut Game, key: KeyboardKey) {
    let result = match key {
        KeyboardKey::KEY_UP     => { if game.field.up() { KeyPressResult::Success } else { KeyPressResult::Failure } }
        KeyboardKey::KEY_DOWN   => { if game.field.down() { KeyPressResult::Success } else { KeyPressResult::Failure }   }
        KeyboardKey::KEY_LEFT   => { if game.field.left() { KeyPressResult::Success } else { KeyPressResult::Failure } }
        KeyboardKey::KEY_RIGHT  => { if game.field.right() { KeyPressResult::Success } else { KeyPressResult::Failure } }

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
