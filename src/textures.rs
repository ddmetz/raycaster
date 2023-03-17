use macroquad::prelude::*;

pub struct Textures {
    pub brick: Texture2D,
    pub wood: Texture2D,
    pub colorstone: Texture2D,
    pub roundstone: Texture2D,
    pub greystone: Texture2D,
}
impl Textures {
    pub fn new() -> Textures {
        let brick =
            Texture2D::from_file_with_format(include_bytes!("../resources/brick.png"), None);
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
