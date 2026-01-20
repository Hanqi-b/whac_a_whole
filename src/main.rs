use macroquad::prelude::*;
use std::sync::Arc;

mod menu;
mod game1;
mod game2;

use menu::draw_menu;
use game1::Game as Game1;
use game2::Game as Game2;

#[derive(PartialEq)]
enum GameState {
    Menu,
    Playing1,
    Playing2,
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
    
    let hemlet_mole_image = load_image("images/helmet_mole.png")
        .await
        .expect("Failed to load helmet_mole image");
    let hemlet_mole_texture = Arc::new(Texture2D::from_image(&hemlet_mole_image));

    let cat_image = load_image("images/cat.png")
        .await
        .expect("Failed to load cat image");
    let cat_texture = Arc::new(Texture2D::from_image(&cat_image));
    
    let mut game_state = GameState::Menu;
    let mut current_game1: Option<Game1> = None;
    let mut current_game2: Option<Game2> = None;

    loop {
        clear_background(LIGHTGRAY);

        match game_state {
            GameState::Menu => {
                if let Some(difficulty) = draw_menu() {
                    if difficulty == 1 {
                        current_game1 = Some(Game1::new(difficulty, background_texture.clone(), mole_texture.clone()));
                        game_state = GameState::Playing1;
                    } else if difficulty == 2 {
                        current_game2 = Some(Game2::new(difficulty, background_texture.clone(), mole_texture.clone(), hemlet_mole_texture.clone(), cat_texture.clone()));
                        game_state = GameState::Playing2;
                    }
                }
            }

            GameState::Playing1 => {
                if let Some(ref mut game) = current_game1 {
                    let return_to_menu = game.update();
                    game.draw();

                    if return_to_menu {
                        game_state = GameState::Menu;
                        current_game1 = None;
                    }
                }
            }

            GameState::Playing2 => {
                if let Some(ref mut game) = current_game2 {
                    let return_to_menu = game.update();
                    game.draw();

                    if return_to_menu {
                        game_state = GameState::Menu;
                        current_game2 = None;
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
