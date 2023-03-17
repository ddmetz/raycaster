use game::PlayerCamera;
use macroquad::prelude::*;
use macroquad::ui::widgets::Button;
pub mod game;
use crate::game::Textures;
use crate::game::GameState;

const SCREEN_W: f32 = 1280.;
const SCREEN_H: f32 = 720.;
const SCREEN_SCALE: f32 = 1.0;
const SCALED_SCREEN_W: f32 = 1280. * SCREEN_SCALE;
const SCALED_SCREEN_H: f32 = 720. * SCREEN_SCALE;
const BACKGROUND_COLOR: Color = color_u8!(110, 93, 143, 255);

fn window_conf() -> Conf {
    Conf {
        window_title: "Rust Raycaster".to_owned(),
        fullscreen: false,
        window_width: SCREEN_W as i32,
        window_height: SCREEN_H as i32,
        sample_count: 0,
        window_resizable: false,
        high_dpi: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    let textures = Textures::new();
    let mut state = GameState::new(SCREEN_SCALE, SCALED_SCREEN_W, SCALED_SCREEN_H);

    let rect = Rect::new(0.0, 0.0, SCALED_SCREEN_W, SCALED_SCREEN_H);
    let screen_camera = Camera2D::from_display_rect(rect);
    //let player_camera = PlayerCamera::new(SCREEN_SCALE, SCALED_SCREEN_W, SCALED_SCREEN_H, FOV);

    loop {
        set_camera(&screen_camera);
        clear_background(BACKGROUND_COLOR);

        state.update(&textures);

        macroquad_profiler::profiler(Default::default());

        next_frame().await;
    }
}
