use macroquad::prelude::*;

// Magic numbers
const MOVE_SPEED: f32 = 4.0;
const ROTATE_SPEED: f32 = 3.5;
const FOV: f32 = 1.0; // in radians

// Colors
const COLOR_1: Color = color_u8!(110, 93, 143, 255);
const COLOR_2: Color = color_u8!(198, 183, 190, 255);
//const COLOR_2: Color = color_u8!(148, 133, 140, 255);
//const COLOR_3: Color = color_u8!(137, 141 ,168, 255);
const COLOR_4: Color = color_u8!(148, 131, 139, 255);

const MAP: [[u8; 16]; 16] = [
    [4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4],
    [4, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 4],
    [4, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 4],
    [4, 0, 0, 4, 4, 0, 4, 4, 0, 4, 0, 0, 0, 0, 0, 4],
    [4, 0, 0, 4, 0, 0, 0, 4, 0, 4, 0, 0, 0, 0, 0, 4],
    [4, 0, 0, 4, 0, 4, 0, 4, 0, 0, 0, 0, 0, 0, 0, 4],
    [4, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 4],
    [4, 0, 0, 0, 0, 4, 4, 4, 1, 1, 4, 4, 4, 4, 1, 4],
    [4, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 4],
    [4, 4, 4, 4, 4, 4, 4, 4, 4, 2, 1, 1, 1, 1, 1, 4],
    [4, 4, 4, 4, 4, 5, 2, 2, 2, 2, 2, 5, 5, 5, 2, 5],
    [4, 4, 4, 4, 4, 5, 2, 2, 2, 2, 2, 2, 2, 5, 2, 5],
    [4, 4, 4, 4, 4, 5, 2, 2, 2, 2, 2, 2, 2, 5, 2, 5],
    [4, 4, 4, 4, 4, 5, 2, 2, 2, 5, 5, 5, 5, 5, 2, 5],
    [4, 4, 4, 4, 4, 5, 2, 2, 2, 2, 2, 2, 2, 2, 2, 5],
    [4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5],
];

const SCREEN_SCALE: f32 = 3.;
const SCREEN_W: f32 = 1280. / SCREEN_SCALE;
const SCREEN_H: f32 = 720. / SCREEN_SCALE;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rust Raycaster".to_owned(),
        fullscreen: false,
        window_width: (SCREEN_SCALE * SCREEN_W) as i32,
        window_height: (SCREEN_SCALE * SCREEN_H) as i32,
        sample_count: 0,
        window_resizable: false,
        high_dpi: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let brick = Texture2D::from_file_with_format(include_bytes!("../resources/brick.png"), None);
    let wood = Texture2D::from_file_with_format(include_bytes!("../resources/wood.png"), None);
    let colorstone =
        Texture2D::from_file_with_format(include_bytes!("../resources/colorstone.png"), None);
    let roundstone =
        Texture2D::from_file_with_format(include_bytes!("../resources/roundstone.png"), None);
    let greystone =
        Texture2D::from_file_with_format(include_bytes!("../resources/greystone.png"), None);
    build_textures_atlas();

    let mut state: GameState = GameState {
        pos_x: 13.0,
        pos_y: 2.1,
        angle: 2.0,
        wall_height: screen_width() / FOV, // 1.5,
        brick,
        wood,
        colorstone,
        roundstone,
        greystone,
        dt: get_frame_time(),
    };

    let rect = Rect::new(0.0, 0.0, SCREEN_W, SCREEN_H);
    let camera = Camera2D::from_display_rect(rect);

    loop {
        clear_background(COLOR_1);
        state.dt = get_frame_time();
        state.wall_height = screen_width() / (FOV);

        set_camera(&camera);
        state.update();

        macroquad_profiler::profiler(Default::default());

        next_frame().await;
    }
}

struct GameState {
    pos_x: f32, // player x position
    pos_y: f32, // player y position
    angle: f32, // radians
    wall_height: f32,
    brick: Texture2D,
    wood: Texture2D,
    colorstone: Texture2D,
    roundstone: Texture2D,
    greystone: Texture2D,
    dt: f32, // change in time since the last frame
}

impl GameState {
    /// Updates player position then renders new frame
    pub fn update(&mut self) {
        self.check_inputs();
        self.raycasting();
    }

    fn check_inputs(&mut self) {
        let mut move_vector = Vec2::new(0.0, 0.0);

        let player_cos = (self.angle).cos();
        let player_sin = (self.angle).sin();

        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            move_vector.x += player_cos;
            move_vector.y += player_sin;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            move_vector.x -= player_cos;
            move_vector.y -= player_sin;
        }

