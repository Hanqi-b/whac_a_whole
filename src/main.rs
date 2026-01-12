use macroquad::prelude::*;

struct Mole {
    x: f32,
    y: f32,
    size: f32,
    visible: bool,
    last_toggle: f64,
}

impl Mole {
    fn new(x: f32, y: f32, size: f32) -> Self {
        Self {
            x,
            y,
            size,
            visible: false,
            last_toggle: get_time(),
        }
    }

    fn update(&mut self) {
        let elapsed = get_time() - self.last_toggle;
        let toggle_interval = rand::gen_range(1.0, 3.0);
        
        if elapsed > toggle_interval {
            self.visible = !self.visible;
            self.last_toggle = get_time();
        }
    }

    fn draw(&self) {
        // Draw hole
        draw_circle(self.x, self.y, self.size, BROWN);
        draw_circle_lines(self.x, self.y, self.size, 3.0, BLACK);
        
        // Draw mole if visible
        if self.visible {
            draw_circle(self.x, self.y - 10.0, self.size * 0.8, DARKBROWN);
            draw_text("🐭", self.x - 15.0, self.y + 10.0, 30.0, WHITE);
        }
    }

    fn is_clicked(&self, mouse_x: f32, mouse_y: f32) -> bool {
        if !self.visible {
            return false;
        }
        let dx = mouse_x - self.x;
        let dy = mouse_y - self.y;
        (dx * dx + dy * dy).sqrt() < self.size
    }
}

#[macroquad::main("Whac-A-Mole")]
async fn main() {
    let mut score = 0;
    let mut mole = Mole::new(400.0, 300.0, 50.0);
    let mut message = String::from("Click the mole when it appears!");
    let mut message_timer = 0.0;

    loop {
        clear_background(LIGHTGRAY);

        // Update mole
        mole.update();

        // Handle mouse click
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            if mole.is_clicked(mouse_x, mouse_y) {
                score += 1;
                mole.visible = false;
                mole.last_toggle = get_time();
                message = format!("Hit! Score: {}", score);
                message_timer = get_time();
            } else {
                message = "Missed!".to_string();
                message_timer = get_time();
            }
        }

        // Draw mole
        mole.draw();

        // Draw UI
        draw_text(
            &format!("Score: {}", score),
            20.0,
            40.0,
            40.0,
            BLACK,
        );

        // Draw message (fade after 1 second)
        if get_time() - message_timer < 1.0 {
            draw_text(&message, 20.0, 80.0, 30.0, DARKGREEN);
        }

        draw_text(
            "Press ESC or Q to quit",
            20.0,
            screen_height() - 20.0,
            20.0,
            DARKGRAY,
        );

        // Check for quit
        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
            break;
        }

        next_frame().await;
    }
}
