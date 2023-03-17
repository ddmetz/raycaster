use macroquad::prelude::*;
// Magic numbers
const MOVE_SPEED: f32 = 4.0;
const ROTATE_SPEED: f32 = 3.5;
const SCREEN_W: f32 = 1280.;
const SCREEN_H: f32 = 720.;
const SCREEN_SCALE: f32 = 1.0;
const SCALED_SCREEN_W: f32 = 1280. * SCREEN_SCALE;
const SCALED_SCREEN_H: f32 = 720. * SCREEN_SCALE;
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

pub struct Textures {
    brick: Texture2D,
    wood: Texture2D,
    colorstone: Texture2D,
    roundstone: Texture2D,
    greystone: Texture2D,
}
impl Textures {
    pub fn new() -> Textures {
        let brick = Texture2D::from_file_with_format(include_bytes!("../resources/brick.png"), None);
        let wood = Texture2D::from_file_with_format(include_bytes!("../resources/wood.png"), None);
        let colorstone =
            Texture2D::from_file_with_format(include_bytes!("../resources/colorstone.png"), None);
        let roundstone =
            Texture2D::from_file_with_format(include_bytes!("../resources/roundstone.png"), None);
        let greystone =
            Texture2D::from_file_with_format(include_bytes!("../resources/greystone.png"), None);
        build_textures_atlas();
        Textures {
            brick,
            wood,
            colorstone,
            roundstone,
            greystone,
        }
    }
}

pub struct PlayerCamera {
    scale: f32,
    w: f32,
    h: f32,
    fov: f32,
}
impl PlayerCamera {
    pub fn new(scale: f32, w: f32, h: f32) -> PlayerCamera {
        PlayerCamera {
            scale,
            w,
            h,
            fov: FOV,
        }
    }
    pub fn update(new_scale: f32, new_w: f32, new_h: f32) {

    }
}

pub struct GameState {
    pos_x: f32, // player x position
    pos_y: f32, // player y position
    angle: f32, // player angle in radians
    pub camera: PlayerCamera,
}

impl GameState {
    pub fn new(scale: f32, camera_w: f32, camera_h: f32) -> GameState {
        GameState {
            pos_x: 13.0,
            pos_y: 2.1,
            angle: 2.0,
            camera: PlayerCamera {
                scale,
                w: camera_w,
                h: camera_h,
                fov: FOV
            }
        }
    }

    /// Updates player position then renders new frame
    pub fn update(&mut self, textures: &Textures) {
        self.check_inputs();
        self.draw_view(textures);
    }

    fn check_inputs(&mut self) {
        // use frame time to calculate consistent movement speed even if fps is inconsistent
        let ft = get_frame_time();

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
        let new_x = self.pos_x + move_vector.x * MOVE_SPEED * ft;
        let new_y = self.pos_y + move_vector.y * MOVE_SPEED * ft;
        if MAP[new_y as usize][new_x as usize] < 4 {
            self.pos_x = new_x;
            self.pos_y = new_y;
        }

        // Rotation
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            self.angle -= 1.0 * ROTATE_SPEED * ft;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            self.angle += 1.0 * ROTATE_SPEED * ft;
        }

        // Mouse camera rotation, currently set_cursor_grab is bugged so i'm just using left/right to turn
        // if mouse_position().0 > self.prev_mouse_pos.0 {
        //     let diff = (mouse_position().0 - self.prev_mouse_pos.0)  * MOUSE_SENS * ft;
        //     self.angle -= diff;
        // } else if mouse_position().0 < self.prev_mouse_pos.0 {
        //     let diff = (self.prev_mouse_pos.0 - mouse_position().0) * MOUSE_SENS * ft;
        //     self.angle += diff;
        // }
        // self.prev_mouse_pos = mouse_position();
    }

    fn draw_view(&self, textures: &Textures) {
        let ray_step = self.camera.fov / self.camera.w;
        let mut ray_angle = self.angle - (self.camera.fov * 0.5);

        for x in 0..self.camera.w as i32 + 1 {
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
            let height = self.camera.w / distance;
            let start_y = (self.camera.h / 2.0) as f32 - (height / 2.) as f32;

            let texture = if texture == 4 {
                textures.brick
            } else if texture == 5 {
                textures.wood
            } else if texture == 6 {
                textures.colorstone
            } else {
                textures.roundstone
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

            self.draw_floor_slice(x, start_y + height, ray_angle, textures);

            ray_angle += ray_step;
        }
    }

    fn draw_floor_slice(&self, x: i32, height: f32, ray_angle: f32, textures: &Textures) {
        let dir_cos = ray_angle.cos();
        let dir_sin = ray_angle.sin();
        let step_y = (3. * self.camera.scale) as usize;
        let dest_length = 6. * self.camera.scale;
        
        for y in (height as i32..self.camera.h as i32).step_by(step_y) {
            let mut distance = self.camera.w / (2.0 * y as f32 - self.camera.h);
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
                textures.roundstone
            } else if texture == 1 {
                textures.greystone
            } else if texture == 2 {
                textures.colorstone
            } else {
                textures.brick
            };

            let texture_x = tilex.fract() * texture.width() as f32;
            let texture_y = tiley.fract() * texture.height() as f32;

            let p = DrawTextureParams {
                dest_size: Some(Vec2::new(dest_length, dest_length)),
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