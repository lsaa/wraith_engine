use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::render::Texture;
use sdl2::image::{LoadTexture};
use std::collections::HashMap;

pub struct TextureManager {
	cache: HashMap<String, Texture>
}

impl TextureManager {
	pub fn new() -> Self {
		Self {
			cache: HashMap::new()
		}
	}

	pub fn create(&mut self, path: &str, canvas: &mut Canvas<Window>) -> Result<(), &str> {
		let creator = canvas.texture_creator();
		let t =  creator.load_texture(path);
		if !t.is_ok() {
			return Err("Couldn't load texture");
		}
		let t = t.unwrap();
		self.cache.insert(String::from(path), t);
		return Ok(());
	}

	pub fn load(&mut self, path: &str) -> Result<&Texture, &str> {
		if self.cache.contains_key(&String::from(path)) {
            return Ok(self.cache.get(&String::from(path)).unwrap());
		}

		Err("Texture isn't loaded")
	}
}