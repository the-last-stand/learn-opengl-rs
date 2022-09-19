use std::path::Path;

pub struct Texture {
    pub data: Vec<u8>,
    pub width: i32,
    pub height: i32,
}

// use image loader
impl From<&Path> for Texture {
    fn from(path: &Path) -> Self {
        let img = image::open(path).unwrap().to_rgb8();
        let width = img.width() as i32;
        let height = img.height() as i32;
        let data: Vec<u8> = img.into_raw();
        Texture {
            data,
            width,
            height,
        }
    }
}
