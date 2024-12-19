use std::collections::HashMap;
use macroquad::texture::Image;
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "assets/"]
pub struct Asset;

#[derive(Default)]
pub struct AssetManager {
    assets: HashMap<String, Image>
}

impl AssetManager {
    pub fn load(&mut self, asset_name: &str, asset_path: &str) {
        if let Some(asset_data) = Asset::get(asset_path) {
            if let Ok(asset_image) = Image::from_file_with_format(&asset_data.data, None) {
                self.assets.insert(String::from(asset_name), asset_image);
                return;
            }
        }
        
        eprintln!("[AssetManager::load] Asset ${asset_name} not found. Loading fallback image.");
        self.assets.insert(String::from(asset_name), Self::get_fallback_image());
    }
    
    pub fn fetch(&self, asset_name: &str) -> &Image {
        if let Some(image) = self.assets.get(asset_name) {
            return image;
        }

        eprintln!("[AssetManager::fetch] Asset ${asset_name} not found.");
        panic!();
    }
    
    fn get_fallback_image() -> Image {
        if let Some(fallback_image_data) = Asset::get("_fallback.png") {
            if let Ok(fallback_image) = Image::from_file_with_format(&fallback_image_data.data, None) {
                return fallback_image;
            }
        }
        Self::generate_default_fallback_image()
    }
    
    fn generate_default_fallback_image() -> Image {
        // Define the dimensions of the fallback image
        let width = 128;
        let height = 128;

        // Create a buffer to hold pixel data (RGBA format)
        let mut pixels = vec![0u8; width * height * 4];

        // Fill the buffer with red (255, 0, 0, 255)
        for pixel in pixels.chunks_exact_mut(4) {
            pixel[0] = 255; // Red
            pixel[1] = 0;   // Green
            pixel[2] = 0;   // Blue
            pixel[3] = 255; // Alpha
        }

        // Create an Image object from the pixel data
        Image {
            bytes: pixels,
            width: width as u16,
            height: height as u16,
        } 
    } 
}