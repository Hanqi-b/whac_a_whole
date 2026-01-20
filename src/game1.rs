use macroquad::prelude::*;
use std::sync::Arc;

const MOLE_WIDTH: f32 = 256.0;
const MOLE_HEIGHT: f32 = 256.0;

pub struct Game {
    moles: Vec<Mole>,
    score: i32,
    message: String,
    message_timer: f64,
    background_texture: Arc<Texture2D>,
    mole_texture: Arc<Texture2D>,
    difficulty: u8,
    start_time: f64,
    game_duration: f64,
    game_over: bool,
}

pub struct Mole {
    x: f32,
    y: f32,
    visible: bool,
    last_toggle: f64,
}

impl Mole {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            visible: false,
            last_toggle: get_time(),
        }
    }

    pub fn update(&mut self) {
        let elapsed = get_time() - self.last_toggle;
        let toggle_interval = rand::gen_range(0.8, 4.0);
        
        if elapsed > toggle_interval {
            self.visible = !self.visible;
            self.last_toggle = get_time();
        }
    }

    pub fn draw(&self, mole_texture: &Arc<Texture2D>) {
        if self.visible {
            // Draw mole image, centered
            draw_texture(
                mole_texture,
                self.x - MOLE_WIDTH / 2.0,
                self.y - MOLE_HEIGHT / 2.0,
                WHITE,
            );
        }
    }

    pub fn is_clicked(&self, mouse_x: f32, mouse_y: f32) -> bool {
        if !self.visible {
            return false;
        }
        // Rectangle collision detection
        let left = self.x - MOLE_WIDTH / 2.0;
        let right = self.x + MOLE_WIDTH / 2.0;
        let top = self.y - MOLE_HEIGHT / 2.0;
        let bottom = self.y + MOLE_HEIGHT / 2.0;
        
        mouse_x >= left && mouse_x <= right && mouse_y >= top && mouse_y <= bottom
    }

    pub fn hide(&mut self) {
        self.visible = false;
        self.last_toggle = get_time();
    }
}

impl Game {
    pub fn new(
        difficulty: u8,
        background_texture: Arc<Texture2D>,
        mole_texture: Arc<Texture2D>,
    ) -> Self {
        // Positions of four moles
        let positions = vec![
            (450.0, 200.0),  // Top left
            (850.0, 200.0),  // Top right
            (450.0, 500.0),  // Bottom left
            (850.0, 500.0),  // Bottom right
        ];

        let moles = positions
            .iter()
            .map(|&(x, y)| Mole::new(x, y))
            .collect();

        Self {
            moles,
            score: 0,
            message: format!("Difficulty: {} - Click the moles!", difficulty),
            message_timer: 0.0,
            background_texture,
            mole_texture,
            difficulty,
            start_time: get_time(),
            game_duration: 60.0,
            game_over: false,
        }
    }

    pub fn update(&mut self) -> bool {
        // Update all moles
        for mole in &mut self.moles {
            if self.game_over {
                mole.visible = false;
            } else {
                mole.update();
            }
        }

        // Handle mouse clicks
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let mut hit = false;

            for mole in &mut self.moles {
                if mole.is_clicked(mouse_x, mouse_y) {
                    self.score += 1;
                    mole.hide();
                    self.message = format!("Hit! Score: {}", self.score);
                    self.message_timer = get_time();
                    hit = true;
                    break;
                }
            }

            if !hit {
                self.message = "Missed!".to_string();
                self.message_timer = get_time();
            }
        }
        
        // Check for game over
        if get_time() - self.start_time >= self.game_duration { 
            self.game_over = true;
        }

        // Check for return to menu
        is_key_pressed(KeyCode::Q)
    }

    pub fn draw(&self) {
        // Draw background
        draw_texture(&self.background_texture, 0.0, 0.0, WHITE);

        // Draw all moles
        for mole in &self.moles {
            mole.draw(&self.mole_texture);
        }

        // Draw UI
        draw_text(
            &format!("Score: {}", self.score),
            20.0,
            40.0,
            40.0,
            WHITE,
        );
        if !self.game_over {
            draw_text(
                &format!("Time: {:.0}", self.game_duration - (get_time() - self.start_time)),
                20.0,
                80.0,
                40.0,
                WHITE,
            );
        } else {
            draw_text(
                "Time: 0",
                20.0,
                80.0,
                40.0,
                WHITE,
            );
        }
        // Draw message (disappears after 0.25 seconds)
        if get_time() - self.message_timer < 0.25 {
            draw_text(&self.message, 20.0, screen_height() - 80.0, 30.0, DARKGREEN);
        }
        if self.game_over {
            draw_text(&format!("Game Over! Final Score: {}", self.score), 20.0, screen_height() - 80.0, 30.0, RED);
        }

        draw_text(
            "Press Q to return to menu, ESC to quit.",
            20.0,
            screen_height() - 20.0,
            20.0,
            DARKGRAY,
        );
    }
}
