use crate::game::GameState;
use game::PlayerCamera;
use macroquad::prelude::*;
use macroquad::ui::widgets::Button;

mod game;
mod textures;

const SCREEN_W: f32 = 1280.;
const SCREEN_H: f32 = 720.;
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
    let mut screen_scale = 1.0;
    let mut scaled_w = SCREEN_W * screen_scale;
    let mut scaled_h = SCREEN_H *  screen_scale;

    let mut state = GameState::new(screen_scale, scaled_w, scaled_h);

    let mut rect = Rect::new(0.0, 0.0, scaled_w, scaled_h);
    let mut screen_camera = Camera2D::from_display_rect(rect);

    loop {
        set_camera(&screen_camera);
        clear_background(BACKGROUND_COLOR);

        state.update();

        macroquad_profiler::profiler(Default::default());

        if Button::new("TOGGLE PIXELATION")
        .position(vec2(0., SCREEN_H-20.0))
        .ui(&mut *macroquad::ui::root_ui())
        {
            if rect.w == SCREEN_W {
                screen_scale = 0.2;
                scaled_w = SCREEN_W * screen_scale;
                scaled_h = SCREEN_H *  screen_scale;
                state.camera = PlayerCamera::new(screen_scale, scaled_w, scaled_h);
                rect = Rect::new(0.0, 0.0, scaled_w, scaled_h);
                screen_camera = Camera2D::from_display_rect(rect);
            } else {
                screen_scale = 1.0;
                scaled_w = SCREEN_W * screen_scale;
                scaled_h = SCREEN_H * screen_scale;
                state.camera = PlayerCamera::new(screen_scale, scaled_w, scaled_h);
                rect = Rect::new(0.0, 0.0, scaled_w, scaled_h);
                screen_camera = Camera2D::from_display_rect(rect);
            }
        }

        next_frame().await;
    };
}
