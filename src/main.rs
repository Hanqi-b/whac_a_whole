use macroquad::prelude::*;
use std::sync::Arc;

mod menu;
mod game1;

use menu::draw_menu;
use game1::Game;

#[derive(PartialEq)]
enum GameState {
    Menu,
    Playing1,
}

#[macroquad::main("Whac-A-Mole")]
async fn main() {
    let background_image = load_image("images/background1.png")
        .await
        .expect("Failed to load background1 image");
    let background_texture = Arc::new(Texture2D::from_image(&background_image));
    
    let mole_image = load_image("images/mole.png")
        .await
        .expect("Failed to load mole image");
    let mole_texture = Arc::new(Texture2D::from_image(&mole_image));
    
    let _hemlet_mole_texture = load_image("images/helmet_mole.png")
        .await
        .expect("Failed to load helmet_mole image");
    let _cat_texture = load_image("images/cat.png")
        .await
        .expect("Failed to load cat image");
    let mut game_state = GameState::Menu;
    let mut current_game: Option<Game> = None;

    loop {
        clear_background(LIGHTGRAY);

        match game_state {
            GameState::Menu => {
                if let Some(difficulty) = draw_menu() {
                    if difficulty == 1 {
                        current_game = Some(Game::new(difficulty, background_texture.clone(), mole_texture.clone()));
                        game_state = GameState::Playing1;
                    }
                }
            }

            GameState::Playing1 => {
                if let Some(ref mut game) = current_game {
                    let return_to_menu = game.update();
                    game.draw();

                    if return_to_menu {
                        game_state = GameState::Menu;
                        current_game = None;
                    }
                }
            }
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
