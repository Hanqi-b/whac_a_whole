use macroquad::prelude::*;
use std::sync::Arc;

const MOLE_WIDTH: f32 = 100.0;
const MOLE_HEIGHT: f32 = 100.0;

pub struct Game {
    moles: Vec<Mole>,
    score: i32,
    message: String,
    message_timer: f64,
    background_texture: Arc<Texture2D>,
    mole_texture: Arc<Texture2D>,
    difficulty: u8,
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
        let toggle_interval = rand::gen_range(1.0, 3.0);
        
        if elapsed > toggle_interval {
            self.visible = !self.visible;
            self.last_toggle = get_time();
        }
    }

    pub fn draw(&self, mole_texture: &Arc<Texture2D>) {
        if self.visible {
            // 绘制地鼠图片，中心对齐
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
        // 矩形碰撞检测
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
        // 四个地鼠的位置
        let positions = vec![
            (250.0, 200.0),  // 左上
            (550.0, 200.0),  // 右上
            (250.0, 400.0),  // 左下
            (550.0, 400.0),  // 右下
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
        }
    }

    pub fn update(&mut self) -> bool {
        // 更新所有地鼠
        for mole in &mut self.moles {
            mole.update();
        }

        // 处理鼠标点击
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

        // 检查返回菜单
        is_key_pressed(KeyCode::Q)
    }

    pub fn draw(&self) {
        // 绘制背景
        draw_texture(&self.background_texture, 0.0, 0.0, WHITE);

        // 绘制所有地鼠
        for mole in &self.moles {
            mole.draw(&self.mole_texture);
        }

        // 绘制UI
        draw_text(
            &format!("Score: {}", self.score),
            20.0,
            40.0,
            40.0,
            BLACK,
        );

        // 绘制消息（1秒后消失）
        if get_time() - self.message_timer < 1.0 {
            draw_text(&self.message, 20.0, 80.0, 30.0, DARKGREEN);
        }

        // 绘制难度和提示
        draw_text(
            &format!("Difficulty: {}", self.difficulty),
            20.0,
            120.0,
            30.0,
            DARKGRAY,
        );

        draw_text(
            "Press Q to return to menu, ESC to quit.",
            20.0,
            screen_height() - 20.0,
            20.0,
            DARKGRAY,
        );
    }
}
