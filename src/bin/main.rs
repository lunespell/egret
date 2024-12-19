use macroquad::miniquad::TextureKind::Texture2D;
use macroquad::prelude::*;
use egret::egret::asset_manager::AssetManager;

#[macroquad::main("MyGame")]
async fn main() {
    loop {
        clear_background(RED);
        let mut asset_manager = AssetManager::default();
        asset_manager.load("bg_fish", "bg_fish.png");
        let bg_fish_img = asset_manager.fetch("bg_fish");
        let bg_fish_tex = macroquad::texture::Texture2D::from_image(bg_fish_img);
        draw_texture(&bg_fish_tex, 0.0, 0.0, WHITE);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}