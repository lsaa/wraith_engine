//
// altas.rs - TextureAtlas builder
//

use rectangle_pack::RectToInsert;
use rectangle_pack::GroupedRectsToPlace;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::image::{LoadTexture};

pub struct TextureAtlas {
	pub img: Texture,
}

impl TextureAtlas {

}

pub struct TextureAtlasBuilder<'a> {
	pub rects: GroupedRectsToPlace<&'a str>,
}

impl TextureAtlasBuilder<'_> {
	pub fn from_files(files: Vec<&str>, canvas: &mut Canvas<Window>) -> Self {
		let mut rects = GroupedRectsToPlace::new();
		let texture_creator = canvas.texture_creator();
		for tex in files.iter() {
			let t = texture_creator.load_texture(*tex).unwrap();
			let (w, h) = (t.query().width, t.query().height);
			rects.push_rect(tex, None, RectToInsert::new(w, h, 1));
		}

		Self {
			rects
		}
	}
}