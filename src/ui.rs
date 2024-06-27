use macroquad::prelude::*;
use miniquad::window::set_mouse_cursor;

use crate::theme;

#[inline]
pub fn button(text: &str, bounds: Rect, font_size: f32) -> bool {
    // TODO: extract screen_width(), etc.

    // 0.0 - 1.0
    let mouse_pos = Vec2::from(mouse_position());
    let scaled_mouse_pos = Vec2::from(mouse_pos) / Vec2::new(screen_width(), screen_height());
    let mb_pressed = is_mouse_button_pressed(MouseButton::Left);

    let color;
    let clicked = if bounds.contains(mouse_pos) {
        set_mouse_cursor(miniquad::CursorIcon::Pointer);

        color = theme::ui::BUTTON_COLOR_HOVER;

        mb_pressed
    } else {
        set_mouse_cursor(miniquad::CursorIcon::Default);
        color = theme::ui::BUTTON_COLOR_NORMAL;

        false
    };

    {
        draw_rectangle(
            bounds.x,
            bounds.y,
            bounds.w,
            bounds.h,
            color,
        );
    }


    let text_size = measure_text(text, None, font_size as u16, 1.0);
    draw_text_ex(text, bounds.x + text_size.width / 2.0, bounds.y + bounds.h / 2.0, TextParams {
        color: theme::text::COLOR, font_size: font_size as u16, ..Default::default()
    });

    clicked
}
