mod draw;
mod game;
mod snake;

use draw::to_coord_u32;
use game::{Difficulty, Game};
use piston_window::{types::Color, *};
use std::io::{self, Write};

const BACKGROUND_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (20, 20);

    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut game = Game::new(width, height, loop { 
        print!("Enter difficulty: ");
        io::stdout().flush().expect("Failed to flush the buffer");

        let mut difficulty = String::new();
        io::stdin().read_line(&mut difficulty).expect("Failed to read a line");

        match difficulty.trim_end().to_lowercase().as_str() {
            "e" | "easy" => break Difficulty::Easy,
            "n" | "normal" => break Difficulty::Normal,
            "h" | "hard" => break Difficulty::Hard,
            "i" | "insane" => break Difficulty::Insane,
            _ => {}
        };
    });

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&event, |ctx, graphics_buf, _gfx_device| {
            clear(BACKGROUND_COLOR, graphics_buf);
            game.draw(&ctx, graphics_buf);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