        // Strafe left/right
        // if is_key_down(KeyCode::A) {
        //     move_vector.x += (self.angle+FRAC_PI_2).cos();
        //     move_vector.y -= (self.angle+FRAC_PI_2).sin();
        // }
        // if is_key_down(KeyCode::D) {
        //     move_vector.x -= (self.angle+FRAC_PI_2).cos();
        //     move_vector.y += (self.angle+FRAC_PI_2).sin();
        // }

        move_vector = move_vector.normalize();
        let new_x = self.pos_x + move_vector.x * MOVE_SPEED * self.dt as f32;
        let new_y = self.pos_y + move_vector.y * MOVE_SPEED * self.dt as f32;
        if MAP[new_y as usize][new_x as usize] < 4 {
            self.pos_x = new_x;
            self.pos_y = new_y;
        }

        // Rotation
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            self.angle -= 1.0 * ROTATE_SPEED * self.dt as f32;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            self.angle += 1.0 * ROTATE_SPEED * self.dt as f32;
        }

        // Mouse camera rotation, currently set_cursor_grab is bugged so i'm just using left/right to turn
        //
        // if mouse_position().0 > self.prev_mouse_pos.0 {
        //     let diff = (mouse_position().0 - self.prev_mouse_pos.0)  * MOUSE_SENS * self.dt as f32;
        //     self.angle -= diff;
        // } else if mouse_position().0 < self.prev_mouse_pos.0 {
        //     let diff = (self.prev_mouse_pos.0 - mouse_position().0) * MOUSE_SENS * self.dt as f32;
        //     self.angle += diff;
        // }
        // self.prev_mouse_pos = mouse_position();
    }

    fn raycasting(&self) {
        let ray_step = FOV / SCREEN_W;
        let mut ray_angle = self.angle - (FOV * 0.5);

        for x in 0..SCREEN_W as i32 + 1 {
            //self.draw_background_slice(x, ray_angle);

            let mut ray_x = self.pos_x;
            let mut ray_y = self.pos_y;
            let mut dist_x = 0.0;
            let mut dist_y = 0.0;

            let ray_cos = (ray_angle).cos() / 300.;
            let ray_sin = (ray_angle).sin() / 300.;

            let mut texture = 0;
            while texture < 4 {
                ray_x += ray_cos;
                ray_y += ray_sin;
                dist_x = self.pos_x - ray_x;
                dist_y = self.pos_y - ray_y;
                texture = MAP[ray_y as usize][ray_x as usize];
            }

            let distance = dist_x.hypot(dist_y) * (ray_angle - self.angle).cos();
            let height = SCREEN_W / distance;
            let start_y = (SCREEN_H / 2.0) as f32 - (height / 2.) as f32;

            let texture = if texture == 4 {
                self.brick
            } else if texture == 5 {
                self.wood
            } else if texture == 6 {
                self.colorstone
            } else {
                self.roundstone
            };

            let params = DrawTextureParams {
                dest_size: Some(Vec2::new(4., height as f32)),
                source: Some(Rect::new(
                    (ray_x + ray_y).fract() * texture.width(),
                    0.0,
                    0.1,
                    texture.height(),
                )),
                ..Default::default()
            };

            draw_texture_ex(texture, x as f32, start_y as f32, COLOR_2, params);

            self.draw_floor_slice(x, start_y + height, ray_angle);

            ray_angle += ray_step;
        }
    }

    fn draw_floor_slice(&self, x: i32, height: f32, ray_angle: f32) {
        let dir_cos = ray_angle.cos();
        let dir_sin = ray_angle.sin();
        for y in height as i32..SCREEN_H as i32 {
            let mut distance = SCREEN_W / (2.0 * y as f32 - SCREEN_H);
            distance = distance / (self.angle - ray_angle).cos();

            // Get the tile position
            let tilex = (distance * dir_cos) + self.pos_x;
            let tiley = (distance * dir_sin) + self.pos_y;

            let texture = if tilex > 15. || tiley > 15. {
                0
            } else {
                MAP[tiley as usize][tilex as usize]
            };

            let texture = if texture == 0 {
                self.roundstone
            } else if texture == 1 {
                self.greystone
            } else if texture == 2 {
                self.colorstone
            } else {
                self.brick
            };

            let texture_x = tilex.fract() * texture.width() as f32;
            let texture_y = tiley.fract() * texture.height() as f32;

            let p = DrawTextureParams {
                dest_size: Some(Vec2::new(4., 4.)),
                source: Some(Rect::new(texture_x, texture_y, 1., 1.)),
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None,
            };
            draw_texture_ex(texture, x as f32, y as f32, COLOR_4, p);
        }
    }
}
