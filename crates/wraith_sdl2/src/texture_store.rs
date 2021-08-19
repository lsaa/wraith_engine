//
//	Texture Store - texture_store.rs
// 

use sdl2::rect::Rect;
use sdl2::render::Texture;

#[derive(Debug)]
pub struct LoadTextureError {
	pub identifier: String
}

impl LoadTextureError {
	pub fn new(identifier: &str) -> Self {
		Self {
			identifier: identifier.to_string()
		}
	}
}

pub trait TextureStore {
	fn get(&mut self, identifier: &str) -> Result<(&Texture, Rect), LoadTextureError>;
}