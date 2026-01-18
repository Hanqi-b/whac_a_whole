use macroquad::prelude::*;

pub fn draw_button(x: f32, y: f32, width: f32, height: f32, text: &str, color: Color) {
    draw_rectangle(x, y, width, height, color);
    draw_rectangle_lines(x, y, width, height, 3.0, BLACK);
    let text_size = measure_text(text, None, 40, 1.0);
    draw_text(
        text,
        x + (width - text_size.width) / 2.0,
        y + (height + text_size.height) / 2.0,
        40.0,
        WHITE,
    );
}

pub fn is_button_clicked(x: f32, y: f32, width: f32, height: f32, mouse_x: f32, mouse_y: f32) -> bool {
    mouse_x >= x && mouse_x <= x + width && mouse_y >= y && mouse_y <= y + height
}

pub fn draw_menu() -> Option<u8> {
    // Draw title
    draw_text(
        "WHAC-A-MOLE",
        screen_width() / 2.0 - 150.0,
        100.0,
        60.0,
        BLACK,
    );

    draw_text(
        "Select Difficulty:",
        screen_width() / 2.0 - 120.0,
        200.0,
        40.0,
        DARKGRAY,
    );

    // Draw difficulty buttons
    let button_width = 200.0;
    let button_height = 60.0;
    let button_x = screen_width() / 2.0 - button_width / 2.0;

    draw_button(button_x, 250.0, button_width, button_height, "Easy (1)", GREEN);
    draw_button(button_x, 330.0, button_width, button_height, "Medium (2)", ORANGE);
    draw_button(button_x, 410.0, button_width, button_height, "Hard (3)", RED);

    draw_text(
        "Click a button to start!",
        screen_width() / 2.0 - 130.0,
        520.0,
        25.0,
        DARKGRAY,
    );

    draw_text(
        "Press Q to return to menu, ESC to quit.",
        20.0,
        screen_height() - 20.0,
        20.0,
        DARKGRAY,
    );

    // Handle button clicks
    if is_mouse_button_pressed(MouseButton::Left) {
        let (mouse_x, mouse_y) = mouse_position();

        if is_button_clicked(button_x, 250.0, button_width, button_height, mouse_x, mouse_y) {
            return Some(1); // Easy
        } else if is_button_clicked(button_x, 330.0, button_width, button_height, mouse_x, mouse_y) {
            return Some(2); // Medium
        } else if is_button_clicked(button_x, 410.0, button_width, button_height, mouse_x, mouse_y) {
            return Some(3); // Hard
        }
    }

    None
}
